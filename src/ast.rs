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
    Assignment(AssignmentStatement), // <-- Add this variant
    Block(BlockStatement),
    Function(FunctionStatement),
    If(IfStatement),
    Expression(Expression), 
}

impl Node for Statement {
    fn to_string(&self) -> String {
        match self {
            Statement::Let(stmt) => stmt.to_string(),
            Statement::Const(stmt) => stmt.to_string(),
            Statement::Assignment(stmt) => stmt.to_string(), // <-- Add this mapping
            Statement::Block(stmt) => stmt.to_string(),
            Statement::Function(stmt) => stmt.to_string(),
            Statement::If(stmt) => stmt.to_string(),
            Statement::Expression(expr) => expr.to_string(),
        }
    }
}

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
        let body = self.statements
            .iter()
            .map(|stmt| stmt.to_string())
            .collect::<Vec<String>>()
            .join("; ");
        format!("{{ {} }}", body)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionStatement {
    pub token: Token,              // The 'fn' token
    pub name: String,              // Function name
    pub parameters: Vec<Parameter>, // List of inputs
    pub return_type: String,       // Return type (defaults to "Void")
    pub body: BlockStatement,      // Inner code block
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

        format!("fn {}({}){} {}", self.name, params, ret_str, self.body.to_string())
    }
}

// Added the IfStatement structure layout
#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub token: Token,                    // The 'if' token
    pub condition: Expression,           // The condition expression
    pub consequence: BlockStatement,     // Code block executed if true
    pub alternative: Option<BlockStatement>, // Optional alternative block executed if false
}

impl Node for IfStatement {
    fn to_string(&self) -> String {
        let mut s = format!("if {} {}", self.condition.to_string(), self.consequence.to_string());
        if let Some(alt) = &self.alternative {
            s.push_str(&format!(" else {}", alt.to_string()));
        }
        s
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
        format!("const {}{} = {}", self.name, type_str, self.value.to_string())
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(String),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool), // <-- Added this variant
    Infix(Box<Expression>, String, Box<Expression>), 
}

impl Node for Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::Identifier(val) => val.clone(),
            Expression::IntegerLiteral(val) => val.to_string(),
            Expression::FloatLiteral(val) => val.to_string(),
            Expression::StringLiteral(val) => format!("\"{}\"", val),
            Expression::BooleanLiteral(val) => val.to_string(), // <-- Added this variant mapping
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

#[derive(Debug, Clone, PartialEq)]
pub struct AssignmentStatement {
    pub token: Token,     // The Assign '=' token
    pub name: String,     // Variable identifier being targeted
    pub value: Expression, // The new expression value being assigned
}

impl Node for AssignmentStatement {
    fn to_string(&self) -> String {
        format!("{} = {}", self.name, self.value.to_string())
    }
}
