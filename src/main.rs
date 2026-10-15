mod tokens; mod lexer; mod ast; mod parser; mod codegen; mod semantic;

use std::{env, fs::File, io::{self, Read, Write}, path::Path, process::Command};
use crate::{lexer::Lexer, parser::Parser, codegen::CodeGenerator, semantic::SemanticAnalyzer};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { return run_repl(); }
    let path = &args[1];
    if !Path::new(path).exists() { println!("Error: File '{}' not found.", path); return Ok(()); }
    let mut code = String::new(); File::open(path)?.read_to_string(&mut code)?;
    println!("=== Commencing Aura Compilation: {} ===", path);
    let mut parser = Parser::new(Lexer::new(&code)); let program = parser.parse_program();
    if !parser.errors.is_empty() { for err in parser.errors { println!("  {}", err); } return Ok(()); }
    let mut analyzer = SemanticAnalyzer::new();
    for s in &program.statements {
        match s {
            ast::Statement::Let(l) => analyzer.declare_variable(l.name.clone(), false, l.token.line),
            ast::Statement::Const(c) => analyzer.declare_variable(c.name.clone(), true, c.token.line),
            ast::Statement::Assignment(a) => analyzer.check_mutability(&a.name, a.token.line),
            _ => {}
        }
    }
    if !analyzer.errors.is_empty() { for err in analyzer.errors { println!("  {}", err); } return Ok(()); }
    File::create("output.ll")?.write_all(CodeGenerator::new().generate(&program).as_bytes())?;
    println!("⚙️ Spawning Clang background linker...");
    if Command::new("clang").args(["output.ll", "-o", "aura_program"]).status()?.success() {
        println!("🎉 SUCCESS! Standalone native binary ready: ./aura_program\n");
    }
    Ok(())
}

fn run_repl() -> io::Result<()> {
    use crate::ast::Node;
    println!("🌌 Aura Interactive Playground (v0.3.5)\n👉 Type code line-by-line. Type 'exit' to quit.\n");
    loop {
        print!("aura >> "); io::stdout().flush()?; let mut buf = String::new(); io::stdin().read_line(&mut buf)?;
        let trimmed = buf.trim(); if trimmed == "exit" { println!("Goodbye! 🚀"); break; } if trimmed.is_empty() { continue; }
        let mut parser = Parser::new(Lexer::new(trimmed)); let program = parser.parse_program();
        if parser.errors.is_empty() { println!("  📊 AST Node: {}", program.to_string()); }
        else { for err in parser.errors { println!("  ❌ {}", err); } }
        println!();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_all_features() {
        let mut p = Parser::new(Lexer::new("fn add(a: Int, b: Int) -> Int {\nreturn a + b\n}\nlet x = add(5, 10)"));
        let prog = p.parse_program(); assert!(p.errors.is_empty());
        let ir = CodeGenerator::new().generate(&prog);
        assert!(ir.contains("ret i64")); assert!(ir.contains("call i64 @add"));
    }
}
