use std::collections::HashMap;
use std::sync::LazyLock; // Safe, efficient static evaluation initialization

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
  // Special Tokens
  Eof,
  Illegal,

  // Identifiers & Literals
  Identifier,
  IntLit,
  FloatLit,
  StringLit,

  // Operators
  Assign,    // '='
  Plus,      // '+'
  Minus,     // '-'
  Asterisk,  // '*'
  Slash,     // '/'
  Arrow,     // '->'

  // Delimiters
  Lparen,    // '('
  Rparen,    // ')'
  Lbrace,    // '{'
  Rbrace,    // '}'
  Comma,     // ','
  Colon,     // ':'

  // Keywords
  Let,
  Const,
  Fn,
  Return,
  If,
  Else,
  Borrow,
}

// Global immutable Map for fast keyword resolution
pub static KEYWORDS: LazyLock<HashMap<&'static str, TokenType>> = LazyLock::new(|| {
  let mut m = HashMap::new();
  m.insert("let", TokenType::Let);
  m.insert("const", TokenType::Const);
  m.insert("fn", TokenType::Fn);
  m.insert("return", TokenType::Return);
  m.insert("if", TokenType::If);
  m.insert("else", TokenType::Else);
  m.insert("borrow", TokenType::Borrow);
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
