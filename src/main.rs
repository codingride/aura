mod tokens; mod lexer; mod ast; mod parser; mod codegen; mod semantic;

use std::{env, fs::File, io::{self, Read, Write}, path::Path, process::Command};
use crate::{lexer::Lexer, parser::Parser, codegen::CodeGenerator, semantic::SemanticAnalyzer};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { return run_repl(); }
    
    // Fix: Extract the targeted file path string element out from the terminal arguments vector slice
    let target_path = &args[1];
    
    if !Path::new(target_path).exists() { 
        println!("Error: File '{}' not found.", target_path); 
        return Ok(()); 
    }
    
    let mut code = String::new(); 
    File::open(target_path)?.read_to_string(&mut code)?;
    
    println!("=== Commencing Aura Compilation: {} ===", target_path);
    let mut parser = Parser::new(Lexer::new(&code)); 
    let program = parser.parse_program();
    
    if !parser.errors.is_empty() { 
        for err in parser.errors { println!("  {}", err); } 
        return Ok(()); 
    }
    
    let mut analyzer = SemanticAnalyzer::new();
    for s in &program.statements {
        match s {
            ast::Statement::Let(l) => {
                analyzer.declare_variable(l.name.clone(), false, l.token.line);
                if let ast::Expression::Identifier(source_var) = &l.value {
                    analyzer.move_resource(source_var, l.token.line);
                }
            }
            ast::Statement::Const(c) => {
                analyzer.declare_variable(c.name.clone(), true, c.token.line);
                if let ast::Expression::Identifier(source_var) = &c.value {
                    analyzer.move_resource(source_var, c.token.line);
                }
            }
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
    fn verify_compile_time_memory_sentinel() {
        let buggy_source = "if true { let secret = 42 }\nlet leakage = secret";
        let mut parser = Parser::new(Lexer::new(buggy_source));
        let program = parser.parse_program();
        let mut analyzer = SemanticAnalyzer::new();
        for stmt in &program.statements {
            match stmt {
                ast::Statement::Let(l) => {
                    analyzer.declare_variable(l.name.clone(), false, l.token.line);
                    if let ast::Expression::Identifier(n) = &l.value { analyzer.resolve_variable(n, l.token.line); }
                }
                ast::Statement::If(i) => {
                    analyzer.enter_scope();
                    for inner_stmt in &i.consequence.statements {
                        if let ast::Statement::Let(l) = inner_stmt { analyzer.declare_variable(l.name.clone(), false, l.token.line); }
                    }
                    analyzer.exit_scope();
                }
                _ => {}
            }
        }
        let has_error = analyzer.errors.iter().any(|err| err.contains("Out-of-scope variable 'secret'"));
        assert!(has_error, "Failed to catch out-of-scope variable reference leakage!");
    }

    #[test]
    fn verify_compile_time_immutability_sentinel() {
        let mut analyzer = SemanticAnalyzer::new();
        analyzer.declare_variable("pi".to_string(), true, 1);
        analyzer.check_mutability("pi", 2);
        let has_error = analyzer.errors.iter().any(|err| err.contains("Cannot mutate constant 'pi'"));
        assert!(has_error);
    }

    #[test]
    fn verify_compile_time_ownership_sentinel() {
        let mut analyzer = SemanticAnalyzer::new();
        analyzer.declare_variable("x".to_string(), false, 1);
        analyzer.move_resource("x", 2);
        analyzer.declare_variable("y".to_string(), false, 2);
        analyzer.resolve_variable("x", 3);

        println!("\n=== Running Aura Ownership Sentinel Checks ===");
        for err in &analyzer.errors {
            println!("  ❌ {}", err);
        }

        assert_eq!(analyzer.errors.len(), 1);
        let has_move_violation = analyzer.errors.iter().any(|err| err.contains("Cannot access variable 'x' because its ownership value has already been transferred"));
        assert!(has_move_violation);
        println!("✓ Success: Linear ownership sentinel caught use-after-move violations beautifully!\n");
    }
}
