use crate::tokens::{Token, TokenType, KEYWORDS};

pub struct Lexer {
    source: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source_code: &str) -> Self {
        let mut l = Lexer {
            source: source_code.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
            line: 1,
            column: 0,
        };
        l.read_char();
        l
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.source.len() {
            self.ch = '\0';
        } else {
            self.ch = self.source[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
        self.column += 1;
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.source.len() { '\0' } else { self.source[self.read_position] }
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            if self.ch.is_whitespace() {
                if self.ch == '\n' { self.line += 1; self.column = 0; }
                self.read_char();
            } else if self.ch == '/' && self.peek_char() == '/' {
                while self.ch != '\n' && self.ch != '\0' { self.read_char(); }
            } else { break; }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();
        let start_col = self.column;
        let token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::Eq, "==".to_string(), self.line, start_col)
                } else {
                    Token::new(TokenType::Assign, self.ch.to_string(), self.line, start_col)
                }
            }
            '<' => Token::new(TokenType::Lt, self.ch.to_string(), self.line, start_col),
            '>' => Token::new(TokenType::Gt, self.ch.to_string(), self.line, start_col),
            '+' => Token::new(TokenType::Plus, self.ch.to_string(), self.line, start_col),
            '*' => Token::new(TokenType::Asterisk, self.ch.to_string(), self.line, start_col),
            '/' => Token::new(TokenType::Slash, self.ch.to_string(), self.line, start_col),
            '(' => Token::new(TokenType::Lparen, self.ch.to_string(), self.line, start_col),
            ')' => Token::new(TokenType::Rparen, self.ch.to_string(), self.line, start_col),
            '{' => Token::new(TokenType::Lbrace, self.ch.to_string(), self.line, start_col),
            '}' => Token::new(TokenType::Rbrace, self.ch.to_string(), self.line, start_col),
            ',' => Token::new(TokenType::Comma, self.ch.to_string(), self.line, start_col),
            ':' => Token::new(TokenType::Colon, self.ch.to_string(), self.line, start_col),
            '-' => {
                if self.peek_char() == '>' {
                    self.read_char();
                    Token::new(TokenType::Arrow, "->".to_string(), self.line, start_col)
                } else {
                    Token::new(TokenType::Minus, self.ch.to_string(), self.line, start_col)
                }
            }
            '"' => return Token::new(TokenType::StringLit, self.read_string(), self.line, start_col),
            '\0' => Token::new(TokenType::Eof, "".to_string(), self.line, start_col),
            c if c.is_alphabetic() || c == '_' => {
                let lit = self.read_identifier();
                let t = KEYWORDS.get(lit.as_str()).cloned().unwrap_or(TokenType::Identifier);
                return Token::new(t, lit, self.line, start_col);
            }
            c if c.is_numeric() => {
                let (lit, is_f) = self.read_number();
                let t = if is_f { TokenType::FloatLit } else { TokenType::IntLit };
                return Token::new(t, lit, self.line, start_col);
            }
            _ => Token::new(TokenType::Illegal, self.ch.to_string(), self.line, start_col),
        };
        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while self.ch.is_alphanumeric() || self.ch == '_' { self.read_char(); }
        self.source[start..self.position].iter().collect()
    }

    fn read_number(&mut self) -> (String, bool) {
        let start = self.position;
        let mut is_f = false;
        while self.ch.is_numeric() || self.ch == '.' {
            if self.ch == '.' { if is_f { break; } is_f = true; }
            self.read_char();
        }
        (self.source[start..self.position].iter().collect(), is_f)
    }

    fn read_string(&mut self) -> String {
        self.read_char();
        let start = self.position;
        while self.ch != '"' && self.ch != '\0' {
            if self.ch == '\n' { self.line += 1; self.column = 0; }
            self.read_char();
        }
        let s = self.source[start..self.position].iter().collect();
        self.read_char();
        s
    }
}
