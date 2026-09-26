use crate::lexer::Lexer;
use crate::tokens::{Token, TokenType};
use crate::ast::{Program, Statement, LetStatement, ConstStatement, Expression};

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
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }
        let name = self.cur_token.literal.clone();
        let mut explicit_type = None;

        if self.peek_token_is(TokenType::Colon) {
            self.next_token();
            if !self.expect_peek(TokenType::Identifier) {
                return None;
            }
            explicit_type = Some(self.cur_token.literal.clone());
        }

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        self.next_token();

        let value = self.parse_expression()?;

        Some(LetStatement {
            token,
            name,
            explicit_type,
            value,
        })
    }

    fn parse_const_statement(&mut self) -> Option<ConstStatement> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }
        let name = self.cur_token.literal.clone();
        let mut explicit_type = None;

        if self.peek_token_is(TokenType::Colon) {
            self.next_token();
            if !self.expect_peek(TokenType::Identifier) {
                return None;
            }
            explicit_type = Some(self.cur_token.literal.clone());
        }

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        self.next_token();

        let value = self.parse_expression()?;

        Some(ConstStatement {
            token,
            name,
            explicit_type,
            value,
        })
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        match self.cur_token.token_type {
            TokenType::Identifier => Some(Expression::Identifier(self.cur_token.literal.clone())),
            TokenType::IntLit => self.cur_token.literal.parse::<i64>().ok().map(Expression::IntegerLiteral),
            TokenType::FloatLit => self.cur_token.literal.parse::<f64>().ok().map(Expression::FloatLiteral),
            TokenType::StringLit => Some(Expression::StringLiteral(self.cur_token.literal.clone())),
            _ => {
                self.errors.push(format!("Line {}: No expression parser match found", self.cur_token.line));
                None
            }
        }
    }
}
