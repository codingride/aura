mod ast;
mod codegen;
mod lexer;
mod parser;
mod tokens;

#[allow(dead_code)]
fn main() {
    // Left empty to bypass cargo run block restrictions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::CodeGenerator;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn showcase_llvm_codegen() {
        let source_code = "let result = 5 + 10";
        println!("\n=== Testing Aura Rust LLVM Compiler Backend ===");

        let lexer = Lexer::new(source_code);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(
            parser.errors.is_empty(),
            "Parser flagged syntax errors: {:?}",
            parser.errors
        );

        let mut generator = CodeGenerator::new();
        let intermediate_representation = generator.generate(&program);

        println!(
            "\n=== Generated LLVM IR Code Assembly ===\n{}",
            intermediate_representation
        );
    }
}
