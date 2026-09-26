use crate::tokens::Token;

// The base trait that all nodes in the Aura tree must support
pub trait Node {
    fn to_string(&self) -> String;
}

// --- Statement Types ---
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Const(ConstStatement),
    Expression(Expression), // Allows standalone values or function calls to act as statements
}

impl Node for Statement {
    fn to_string(&self) -> String {
        match self {
            Statement::Let(stmt) => stmt.to_string(),
            Statement::Const(stmt) => stmt.to_string(),
            Statement::Expression(expr) => expr.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement {
    pub token: Token,              // The Token::Let token
    pub name: String,              // The variable identifier name
    pub explicit_type: Option<String>, // Optional explicit type annotation (e.g., Some("Int"))
    pub value: Expression,         // The inner expression value being assigned
}

impl Node for LetStatement {
    fn to_string(&self) -> String {
        let type_str = match &self.explicit_type {
            Some(t) => format!(": {}", t),
            None => "".to_string(),
        };
        format!("let {}{} = {}", self.name, type_str, self.value.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstStatement {
    pub token: Token,              // The Token::Const token
    pub name: String,              // The constant identifier name
    pub explicit_type: Option<String>,
    pub value: Expression,
}

impl Node for ConstStatement {
    fn to_string(&self) -> String {
        let type_str = match &self.explicit_type {
            Some(t) => format!(": {}", t),
            None => "".to_string(),
        };
        format!("const {}{} = {}", self.name, type_str, self.value.to_string())
    }
}

// --- Expression Types ---

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(String),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
}

impl Node for Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::Identifier(val) => val.clone(),
            Expression::IntegerLiteral(val) => val.to_string(),
            Expression::FloatLiteral(val) => val.to_string(),
            Expression::StringLiteral(val) => format!("\"{}\"", val),
        }
    }
}

// --- Program Root Node ---

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn to_string(&self) -> String {
        self.statements
            .iter()
            .map(|stmt| stmt.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
