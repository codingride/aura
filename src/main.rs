mod apm;
mod ast;
mod codegen;
mod lexer;
mod parser;
mod semantic;
mod tokens;

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

    if args.len() >= 2 && args[1] == "apm" {
        let pm = PackageManager::new();
        if args.len() == 3 && args[2] == "init" {
            pm.initialize_project()?;
        } else if args.len() == 4 && args[2] == "install" {
            pm.install_dependency(&args[3])?;
        } else {
            println!(
                "Aura Package Manager (apm) toolchain v0.5.0\nUsage:\n  aura apm init\n  aura apm install [package]"
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

    println!(
        "=== Commencing Aura Ecosystem Linkage Compilation: {} ===",
        target_path
    );
    let mut parser = Parser::new(Lexer::new(&code));
    let mut program = parser.parse_program();

    // --- New Feature Loop: Multi-File Deep Package Resolver Stitching ---
    let mut external_statements = Vec::new();
    for stmt in &program.statements {
        if let ast::Statement::Import(import_stmt) = stmt {
            let library_path = format!("apm_packages/{}.au", import_stmt.library);
            if Path::new(&library_path).exists() {
                println!(
                    "📦 Stitching ecosystem code tracking blocks from vendor cache: {}",
                    library_path
                );
                let mut lib_code = String::new();
                File::open(&library_path)?.read_to_string(&mut lib_code)?;

                // Parse the downloaded file into a clean parallel AST structure tree
                let mut lib_parser = Parser::new(Lexer::new(&lib_code));
                let lib_program = lib_parser.parse_program();

                for lib_stmt in lib_program.statements {
                    external_statements.push(lib_stmt); // Extract function fragments into active compiler stream
                }
            } else {
                println!(
                    "❌ Compilation Error: Imported dependency library '{}' not found. Run 'aura apm install' first.",
                    import_stmt.library
                );
                return Ok(());
            }
        }
    }

    // Append the loaded package dependency modules straight into the global executable tree stream!
    program.statements.extend(external_statements);

    if !parser.errors.is_empty() {
        for err in parser.errors {
            println!("  {}", err);
        }
        return Ok(());
    }

    // Safety sentinels validation sweep pass
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

    File::create("output.ll")?.write_all(
        CodeGenerator::new()
            .generate_with_target(&program, false)
            .as_bytes(),
    )?;
    println!("⚙️  Spawning Clang background linker...");
    if Command::new("clang")
        .args(["output.ll", "-o", "aura_program"])
        .status()?
        .success()
    {
        println!("🎉 SUCCESS! Ecosystem Linked standalone binary ready: ./aura_program\n");
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
    fn verify_module_imports_parsing() {
        let mut p = Parser::new(Lexer::new(
            "import standard_math\nlet result = add_bonus(10)",
        ));
        let prog = p.parse_program();
        assert!(p.errors.is_empty(), "Failed to parse import token streams!");
        let has_import = prog
            .statements
            .iter()
            .any(|s| matches!(s, ast::Statement::Import(_)));
        assert!(has_import);
    }
}
