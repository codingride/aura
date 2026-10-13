mod tokens; mod lexer; mod ast; mod parser; mod codegen; mod semantic;

use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::Command;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::codegen::CodeGenerator;
use crate::semantic::SemanticAnalyzer;

fn main() -> io::Result<()> {
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

    let mut file = File::open(input_path)?;
    let mut source_code = String::new();
    file.read_to_string(&mut source_code)?;

    println!("=== Commencing Aura Native Source Compilation: {} ===", input_path);

    let lexer = Lexer::new(&source_code);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    if !parser.errors.is_empty() {
        println!("❌ Syntax Error compilation aborted:");
        for err in parser.errors { println!("  {}", err); }
        return Ok(());
    }

    let mut analyzer = SemanticAnalyzer::new();
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

    let mut generator = CodeGenerator::new();
    let ir_output = generator.generate(&program);

    let temp_ll_path = "output.ll";
    let mut out_file = File::create(temp_ll_path)?;
    out_file.write_all(ir_output.as_bytes())?;

    println!("✓ Intermediate assembly written to: {}", temp_ll_path);
    println!("⚙️  Spawning background system process to link native assembler via Clang...");
    
    let binary_output_name = "aura_program";
    let clang_status = Command::new("clang")
        .arg(temp_ll_path)
        .arg("-o")
        .arg(binary_output_name)
        .status();

    match clang_status {
        Ok(status) if status.success() => {
            println!("🎉 SUCCESS: Binary compilation successfully completed!");
            println!("🚀 Your standalone native executable machine binary is ready: ./{}", binary_output_name);
        }
        Ok(status) => {
            println!("❌ Clang Error: Assembler failed with status: {}", status);
        }
        Err(e) => {
            println!("❌ System Process Error: Could not execute 'clang'. Details: {}", e);
        }
    }

    println!();
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::codegen::CodeGenerator;

    #[test]
    fn run_all_ubuntu_compiler_checks() {
        let source = "fn add(a: Int, b: Int) -> Int {\nreturn a + b\n}\nlet final_score = add(5, 10)";
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        assert!(parser.errors.is_empty());

        let mut generator = CodeGenerator::new();
        let ir_output = generator.generate(&program);
        
        // Assert return opcode generation mechanics are functional
        assert!(ir_output.contains("ret i64"));
        assert!(ir_output.contains("call i64 @add"));
    }
}
