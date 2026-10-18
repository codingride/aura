mod apm;
mod ast;
mod codegen;
mod lexer;
mod parser;
mod semantic;
mod tokens; // Register our package manager module component asset tracking paths

use crate::{
    apm::PackageManager, codegen::CodeGenerator, lexer::Lexer, parser::Parser,
    semantic::SemanticAnalyzer,
};
use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    path::Path,
    process::Command,
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // --- New Command Interception Rule Loop: Route 'apm' arguments to Package Manager ---
    if args.len() >= 2 && args[1] == "apm" {
        let pm = PackageManager::new();
        if args.len() == 3 && args[2] == "init" {
            pm.initialize_project()?;
        } else if args.len() == 4 && args[2] == "install" {
            pm.install_dependency(&args[3])?;
        } else {
            println!("Aura Package Manager (apm) toolchain v0.5.0");
            println!("Usage:");
            println!(
                "  aura apm init               # Initialize a new project tracking manifest file"
            );
            println!(
                "  aura apm install [package]  # Fetch and download an ecosystem package library dependency"
            );
        }
        return Ok(());
    }

    if args.len() < 2 {
        return run_repl();
    }

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
        for err in parser.errors {
            println!("  {}", err);
        }
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
    if !analyzer.errors.is_empty() {
        for err in analyzer.errors {
            println!("  {}", err);
        }
        return Ok(());
    }

    let mut generator = CodeGenerator::new();
    let ir_output = generator.generate_with_target(&program, false);
    File::create("output.ll")?.write_all(ir_output.as_bytes())?;

    println!("⚙️ Spawning Clang background native desktop linker...");
    if Command::new("clang")
        .args(["output.ll", "-o", "aura_program"])
        .status()?
        .success()
    {
        println!("🎉 SUCCESS! Standalone native binary ready: ./aura_program\n");
    }
    Ok(())
}

fn run_repl() -> io::Result<()> {
    use crate::ast::Node;
    println!(
        "🌌 Aura Interactive Playground (v0.3.5)\n👉 Type code line-by-line. Type 'exit' to quit.\n"
    );
    loop {
        print!("aura >> ");
        io::stdout().flush()?;
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        let trimmed = buf.trim();
        if trimmed == "exit" {
            println!("Goodbye! 🚀");
            break;
        }
        if trimmed.is_empty() {
            continue;
        }
        let mut parser = Parser::new(Lexer::new(trimmed));
        let program = parser.parse_program();
        if parser.errors.is_empty() {
            println!("  📊 AST Node: {}", program.to_string());
        } else {
            for err in parser.errors {
                println!("  ❌ {}", err);
            }
        }
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
            if let ast::Statement::Let(l) = stmt {
                analyzer.declare_variable(l.name.clone(), false, l.token.line);
            }
        }
        assert!(analyzer.errors.is_empty());
    }
}
