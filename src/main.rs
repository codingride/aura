mod tokens;
mod lexer;
mod ast;
mod parser;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::ast::Node;

fn main() {
    let source_code = "let age = 42\nconst pi: Float = 3.14";
    println!("=== Testing Aura Rust Compiler Frontend ===");
    
    let lexer = Lexer::new(source_code);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    if !parser.errors.is_empty() {
        println!("❌ Found parsing errors:");
        for err in parser.errors {
            println!("  {}", err);
        }
        return;
    }

    println!("Successfully parsed AST layout:\n{}", program.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_parser() {
        let source = "let age = 42\nconst pi: Float = 3.14\nlet msg = \"Hello Aura\"";
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(parser.errors.is_empty(), "Parser flagged syntax errors: {:?}", parser.errors);

        let expected = vec![
            "let age = 42",
            "const pi: Float = 3.14",
            "let msg = \"Hello Aura\""
        ];

        assert_eq!(program.statements.len(), expected.len());

        for (i, stmt) in program.statements.iter().enumerate() {
            assert_eq!(stmt.to_string(), expected[i]);
        }
    }
}
