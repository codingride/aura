use crate::lexer::Lexer;
use crate::tokens::{Token, TokenType};
use crate::ast::{Program, Statement, LetStatement, ConstStatement, Expression, BlockStatement, FunctionStatement, Parameter, IfStatement};

const LOWEST: i32 = 1;
const COMPARISON: i32 = 2;
const SUM: i32 = 3;     // + or -
const PRODUCT: i32 = 4; // * or /

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
        
        Parser {
            lexer,
            cur_token,
            peek_token,
            errors: Vec::new(),
        }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.token_type == t
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.token_type == t
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }

    fn peek_error(&mut self, t: TokenType) {
        let msg = format!(
            "Line {}, Col {}: Expected next token to be {:?}, got {:?} instead",
            self.peek_token.line, self.peek_token.column, t, self.peek_token.token_type
        );
        self.errors.push(msg);
    }

    fn token_precedence(&self, token_type: TokenType) -> i32 {
        match token_type {
            TokenType::Eq | TokenType::Lt | TokenType::Gt => COMPARISON,
            TokenType::Plus | TokenType::Minus => SUM,
            TokenType::Asterisk | TokenType::Slash => PRODUCT,
            _ => LOWEST,
        }
    }

    fn cur_precedence(&self) -> i32 {
        self.token_precedence(self.cur_token.token_type)
    }

    fn peek_precedence(&self) -> i32 {
        self.token_precedence(self.peek_token.token_type)
    }

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
            TokenType::If => self.parse_if_statement().map(Statement::If), // Intercept routing
            _ => None,
        }
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

        let value = self.parse_expression(LOWEST)?;

        Some(LetStatement { token, name, explicit_type, value })
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

        let value = self.parse_expression(LOWEST)?;

        Some(ConstStatement { token, name, explicit_type, value })
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

        let body = self.parse_block_statement()?;

        Some(FunctionStatement { token, name, parameters, return_type, body })
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<Parameter>> {
        let mut params = Vec::new();

        if self.peek_token_is(TokenType::Rparen) {
            self.next_token();
            return Some(params);
        }

        self.next_token(); 

        let first_name = self.cur_token.literal.clone();
        if !self.expect_peek(TokenType::Colon) { return None; }
        if !self.expect_peek(TokenType::Identifier) { return None; }
        let first_type = self.cur_token.literal.clone();
        
        params.push(Parameter { name: first_name, param_type: first_type });

        while self.peek_token_is(TokenType::Comma) {
            self.next_token(); 
            self.next_token(); 
            
            let p_name = self.cur_token.literal.clone();
            if !self.expect_peek(TokenType::Colon) { return None; }
            if !self.expect_peek(TokenType::Identifier) { return None; }
            let p_type = self.cur_token.literal.clone();
            
            params.push(Parameter { name: p_name, param_type: p_type });
        }

        if !self.expect_peek(TokenType::Rparen) { return None; }

        Some(params)
    }

    fn parse_block_statement(&mut self) -> Option<BlockStatement> {
        let token = self.cur_token.clone(); 
        let mut statements = Vec::new();

        self.next_token();

        while !self.cur_token_is(TokenType::Rbrace) && !self.cur_token_is(TokenType::Eof) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }

        Some(BlockStatement { token, statements })
    }

    // This is the function that fixes the missing method error!
    fn parse_if_statement(&mut self) -> Option<IfStatement> {
        let token = self.cur_token.clone(); 
        
        self.next_token(); 
        let condition = self.parse_expression(LOWEST)?;

        if !self.expect_peek(TokenType::Lbrace) { return None; }
        let consequence = self.parse_block_statement()?;
        let mut alternative = None;

        if self.peek_token_is(TokenType::Else) {
            self.next_token(); 
            if !self.expect_peek(TokenType::Lbrace) { return None; }
            alternative = Some(self.parse_block_statement()?);
        }

        Some(IfStatement {
            token,
            condition,
            consequence,
            alternative,
        })
    }

    fn parse_expression(&mut self, precedence: i32) -> Option<Expression> {
        let mut left_expr = match self.cur_token.token_type {
            TokenType::Identifier => Some(Expression::Identifier(self.cur_token.literal.clone())),
            TokenType::IntLit => self.cur_token.literal.parse::<i64>().ok().map(Expression::IntegerLiteral),
            TokenType::FloatLit => self.cur_token.literal.parse::<f64>().ok().map(Expression::FloatLiteral),
            TokenType::StringLit => Some(Expression::StringLiteral(self.cur_token.literal.clone())),
            TokenType::True => Some(Expression::BooleanLiteral(true)),
            TokenType::False => Some(Expression::BooleanLiteral(false)),
            _ => {
                self.errors.push(format!("Line {}: No prefix parser match found for {:?}", self.cur_token.line, self.cur_token.token_type));
                None
            }
        }?;

        while !self.peek_token_is(TokenType::Eof) && precedence < self.peek_precedence() {
            match self.peek_token.token_type {
                TokenType::Plus | TokenType::Minus | TokenType::Asterisk | TokenType::Slash | TokenType::Eq | TokenType::Lt | TokenType::Gt => {
                    self.next_token();
                    left_expr = self.parse_infix_expression(left_expr)?;
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
        
        let right = self.parse_expression(precedence)?;
        
        Some(Expression::Infix(Box::new(left), operator, Box::new(right)))
    }
}
