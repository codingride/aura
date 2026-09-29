use crate::ast::{Expression, Program, Statement};

pub struct CodeGenerator {
    // Tracks a counter to give every temporary CPU register a unique name (e.g., %1, %2)
    register_count: usize,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator { register_count: 0 }
    }

    pub fn generate(&mut self, program: &Program) -> String {
        let mut llvm_ir = String::new();

        // Inject standard LLVM target boilerplate metadata header
        llvm_ir.push_str("; ModuleID = 'aura_main'\n");
        llvm_ir.push_str("source_filename = \"main.au\"\n\n");

        for stmt in &program.statements {
            match stmt {
                Statement::Let(let_stmt) => {
                    llvm_ir.push_str(&format!("; let {} = ...\n", let_stmt.name));
                    let val_ir = self.gen_expression(&let_stmt.value);

                    llvm_ir.push_str(&format!("  %{} = alloca i64, align 8\n", let_stmt.name));
                    llvm_ir.push_str(&format!(
                        "  store i64 {}, i64* %{}, align 8\n\n",
                        val_ir, let_stmt.name
                    ));
                }
                // Replace the Statement::Const branch in src/codegen.rs with this:
                Statement::Const(const_stmt) => {
                    llvm_ir.push_str(&format!("; const {} = ...\n", const_stmt.name));
                    let val_ir = self.gen_expression(&const_stmt.value);

                    llvm_ir.push_str(&format!("  %{} = alloca i64, align 8\n", const_stmt.name));
                    // Fix: Added val_ir as the second parameter to complete the formatting macro
                    llvm_ir.push_str(&format!(
                        "  store i64 {}, i64* %{}, align 8\n\n",
                        val_ir, const_stmt.name
                    ));
                }

                _ => {} // Handled in future optimization steps
            }
        }

        llvm_ir
    }

    fn gen_expression(&mut self, expr: &Expression) -> String {
        match expr {
            Expression::IntegerLiteral(val) => val.to_string(),
            Expression::Identifier(name) => format!("%{}", name),
            Expression::Infix(left, op, right) => {
                let left_val = self.gen_expression(left);
                let right_val = self.gen_expression(right);

                self.register_count += 1;
                let reg = self.register_count;

                // Translate high-level mathematical operations directly into CPU instructions
                let llvm_op = match op.as_str() {
                    "+" => "add nsw i64",
                    "-" => "sub nsw i64",
                    "*" => "mul nsw i64",
                    "/" => "sdiv i64",
                    _ => "add i64",
                };

                println!("  %{} = {} {}, {}", reg, llvm_op, left_val, right_val);
                format!("%{}", reg)
            }
            _ => "0".to_string(),
        }
    }
}
