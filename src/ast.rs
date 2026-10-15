use crate::tokens::Token;

#[allow(dead_code)]
pub trait Node { fn to_string(&self) -> String; }

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Const(ConstStatement),
    Assignment(AssignmentStatement),
    Return(ReturnStatement),
    Block(BlockStatement),
    Function(FunctionStatement),
    If(IfStatement),
    Expression(Expression), 
}

impl Node for Statement {
    fn to_string(&self) -> String {
        match self {
            Statement::Let(s) => s.to_string(),
            Statement::Const(s) => s.to_string(),
            Statement::Assignment(s) => s.to_string(),
            Statement::Return(s) => s.to_string(),
            Statement::Block(s) => s.to_string(),
            Statement::Function(s) => s.to_string(),
            Statement::If(s) => s.to_string(),
            Statement::Expression(e) => e.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter { pub name: String, pub param_type: String }

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatement { pub token: Token, pub statements: Vec<Statement> }

impl Node for BlockStatement {
    fn to_string(&self) -> String {
        let body = self.statements.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("; ");
        format!("{{ {} }}", body)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionStatement {
    pub token: Token, pub name: String, pub parameters: Vec<Parameter>, pub return_type: String, pub body: BlockStatement,
}

impl Node for FunctionStatement {
    fn to_string(&self) -> String {
        let params = self.parameters.iter().map(|p| format!("{}: {}", p.name, p.param_type)).collect::<Vec<String>>().join(", ");
        let ret = if self.return_type == "Void" { "".to_string() } else { format!(" -> {}", self.return_type) };
        format!("fn {}({}){} {}", self.name, params, ret, self.body.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement { pub token: Token, pub condition: Expression, pub consequence: BlockStatement, pub alternative: Option<BlockStatement> }

impl Node for IfStatement {
    fn to_string(&self) -> String {
        let mut s = format!("if {} {}", self.condition.to_string(), self.consequence.to_string());
        if let Some(alt) = &self.alternative { s.push_str(&format!(" else {}", alt.to_string())); }
        s
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement { pub token: Token, pub name: String, pub explicit_type: Option<String>, pub value: Expression }

impl Node for LetStatement {
    fn to_string(&self) -> String {
        let t = match &self.explicit_type { Some(t) => format!(": {}", t), None => "".to_string() };
        format!("let {}{} = {}", self.name, t, self.value.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstStatement { pub token: Token, pub name: String, pub explicit_type: Option<String>, pub value: Expression }

impl Node for ConstStatement {
    fn to_string(&self) -> String {
        let t = match &self.explicit_type { Some(t) => format!(": {}", t), None => "".to_string() };
        format!("const {}{} = {}", self.name, t, self.value.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssignmentStatement { pub token: Token, pub name: String, pub value: Expression }

impl Node for AssignmentStatement {
    fn to_string(&self) -> String { format!("{} = {}", self.name, self.value.to_string()) }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement { pub token: Token, pub return_value: Expression }

impl Node for ReturnStatement {
    fn to_string(&self) -> String { format!("return {}", self.return_value.to_string()) }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression { Identifier(String), IntegerLiteral(i64), FloatLiteral(f64), StringLiteral(String), BooleanLiteral(bool), Infix(Box<Expression>, String, Box<Expression>), FunctionCall(String, Vec<Expression>) }

impl Node for Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::Identifier(v) => v.clone(),
            Expression::IntegerLiteral(v) => v.to_string(),
            Expression::FloatLiteral(v) => v.to_string(),
            Expression::StringLiteral(v) => format!("\"{}\"", v),
            Expression::BooleanLiteral(v) => v.to_string(),
            Expression::Infix(l, op, r) => format!("({} {} {})", l.to_string(), op, r.to_string()),
            Expression::FunctionCall(name, args) => {
                let args_str = args.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(", ");
                format!("{}({})", name, args_str)
            }
        }
    }
}

pub struct Program { pub statements: Vec<Statement> }
impl Node for Program { fn to_string(&self) -> String { self.statements.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("\n") } }
