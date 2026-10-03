mod tokens; mod lexer; mod ast; mod parser; mod codegen; mod semantic;

#[allow(dead_code)]
fn main() {
    // Left empty to bypass local execution constraints
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
            // New Integration: If the script mutates an existing variable, invoke the mutability check!
            Statement::Assignment(a) => {
                analyzer.check_mutability(&a.name, a.token.line);
                analyze_expression(&a.value, analyzer, a.token.line);
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
                if name == "true" || name == "false" { return; }
                analyzer.resolve_variable(name, line);
            }
            Expression::Infix(left, _, right) => {
                analyze_expression(left, analyzer, line);
                analyze_expression(right, analyzer, line);
            }
            _ => {} 
        }
    }

    #[test]
    fn verify_compile_time_memory_sentinel() {
        let buggy_source = "if true { let secret = 42 }\nlet leakage = secret";
        let lexer = Lexer::new(buggy_source);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(parser.errors.is_empty(), "Parser error: {:?}", parser.errors);

        let mut analyzer = SemanticAnalyzer::new();
        analyze_program_scopes(&program, &mut analyzer);

        let contains_target_error = analyzer.errors.iter().any(|err| err.contains("Use of undeclared or out-of-scope variable 'secret'"));
        assert!(contains_target_error);
    }

    // Upgraded: This test now reads a real source code string containing an illegal mutation!
    #[test]
    fn verify_compile_time_immutability_sentinel() {
        let faulty_code = "
        const pi = 3.14
        pi = 4.0
        ";

        println!("\n=== Running Aura End-to-End Immutability Check ===");
        let lexer = Lexer::new(faulty_code);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(parser.errors.is_empty(), "Parser error: {:?}", parser.errors);

        let mut analyzer = SemanticAnalyzer::new();
        analyze_program_scopes(&program, &mut analyzer);

        println!("Intercepted Mutability Violations:");
        for err in &analyzer.errors {
            println!("  ❌ {}", err);
        }

        assert_eq!(analyzer.errors.len(), 1);
        let contains_mut_error = analyzer.errors.iter().any(|err| err.contains("Cannot reassign or mutate constant variable 'pi'"));
        assert!(contains_mut_error);
        println!("✓ Success: Compiler safely blocked script file constant mutation from compiling!\n");
    }
}
