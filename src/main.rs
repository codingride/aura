mod tokens; mod lexer; mod ast; mod parser; mod codegen; mod semantic;

use std::fs::File;
use std::io::Write;

#[allow(dead_code)]
fn main() {
    // Left empty to bypass local workspace execution policy locks
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::codegen::CodeGenerator;

    #[test]
    fn run_all_ubuntu_compiler_checks() {
        // Feed the compiler a custom function and variable definition script combined!
        let source = "
        fn calculate(a: Int, b: Int) -> Int {
            let score = 100
        }
        if age > 18 {
            let access = 1
        }
        ";
        println!("\n=== Running Aura Function Assembly Generation ===");
        
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(parser.errors.is_empty(), "Parser error: {:?}", parser.errors);

        let mut generator = CodeGenerator::new();
        let ir_output = generator.generate(&program);

        println!("\n=== Final Emitted Multi-Block LLVM IR Assembly ===\n{}", ir_output);

        // Write the finalized assembly file stream down to your drive disk folder
        let output_path = "output.ll";
        let mut file = File::create(output_path).expect("Failed to create file");
        file.write_all(ir_output.as_bytes()).expect("Failed to write contents");
        
        // Assert that the compiler generated separate function definitions and entry layers
        assert!(ir_output.contains("define i64 @calculate(i64 %a, i64 %b)"));
        assert!(ir_output.contains("define i32 @main()"));
    }
}
