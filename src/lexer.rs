use crate::tokens::{Token, TokenType, KEYWORDS};

pub struct Lexer {
    source: Vec<char>,
    position: usize,       // Points to current char
    read_position: usize,  // Points after current char
    ch: char,              // Current char under examination
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
        if self.read_position >= self.source.len() {
            '\0'
        } else {
            self.source[self.read_position]
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            if self.ch.is_whitespace() {
                if self.ch == '\n' {
                    self.line += 1;
                    self.column = 0;
                }
                self.read_char();
            } else if self.ch == '/' && self.peek_char() == '/' {
                while self.ch != '\n' && self.ch != '\0' {
                    self.read_char();
                }
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();

        let start_col = self.column;
        let token = match self.ch {
            '=' => Token::new(TokenType::Assign, self.ch.to_string(), self.line, start_col),
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
            '"' => {
                let literal = self.read_string();
                return Token::new(TokenType::StringLit, literal, self.line, start_col);
            }
            '\0' => Token::new(TokenType::Eof, "".to_string(), self.line, start_col),
            c if c.is_alphabetic() || c == '_' => {
                let literal = self.read_identifier();
                let token_type = KEYWORDS.get(literal.as_str()).cloned().unwrap_or(TokenType::Identifier);
                return Token::new(token_type, literal, self.line, start_col);
            }
            c if c.is_numeric() => {
                let (literal, is_float) = self.read_number();
                let token_type = if is_float { TokenType::FloatLit } else { TokenType::IntLit };
                return Token::new(token_type, literal, self.line, start_col);
            }
            _ => Token::new(TokenType::Illegal, self.ch.to_string(), self.line, start_col),
        };

        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while self.ch.is_alphanumeric() || self.ch == '_' {
            self.read_char();
        }
        self.source[start..self.position].iter().collect()
    }

    fn read_number(&mut self) -> (String, bool) {
        let start = self.position;
        let mut is_float = false;
        while self.ch.is_numeric() || self.ch == '.' {
            if self.ch == '.' {
                if is_float { break; }
                is_float = true;
            }
            self.read_char();
        }
        (self.source[start..self.position].iter().collect(), is_float)
    }

    fn read_string(&mut self) -> String {
        self.read_char(); // Advance past opening quote
        let start = self.position;
        while self.ch != '"' && self.ch != '\0' {
            if self.ch == '\n' {
                self.line += 1;
                self.column = 0;
            }
            self.read_char();
        }
        let string_val: String = self.source[start..self.position].iter().collect();
        self.read_char(); // Advance past closing quote
        string_val
    }
}
