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

    // Check if user is requesting a custom WebAssembly target
    let mut is_wasm_target = false;
    let mut file_idx = 1;

    for (i, arg) in args.iter().enumerate().skip(1) {
        if arg == "--target=wasm" || arg == "wasm" {
            is_wasm_target = true;
        } else if !arg.starts_with('-') {
            file_idx = i;
        }
    }

    let target_path = &args[file_idx];
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

    // Improved Multi-File Path Fallback Resolver
    let mut external_statements = Vec::new();
    for stmt in &program.statements {
        if let ast::Statement::Import(import_stmt) = stmt {
            // Choice A: Look relative to the current executed context
            let mut library_path = format!("apm_packages/{}.au", import_stmt.library);

            // Choice B: Fallback to project root directory vendor cache if invoked from a subfolder
            if !Path::new(&library_path).exists() {
                library_path = format!("../apm_packages/{}.au", import_stmt.library);
            }
            if !Path::new(&library_path).exists() {
                library_path = format!("examples/apm_packages/{}.au", import_stmt.library);
            }

            if Path::new(&library_path).exists() {
                println!(
                    "📦 Stitching ecosystem code tracking blocks from vendor cache: {}",
                    library_path
                );
                let mut lib_code = String::new();
                File::open(&library_path)?.read_to_string(&mut lib_code)?;

                let mut lib_parser = Parser::new(Lexer::new(&lib_code));
                let lib_program = lib_parser.parse_program();

                for lib_stmt in lib_program.statements {
                    external_statements.push(lib_stmt);
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

    program.statements.extend(external_statements);

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
    let ir_output = generator.generate_with_target(&program, is_wasm_target);
    File::create("output.ll")?.write_all(ir_output.as_bytes())?;

    if is_wasm_target {
        println!("⚙️  Spawning Clang background web-assembler (Target: wasm32)...");
        let status = Command::new("clang")
            .args([
                "--target=wasm32",
                "-nostdlib",
                "-Wl,--no-entry",
                "-Wl,--export-all",
                "output.ll",
                "-o",
                "aura_module.wasm",
            ])
            .status()?;

        if status.success() {
            println!(
                "🎉 SUCCESS! Web-Ready bytecode assembly module packaged: ./aura_module.wasm\n"
            );
        } else {
            println!(
                "⚠️ Notice: Standard Wasm linking requires 'lld' or target headers installed on your system.\n"
            );
        }
    } else {
        println!("⚙️  Spawning Clang background native desktop linker...");
        if Command::new("clang")
            .args(["output.ll", "-o", "aura_program"])
            .status()?
            .success()
        {
            println!("🎉 SUCCESS! Standalone native binary ready: ./aura_program\n");
        }
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
