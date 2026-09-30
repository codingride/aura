mod tokens; mod lexer; mod ast; mod parser; mod codegen;

#[allow(dead_code)]
fn main() {}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::ast::Node;

    #[test]
    fn run_all_ubuntu_compiler_checks() {
        // Feed the compiler an explicit relational numeric check statement script block
        let source = "if age > 18 {\nlet access = 1\n}";
        println!("\n=== Testing Aura Comparison Parser Systems ===");
        
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(parser.errors.is_empty(), "Parser error encountered: {:?}", parser.errors);

        // Verify the math operators grouped correctly into our string tree representations
        let expected_ast = "if (age > 18) { let access = 1 }";
        assert_eq!(program.to_string(), expected_ast);
        println!("✓ Relational dynamic AST nodes clustered perfectly: {}\n", program.to_string());
    }
}
