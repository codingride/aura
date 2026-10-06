use crate::lexer::Lexer;
use crate::tokens::{Token, TokenType};
use crate::ast::{Program, Statement, LetStatement, ConstStatement, AssignmentStatement, Expression, BlockStatement, FunctionStatement, Parameter, IfStatement};

const LOWEST: i32 = 1;
const COMPARISON: i32 = 2; 
const SUM: i32 = 3;     
const PRODUCT: i32 = 4; 
const CALL: i32 = 5; // <-- Add high call precedence tier for parenthesized arguments!

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser { lexer, cur_token, peek_token, errors: Vec::new() }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn cur_token_is(&self, t: TokenType) -> bool { self.cur_token.token_type == t }
    fn peek_token_is(&self, t: TokenType) -> bool { self.peek_token.token_type == t }
    
    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) { self.next_token(); true } else { self.peek_error(t); false }
    }

    fn peek_error(&mut self, t: TokenType) {
        self.errors.push(format!("Line {}, Col {}: Expected {:?}", self.peek_token.line, self.peek_token.column, t));
    }

    fn token_precedence(&self, token_type: TokenType) -> i32 {
        match token_type {
            TokenType::Lparen => CALL, // <-- Register '(' with CALL priority precedence!
            TokenType::Eq | TokenType::Lt | TokenType::Gt => COMPARISON,
            TokenType::Plus | TokenType::Minus => SUM,
            TokenType::Asterisk | TokenType::Slash => PRODUCT,
            _ => LOWEST,
        }
    }

    fn cur_precedence(&self) -> i32 { self.token_precedence(self.cur_token.token_type) }
    fn peek_precedence(&self) -> i32 { self.token_precedence(self.peek_token.token_type) }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: Vec::new() };
        while !self.cur_token_is(TokenType::Eof) {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.token_type {
            TokenType::Let => self.parse_let_statement().map(Statement::Let),
            TokenType::Const => self.parse_const_statement().map(Statement::Const),
            TokenType::Fn => self.parse_function_statement().map(Statement::Function),
            TokenType::If => self.parse_if_statement().map(Statement::If),
            TokenType::Identifier => {
                if self.peek_token_is(TokenType::Assign) {
                    self.parse_assignment_statement().map(Statement::Assignment)
                } else {
                    // Fall back to general expression statements if it's a raw functional invoke call
                    self.parse_expression_statement()
                }
            }
            _ => None,
        }
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expr = self.parse_expression(LOWEST)?;
        Some(Statement::Expression(expr))
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let token = self.cur_token.clone();
        if !self.expect_peek(TokenType::Identifier) { return None; }
        let name = self.cur_token.literal.clone();
        let mut explicit_type = None;

        if self.peek_token_is(TokenType::Colon) {
            self.next_token();
            if !self.expect_peek(TokenType::Identifier) { return None; }
            explicit_type = Some(self.cur_token.literal.clone());
        }

        if !self.expect_peek(TokenType::Assign) { return None; }
        self.next_token();

        Some(LetStatement { token, name, explicit_type, value: self.parse_expression(LOWEST)? })
    }

    fn parse_const_statement(&mut self) -> Option<ConstStatement> {
        let token = self.cur_token.clone();
        if !self.expect_peek(TokenType::Identifier) { return None; }
        let name = self.cur_token.literal.clone();
        let mut explicit_type = None;

        if self.peek_token_is(TokenType::Colon) {
            self.next_token();
            if !self.expect_peek(TokenType::Identifier) { return None; }
            explicit_type = Some(self.cur_token.literal.clone());
        }

        if !self.expect_peek(TokenType::Assign) { return None; }
        self.next_token();

        Some(ConstStatement { token, name, explicit_type, value: self.parse_expression(LOWEST)? })
    }

    fn parse_assignment_statement(&mut self) -> Option<AssignmentStatement> {
        let name = self.cur_token.literal.clone();
        self.next_token(); 
        let token = self.cur_token.clone();
        self.next_token(); 
        let value = self.parse_expression(LOWEST)?;
        Some(AssignmentStatement { token, name, value })
    }

    fn parse_function_statement(&mut self) -> Option<FunctionStatement> {
        let token = self.cur_token.clone();
        if !self.expect_peek(TokenType::Identifier) { return None; }
        let name = self.cur_token.literal.clone();
        if !self.expect_peek(TokenType::Lparen) { return None; }

        let parameters = self.parse_function_parameters()?;
        let mut return_type = "Void".to_string();
        if self.peek_token_is(TokenType::Arrow) {
            self.next_token(); 
            if !self.expect_peek(TokenType::Identifier) { return None; }
            return_type = self.cur_token.literal.clone();
        }
        if !self.expect_peek(TokenType::Lbrace) { return None; }

        Some(FunctionStatement { token, name, parameters, return_type, body: self.parse_block_statement()? })
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<Parameter>> {
        let mut params = Vec::new();
        if self.peek_token_is(TokenType::Rparen) { self.next_token(); return Some(params); }
        self.next_token(); 

        let first_name = self.cur_token.literal.clone();
        if !self.expect_peek(TokenType::Colon) || !self.expect_peek(TokenType::Identifier) { return None; }
        params.push(Parameter { name: first_name, param_type: self.cur_token.literal.clone() });

        while self.peek_token_is(TokenType::Comma) {
            self.next_token(); self.next_token();
            let p_name = self.cur_token.literal.clone();
            if !self.expect_peek(TokenType::Colon) || !self.expect_peek(TokenType::Identifier) { return None; }
            params.push(Parameter { name: p_name, param_type: self.cur_token.literal.clone() });
        }
        if !self.expect_peek(TokenType::Rparen) { return None; }
        Some(params)
    }

    fn parse_block_statement(&mut self) -> Option<BlockStatement> {
        let token = self.cur_token.clone(); 
        let mut statements = Vec::new();
        self.next_token();

        while !self.cur_token_is(TokenType::Rbrace) && !self.cur_token_is(TokenType::Eof) {
            if let Some(stmt) = self.parse_statement() { statements.push(stmt); }
            self.next_token();
        }
        Some(BlockStatement { token, statements })
    }

    fn parse_if_statement(&mut self) -> Option<IfStatement> {
        let token = self.cur_token.clone(); self.next_token();
        let condition = self.parse_expression(LOWEST)?;

        if !self.expect_peek(TokenType::Lbrace) { return None; }
        let consequence = self.parse_block_statement()?;
        let mut alternative = None;

        if self.peek_token_is(TokenType::Else) {
            self.next_token(); 
            if !self.expect_peek(TokenType::Lbrace) { return None; }
            alternative = Some(self.parse_block_statement()?);
        }
        Some(IfStatement { token, condition, consequence, alternative })
    }

    fn parse_expression(&mut self, precedence: i32) -> Option<Expression> {
        let mut left_expr = match self.cur_token.token_type {
            TokenType::Identifier => Some(Expression::Identifier(self.cur_token.literal.clone())),
            TokenType::IntLit => self.cur_token.literal.parse::<i64>().ok().map(Expression::IntegerLiteral),
            TokenType::FloatLit => self.cur_token.literal.parse::<f64>().ok().map(Expression::FloatLiteral),
            TokenType::StringLit => Some(Expression::StringLiteral(self.cur_token.literal.clone())),
            TokenType::True => Some(Expression::BooleanLiteral(true)),
            TokenType::False => Some(Expression::BooleanLiteral(false)),
            _ => { self.errors.push(format!("Line {}: Prefix match error", self.cur_token.line)); None }
        }?;

        // Updated Pratt processing loop to intercept high-precedence function call arguments!
        while !self.peek_token_is(TokenType::Eof) && precedence < self.peek_precedence() {
            match self.peek_token.token_type {
                TokenType::Plus | TokenType::Minus | TokenType::Asterisk | TokenType::Slash |
                TokenType::Eq | TokenType::Lt | TokenType::Gt => {
                    self.next_token(); left_expr = self.parse_infix_expression(left_expr)?;
                }
                TokenType::Lparen => {
                    self.next_token();
                    left_expr = self.parse_function_call_arguments(left_expr)?;
                }
                _ => return Some(left_expr),
            }
        }
        Some(left_expr)
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let operator = self.cur_token.literal.clone();
        let precedence = self.cur_precedence();
        self.next_token(); 
        Some(Expression::Infix(Box::new(left), operator, Box::new(self.parse_expression(precedence)?)))
    }

    // New Function: Parses the comma-separated arguments inside 'calculate(5, 10)'
    fn parse_function_call_arguments(&mut self, left: Expression) -> Option<Expression> {
        let function_name = match left {
            Expression::Identifier(name) => name,
            _ => { self.errors.push("Expected function identifier name before call".to_string()); return None; }
        };
        let mut args = Vec::new();
        if self.peek_token_is(TokenType::Rparen) {
            self.next_token();
            return Some(Expression::FunctionCall(function_name, args));
        }
        self.next_token(); // Move to first argument expression
        args.push(self.parse_expression(LOWEST)?);
        while self.peek_token_is(TokenType::Comma) {
            self.next_token(); // On ','
            self.next_token(); // Move to next expression argument
            args.push(self.parse_expression(LOWEST)?);
        }
        if !self.expect_peek(TokenType::Rparen) { return None; }
        Some(Expression::FunctionCall(function_name, args))
    }
}
