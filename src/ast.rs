use crate::tokens::Token;
#[allow(dead_code)]
pub trait Node {
    fn to_string(&self) -> String;
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Const(ConstStatement),
    Block(BlockStatement),
    Function(FunctionStatement),
    Expression(Expression),
}

impl Node for Statement {
    fn to_string(&self) -> String {
        match self {
            Statement::Let(stmt) => stmt.to_string(),
            Statement::Const(stmt) => stmt.to_string(),
            Statement::Block(stmt) => stmt.to_string(),
            Statement::Function(stmt) => stmt.to_string(),
            Statement::Expression(expr) => expr.to_string(),
        }
    }
}

// Represents an input parameter name and type, e.g., "a: Int"
#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub param_type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatement {
    pub token: Token, // The '{' token
    pub statements: Vec<Statement>,
}

impl Node for BlockStatement {
    fn to_string(&self) -> String {
        let body = self
            .statements
            .iter()
            .map(|stmt| stmt.to_string())
            .collect::<Vec<String>>()
            .join("; ");
        format!("{{ {} }}", body)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionStatement {
    pub token: Token,               // The 'fn' token
    pub name: String,               // Function name
    pub parameters: Vec<Parameter>, // List of inputs
    pub return_type: String,        // Return type (defaults to "Void")
    pub body: BlockStatement,       // Inner code block
}

impl Node for FunctionStatement {
    fn to_string(&self) -> String {
        let params = self.parameters
            .iter()
            .map(|p| format!("{}: {}", p.name, p.param_type))
            .collect::<Vec<String>>()
            .join(", ");
        
        let ret_str = if self.return_type == "Void" {
            "".to_string()
        } else {
            format!(" -> {}", self.return_type)
        };

        // Clean, standard engineering approach without text-replacement hacks
        format!("fn {}({}){} {}", self.name, params, ret_str, self.body.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement {
    pub token: Token,
    pub name: String,
    pub explicit_type: Option<String>,
    pub value: Expression,
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
    pub token: Token,
    pub name: String,
    pub explicit_type: Option<String>,
    pub value: Expression,
}

impl Node for ConstStatement {
    fn to_string(&self) -> String {
        let type_str = match &self.explicit_type {
            Some(t) => format!(": {}", t),
            None => "".to_string(),
        };
        format!(
            "const {}{} = {}",
            self.name,
            type_str,
            self.value.to_string()
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(String),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    Infix(Box<Expression>, String, Box<Expression>),
}

impl Node for Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::Identifier(val) => val.clone(),
            Expression::IntegerLiteral(val) => val.to_string(),
            Expression::FloatLiteral(val) => val.to_string(),
            Expression::StringLiteral(val) => format!("\"{}\"", val),
            Expression::Infix(left, op, right) => {
                format!("({} {} {})", left.to_string(), op, right.to_string())
            }
        }
    }
}

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
