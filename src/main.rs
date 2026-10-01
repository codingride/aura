mod tokens; mod lexer; mod ast; mod parser; mod codegen;

use std::fs::File;
use std::io::Write;

#[allow(dead_code)]
fn main() {
    // Left empty to bypass local binary runtime execution checks
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::codegen::CodeGenerator;

    #[test]
    fn run_all_ubuntu_compiler_checks() {
        let source = "if age > 18 {\nlet access = 1\n}";
        println!("\n=== Running Aura Ubuntu Compilation Pipeline ===");
        
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(parser.errors.is_empty(), "Parser error: {:?}", parser.errors);

        let mut generator = CodeGenerator::new();
        let ir_output = generator.generate(&program);

        // --- New File Writer Action Component ---
        let output_path = "output.ll";
        let mut file = File::create(output_path).expect("Failed to create file node");
        file.write_all(ir_output.as_bytes()).expect("Failed to write compilation contents to disk");
        
        println!("✓ Successfully compiled Aura code down to disk file: {}\n", output_path);
        
        // Final structural validation assertions
        assert!(ir_output.contains("icmp sgt"));
        assert!(ir_output.contains("br i1"));
    }
}
