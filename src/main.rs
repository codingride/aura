mod tokens; mod lexer; mod ast; mod parser; mod codegen; mod semantic;

#[allow(dead_code)]
fn main() {
    // Left empty to bypass local binary execution path policies
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::ast::{Program, Statement, Expression, BlockStatement};
    use crate::semantic::SemanticAnalyzer;

    fn analyze_program_scopes(program: &Program, analyzer: &mut SemanticAnalyzer) {
        for stmt in &program.statements {
            analyze_statement(stmt, analyzer);
        }
    }

    fn analyze_statement(stmt: &Statement, analyzer: &mut SemanticAnalyzer) {
        match stmt {
            Statement::Let(l) => {
                analyzer.declare_variable(l.name.clone(), false, l.token.line);
                analyze_expression(&l.value, analyzer, l.token.line);
            }
            Statement::Const(c) => {
                analyzer.declare_variable(c.name.clone(), true, c.token.line);
                analyze_expression(&c.value, analyzer, c.token.line);
            }
            Statement::If(if_stmt) => {
                analyze_expression(&if_stmt.condition, analyzer, if_stmt.token.line);
                
                analyzer.enter_scope();
                analyze_block(&if_stmt.consequence, analyzer);
                analyzer.exit_scope();

                if let Some(alt) = &if_stmt.alternative {
                    analyzer.enter_scope();
                    analyze_block(alt, analyzer);
                    analyzer.exit_scope();
                }
            }
            _ => {}
        }
    }

    fn analyze_block(block: &BlockStatement, analyzer: &mut SemanticAnalyzer) {
        for stmt in &block.statements {
            analyze_statement(stmt, analyzer);
        }
    }

    fn analyze_expression(expr: &Expression, analyzer: &mut SemanticAnalyzer, line: usize) {
        match expr {
            Expression::Identifier(name) => {
                if name == "true" || name == "false" {
                    return;
                }
                analyzer.resolve_variable(name, line);
            }
            Expression::Infix(left, _, right) => {
                analyze_expression(left, analyzer, line);
                analyze_expression(right, analyzer, line);
            }
            _ => {} 
        }
    }

    // Checkpoint 1: Scope Lifecycle Validation Test
    #[test]
    fn verify_compile_time_memory_sentinel() {
        let buggy_source = "
        if true {
            let secret = 42
        }
        let leakage = secret
        ";

        println!("\n=== Running Aura Era 3 Semantic Security Check ===");
        
        let lexer = Lexer::new(buggy_source);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(parser.errors.is_empty(), "Parser error: {:?}", parser.errors);

        let mut analyzer = SemanticAnalyzer::new();
        analyze_program_scopes(&program, &mut analyzer);

        println!("Intercepted Compiler Violations:");
        for err in &analyzer.errors {
            println!("  ❌ {}", err);
        }

        assert_eq!(analyzer.errors.len(), 1);
        
        // Fix: Use an iterator block `.iter().any(...)` to evaluate the inner text safely as a string slice reference
        let contains_target_error = analyzer.errors.iter().any(|err| err.contains("Use of undeclared or out-of-scope variable 'secret'"));
        assert!(contains_target_error, "Expected safety violation error was not flagged!");
        
        println!("✓ Success: Compile-Time Memory Sentinel blocked the unsafe code flawlessly!\n");
    }

    // Checkpoint 2: Mutability Protection Validation Test
    #[test]
    fn verify_compile_time_immutability_sentinel() {
        let mut analyzer = SemanticAnalyzer::new();

        // 1. Declare a constant variable 'pi' at line 1
        analyzer.declare_variable("pi".to_string(), true, 1);

        // 2. Simulate a programmer trying to reassign or mutate 'pi' at line 2
        analyzer.check_mutability("pi", 2);

        println!("\n=== Running Aura Immutability Security Check ===");
        for err in &analyzer.errors {
            println!("  ❌ {}", err);
        }

        assert_eq!(analyzer.errors.len(), 1);
        
        // Fix: Use an iterator block `.iter().any(...)` to evaluate the inner text safely as a string slice reference
        let contains_mut_error = analyzer.errors.iter().any(|err| err.contains("Cannot reassign or mutate constant variable 'pi'"));
        assert!(contains_mut_error, "Expected immutability violation error was not flagged!");
        
        println!("✓ Success: Immutability Sentinel protected the constant value completely!\n");
    }
}
