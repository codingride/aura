mod tokens; mod lexer; mod ast; mod parser; mod codegen;

#[allow(dead_code)]
fn main() {}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::codegen::CodeGenerator;

    #[test]
    fn run_all_ubuntu_compiler_checks() {
        // Feed the compiler an explicit conditional statement script code block
        let source = "if true {\nlet target = 99\n} else {\nlet target = 0\n}";
        println!("\n=== Testing Aura Native Linux LLVM Codegen ===");
        
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(parser.errors.is_empty(), "Parser error: {:?}", parser.errors);

        let mut generator = CodeGenerator::new();
        let ir_output = generator.generate(&program);

        println!("\n=== Generated Branching LLVM IR Assembly ===\n{}", ir_output);
        
        // Assert our structural labels are generated perfectly into the memory assembly layout
        assert!(ir_output.contains("br i1"));
        assert!(ir_output.contains("then_1:"));
        assert!(ir_output.contains("else_1:"));
        assert!(ir_output.contains("merge_1:"));
    }
}
