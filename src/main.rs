mod tokens; mod lexer; mod ast; mod parser; mod codegen; mod semantic;

use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::codegen::CodeGenerator;
use crate::semantic::SemanticAnalyzer;

fn main() -> io::Result<()> {
    // 1. Ingest command-line arguments passed from the Linux shell terminal
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Aura Compiler toolchain v0.3.5 (Native Linux Platform)");
        println!("Usage: cargo run -- [file.au]");
        return Ok(());
    }

    let input_path = &args[1];
    if !Path::new(input_path).exists() {
        println!("Error: Target compilation file source '{}' not found.", input_path);
        return Ok(());
    }

    // 2. Stream read the raw text code directly out from your hard drive filesystem folder
    let mut file = File::open(input_path)?;
    let mut source_code = String::new();
    file.read_to_string(&mut source_code)?;

    println!("=== Commencing Aura Native Source Compilation: {} ===", input_path);

    // 3. Process Frontend Token Streams
    let lexer = Lexer::new(&source_code);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    if !parser.errors.is_empty() {
        println!("❌ Syntax Error compilation aborted:");
        for err in parser.errors { println!("  {}", err); }
        return Ok(());
    }

    // 4. Run Static Semantic Analysis Scope Verifications
    let mut analyzer = SemanticAnalyzer::new();
    // (Helper evaluation call mirror abstracted from internal testing layouts)
    for stmt in &program.statements {
        match stmt {
            ast::Statement::Let(l) => analyzer.declare_variable(l.name.clone(), false, l.token.line),
            ast::Statement::Const(c) => analyzer.declare_variable(c.name.clone(), true, c.token.line),
            ast::Statement::Assignment(a) => analyzer.check_mutability(&a.name, a.token.line),
            _ => {}
        }
    }

    if !analyzer.errors.is_empty() {
        println!("❌ Static Security Violation compilation aborted:");
        for err in analyzer.errors { println!("  {}", err); }
        return Ok(());
    }

    // 5. Generate Low-Level LLVM IR Text Stream Assemblies
    let mut generator = CodeGenerator::new();
    let ir_output = generator.generate(&program);

    // 6. Write the finalized compilation blocks directly out into a physical output.ll file
    let output_path = "output.ll";
    let mut out_file = File::create(output_path)?;
    out_file.write_all(ir_output.as_bytes())?;

    println!("✓ Successfully compiled assembly targets written to disk file: {}", output_path);
    println!("👉 Next step: Run 'clang output.ll -o aura_program' to pack your native binary executable!\n");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::ast::Node;
    use crate::codegen::CodeGenerator;

    #[test]
    fn run_all_ubuntu_compiler_checks() {
        let source = "fn calculate(a: Int, b: Int) -> Int {\nlet score = 100\n}\nlet final_score = calculate(5, 10)";
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        assert!(parser.errors.is_empty());

        let mut generator = CodeGenerator::new();
        let ir_output = generator.generate(&program);
        assert!(ir_output.contains("call i64 @calculate(i64 5, i64 10)"));
    }
}
