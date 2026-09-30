use crate::ast::{Program, Statement, Expression, BlockStatement};

#[allow(dead_code)]
pub struct CodeGenerator {
    register_count: usize,
    label_count: usize,
}

#[allow(dead_code)]
impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator { 
            register_count: 0,
            label_count: 0,
        }
    }

    pub fn generate(&mut self, program: &Program) -> String {
        let mut ir = String::new();
        ir.push_str("; ModuleID = 'aura_main'\nsource_filename = \"main.au\"\n\n");
        
        for stmt in &program.statements {
            ir.push_str(&self.gen_statement(stmt));
        }
        ir
    }

    fn gen_statement(&mut self, stmt: &Statement) -> String {
        let mut ir = String::new();
        match stmt {
            Statement::Let(l) => {
                ir.push_str(&format!("; let {} = ...\n", l.name));
                let val = self.gen_expression(&l.value);
                ir.push_str(&format!("  %{} = alloca i64, align 8\n", l.name));
                ir.push_str(&format!("  store i64 {}, i64* %{}, align 8\n\n", val, l.name));
            }
            Statement::Const(c) => {
                ir.push_str(&format!("; const {} = ...\n", c.name));
                let val = self.gen_expression(&c.value);
                ir.push_str(&format!("  %{} = alloca i64, align 8\n", c.name));
                ir.push_str(&format!("  store i64 {}, i64* %{}, align 8\n\n", val, c.name));
            }
            Statement::If(if_stmt) => {
                ir.push_str("; if condition branching\n");
                let cond_val = self.gen_expression(&if_stmt.condition);
                
                self.label_count += 1;
                let id = self.label_count;
                
                let then_label = format!("then_{}", id);
                let else_label = format!("else_{}", id);
                let merge_label = format!("merge_{}", id);
                
                // 1. Emit the conditional jump command (i1 means a 1-bit boolean flag)
                if if_stmt.alternative.is_some() {
                    ir.push_str(&format!("  br i1 {}, label %{}, label %{}\n\n", cond_val, then_label, else_label));
                } else {
                    ir.push_str(&format!("  br i1 {}, label %{}, label %{}\n\n", cond_val, then_label, merge_label));
                }
                
                // 2. Emit the 'Then' block path
                ir.push_str(&format!("{}:\n", then_label));
                ir.push_str(&self.gen_block(&if_stmt.consequence));
                ir.push_str(&format!("  br label %{}\n\n", merge_label));
                
                // 3. Emit the optional 'Else' block path
                if let Some(alt) = &if_stmt.alternative {
                    ir.push_str(&format!("{}:\n", else_label));
                    ir.push_str(&self.gen_block(alt));
                    ir.push_str(&format!("  br label %{}\n\n", merge_label));
                }
                
                // 4. Emit the compilation merge anchor point where paths rejoin
                ir.push_str(&format!("{}:\n", merge_label));
            }
            Statement::Block(b) => {
                ir.push_str(&self.gen_block(b));
            }
            _ => {}
        }
        ir
    }

    fn gen_block(&mut self, block: &BlockStatement) -> String {
        let mut ir = String::new();
        for stmt in &block.statements {
            ir.push_str(&self.gen_statement(stmt));
        }
        ir
    }

    fn gen_expression(&mut self, expr: &Expression) -> String {
        match expr {
            Expression::IntegerLiteral(v) => v.to_string(),
            Expression::BooleanLiteral(b) => if *b { "1".to_string() } else { "0".to_string() },
            Expression::Identifier(n) => format!("%{}", n),
            Expression::Infix(l, op, r) => {
                let lv = self.gen_expression(l); 
                let rv = self.gen_expression(r);
                self.register_count += 1; 
                let reg = self.register_count;
                
                let llvm_op = match op.as_str() { 
                    "+" => "add nsw i64", 
                    "-" => "sub nsw i64", 
                    "*" => "mul nsw i64", 
                    "/" => "sdiv i64", 
                    _ => "add i64" 
                };
                
                // Fixed: Print the math registration block out to standard stream like Phase 1
                println!("  %{} = {} {}, {}", reg, llvm_op, lv, rv);
                format!("%{}", reg)
            }
            _ => "0".to_string(),
        }
    }
}
