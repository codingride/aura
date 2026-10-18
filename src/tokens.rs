use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    Eof,
    Illegal,
    Identifier,
    IntLit,
    FloatLit,
    StringLit,
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Arrow,
    Lt,
    Gt,
    Eq,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Comma,
    Colon,
    Let,
    Const,
    Fn,
    Return,
    If,
    Else,
    Borrow,
    True,
    False,
}

pub static KEYWORDS: LazyLock<HashMap<&'static str, TokenType>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("let", TokenType::Let);
    m.insert("const", TokenType::Const);
    m.insert("fn", TokenType::Fn);
    m.insert("return", TokenType::Return);
    m.insert("if", TokenType::If);
    m.insert("else", TokenType::Else);
    m.insert("borrow", TokenType::Borrow);
    m.insert("true", TokenType::True);
    m.insert("false", TokenType::False);
    m
});

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String, line: usize, column: usize) -> Self {
        Token {
            token_type,
            literal,
            line,
            column,
        }
    }
}
