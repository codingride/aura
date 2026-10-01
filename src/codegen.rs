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
        
        // 1. Declare the global formatting string constant for printing integers with a newline
        ir.push_str("@.str.print_int = private unnamed_addr constant [4 x i8] c\"%d\\0A\\00\", align 1\n\n");
        
        // 2. Declare the external standard C library function print module
        ir.push_str("declare i32 @printf(ptr, ...)\n\n");

        ir.push_str("define i32 @main() {\n");
        ir.push_str("entry:\n");

        ir.push_str("  %age = alloca i64, align 8\n");
        ir.push_str("  store i64 25, i64* %age, align 8\n\n");

        for stmt in &program.statements {
            ir.push_str(&self.gen_statement(stmt));
        }
        
        ir.push_str("  ret i32 0\n");
        ir.push_str("}\n");
        ir
    }

    fn gen_statement(&mut self, stmt: &Statement) -> String {
        let mut ir = String::new();
        match stmt {
            Statement::Let(l) => {
                ir.push_str(&format!("; let {} = ...\n", l.name));
                let val = self.gen_expression(&l.value, &mut ir);
                ir.push_str(&format!("  %{} = alloca i64, align 8\n", l.name));
                ir.push_str(&format!("  store i64 {}, i64* %{}, align 8\n\n", val, l.name));
                
                // --- Fix: Swapped 'let _unused_call' with a standard LLVM register value layout ---
                self.register_count += 1;
                let loaded_reg = self.register_count;
                self.register_count += 1;
                let print_call_reg = self.register_count;
                
                ir.push_str(&format!("  ; print the variable out to screen\n"));
                ir.push_str(&format!("  %{} = load i64, ptr %{}, align 8\n", loaded_reg, l.name));
                ir.push_str(&format!("  %{} = call i32 (ptr, ...) @printf(ptr @.str.print_int, i64 %{})\n\n", print_call_reg, loaded_reg));
            }
            Statement::Const(c) => {
                ir.push_str(&format!("; const {} = ...\n", c.name));
                let val = self.gen_expression(&c.value, &mut ir);
                ir.push_str(&format!("  %{} = alloca i64, align 8\n", c.name));
                ir.push_str(&format!("  store i64 {}, i64* %{}, align 8\n\n", val, c.name));
            }
            Statement::If(if_stmt) => {
                ir.push_str("; if condition branching\n");
                let cond_val = self.gen_expression(&if_stmt.condition, &mut ir);
                
                self.label_count += 1;
                let id = self.label_count;
                
                let then_label = format!("then_{}", id);
                let else_label = format!("else_{}", id);
                let merge_label = format!("merge_{}", id);
                
                if if_stmt.alternative.is_some() {
                    ir.push_str(&format!("  br i1 {}, label %{}, label %{}\n\n", cond_val, then_label, else_label));
                } else {
                    ir.push_str(&format!("  br i1 {}, label %{}, label %{}\n\n", cond_val, then_label, merge_label));
                }
                
                ir.push_str(&format!("{}:\n", then_label));
                ir.push_str(&self.gen_block(&if_stmt.consequence));
                ir.push_str(&format!("  br label %{}\n\n", merge_label));
                
                if let Some(alt) = &if_stmt.alternative {
                    ir.push_str(&format!("{}:\n", else_label));
                    ir.push_str(&self.gen_block(alt));
                    ir.push_str(&format!("  br label %{}\n\n", merge_label));
                }
                
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

    fn gen_expression(&mut self, expr: &Expression, stream: &mut String) -> String {
        match expr {
            Expression::IntegerLiteral(v) => v.to_string(),
            Expression::BooleanLiteral(b) => if *b { "1".to_string() } else { "0".to_string() },
            Expression::Identifier(n) => {
                self.register_count += 1;
                let reg = self.register_count;
                stream.push_str(&format!("  %{} = load i64, ptr %{}, align 8\n", reg, n));
                format!("%{}", reg)
            }
            Expression::Infix(l, op, r) => {
                let lv = self.gen_expression(l, stream); 
                let rv = self.gen_expression(r, stream);
                self.register_count += 1; 
                let reg = self.register_count;
                
                match op.as_str() {
                    "+" => { stream.push_str(&format!("  %{} = add nsw i64 {}, {}\n", reg, lv, rv)); }
                    "-" => { stream.push_str(&format!("  %{} = sub nsw i64 {}, {}\n", reg, lv, rv)); }
                    "*" => { stream.push_str(&format!("  %{} = mul nsw i64 {}, {}\n", reg, lv, rv)); }
                    "/" => { stream.push_str(&format!("  %{} = sdiv i64 {}, {}\n", reg, lv, rv)); }
                    ">" => { stream.push_str(&format!("  %{} = icmp sgt i64 {}, {}\n", reg, lv, rv)); }
                    "<" => { stream.push_str(&format!("  %{} = icmp slt i64 {}, {}\n", reg, lv, rv)); }
                    "==" => { stream.push_str(&format!("  %{} = icmp eq i64 {}, {}\n", reg, lv, rv)); }
                    _ => { stream.push_str(&format!("  %{} = add i64 {}, {}\n", reg, lv, rv)); }
                }
                format!("%{}", reg)
            }
            _ => "0".to_string(),
        }
    }
}
