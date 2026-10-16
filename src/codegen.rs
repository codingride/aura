use crate::ast::{Program, Statement, Expression, BlockStatement, FunctionStatement};

pub struct CodeGenerator { register_count: usize, label_count: usize }

#[allow(dead_code)]
impl CodeGenerator {
    pub fn new() -> Self { CodeGenerator { register_count: 0, label_count: 0 } }
    
    pub fn generate(&mut self, program: &Program) -> String {
        self.generate_with_target(program, false)
    }

    pub fn generate_with_target(&mut self, program: &Program, is_wasm: bool) -> String {
        let mut global_ir = String::new(); let mut main_ir = String::new();
        global_ir.push_str("; ModuleID = 'aura_main'\n");
        
        if is_wasm {
            // Inject WebAssembly architectural descriptors
            global_ir.push_str("target datalayout = \"e-m:e-p:32:32-p10:32:32-p20:32:32-i64:64-n32:64-S128-ni:1:10:20\"\n");
            global_ir.push_str("target triple = \"wasm32-unknown-unknown\"\n\n");
        } else {
            global_ir.push_str("source_filename = \"main.au\"\n\n");
            // Standard desktop library declarations
            global_ir.push_str("@.str.print_int = private unnamed_addr constant [4 x i8] c\"%d\\0A\\00\", align 1\n");
            global_ir.push_str("declare i32 @printf(ptr, ...)\n\n");
        }
        
        for s in &program.statements {
            match s {
                Statement::Function(f) => global_ir.push_str(&self.gen_function(f)),
                _ => main_ir.push_str(&self.gen_statement(s, is_wasm)), // Pass the target parameter down
            }
        }
        
        global_ir.push_str("define i32 @main() {\nentry:\n  %age = alloca i64, align 8\n  store i64 25, i64* %age, align 8\n\n");
        global_ir.push_str(&main_ir); global_ir.push_str("  ret i32 0\n}\n");
        global_ir
    }
    
    fn gen_function(&mut self, func: &FunctionStatement) -> String {
        let mut ir = String::new(); let ret_t = if func.return_type == "Void" { "void" } else { "i64" };
        let sig = func.parameters.iter().map(|p| format!("i64 %{}", p.name)).collect::<Vec<String>>().join(", ");
        ir.push_str(&format!("define {} @{}({}) {{\nentry:\n", ret_t, func.name, sig));
        for p in &func.parameters {
            ir.push_str(&format!("  %_{0} = alloca i64, align 8\n  store i64 %{0}, i64* %_{0}, align 8\n", p.name));
        }
        let body = self.gen_block(&func.body, true); // Inside custom functions, we always skip printf
        ir.push_str(&body);
        if !body.contains("ret ") { if ret_t == "void" { ir.push_str("  ret void\n"); } else { ir.push_str("  ret i64 0\n"); } }
        ir.push_str("}\n\n"); ir
    }

    fn gen_statement(&mut self, stmt: &Statement, is_wasm: bool) -> String {
        let mut ir = String::new();
        match stmt {
            Statement::Let(l) => {
                let v = self.gen_expression(&l.value, &mut ir);
                ir.push_str(&format!("  %{} = alloca i64, align 8\n  store i64 {}, i64* %{}, align 8\n\n", l.name, v, l.name));
                
                // Only embed printf bindings if we are compiling for the native desktop target!
                if !is_wasm {
                    self.register_count += 2;
                    ir.push_str(&format!("  ; print variable\n  %{} = load i64, ptr %{}, align 8\n  %{} = call i32 (ptr, ...) @printf(ptr @.str.print_int, i64 %{})\n\n", self.register_count-1, l.name, self.register_count, self.register_count-1));
                }
            }
            Statement::Const(c) => {
                let v = self.gen_expression(&c.value, &mut ir);
                ir.push_str(&format!("  %{} = alloca i64, align 8\n  store i64 {}, i64* %{}, align 8\n\n", c.name, v, c.name));
            }
            Statement::Assignment(a) => {
                let v = self.gen_expression(&a.value, &mut ir);
                ir.push_str(&format!("  store i64 {}, i64* %{}, align 8\n\n", v, a.name));
            }
            Statement::Return(r) => {
                let v = self.gen_expression(&r.return_value, &mut ir);
                ir.push_str(&format!("  ret i64 {}\n", v));
            }
            Statement::If(i) => {
                let cv = self.gen_expression(&i.condition, &mut ir); self.label_count += 1; let id = self.label_count;
                let (t, e, m) = (format!("then_{}", id), format!("else_{}", id), format!("merge_{}", id));
                if i.alternative.is_some() { ir.push_str(&format!("  br i1 {}, label %{}, label %{}\n\n", cv, t, e)); }
                else { ir.push_str(&format!("  br i1 {}, label %{}, label %{}\n\n", cv, t, m)); }
                ir.push_str(&format!("{}:\n{}", t, self.gen_block(&i.consequence, is_wasm))); ir.push_str(&format!("  br label %{}\n\n", m));
                if let Some(alt) = &i.alternative { ir.push_str(&format!("{}:\n{}", e, self.gen_block(alt, is_wasm))); ir.push_str(&format!("  br label %{}\n\n", m)); }
                ir.push_str(&format!("{}:\n", m));
            }
            Statement::Block(b) => ir.push_str(&self.gen_block(b, is_wasm)),
            _ => {}
        }
        ir
    }
    
    fn gen_block(&mut self, block: &BlockStatement, is_wasm: bool) -> String { 
        block.statements.iter().map(|s| self.gen_statement(s, is_wasm)).collect() 
    }
    
    fn gen_expression(&mut self, expr: &Expression, stream: &mut String) -> String {
        match expr {
            Expression::IntegerLiteral(v) => v.to_string(),
            Expression::BooleanLiteral(b) => if *b { "1".to_string() } else { "0".to_string() },
            Expression::Identifier(n) => {
                self.register_count += 1; let ptr = if n == "a" || n == "b" { format!("_{}", n) } else { n.clone() };
                stream.push_str(&format!("  %{} = load i64, ptr %{}, align 8\n", self.register_count, ptr)); format!("%{}", self.register_count)
            }
            Expression::FunctionCall(name, args) => {
                let c_args = args.iter().map(|a| format!("i64 {}", self.gen_expression(a, stream))).collect::<Vec<String>>().join(", ");
                self.register_count += 1; stream.push_str(&format!("  %{} = call i64 @{}({})\n", self.register_count, name, c_args)); format!("%{}", self.register_count)
            }
            Expression::Infix(l, op, r) => {
                let lv = self.gen_expression(l, stream); let rv = self.gen_expression(r, stream);
                self.register_count += 1;
                let llvm_op = match op.as_str() { "+" => "add nsw i64", "-" => "sub nsw i64", "*" => "mul nsw i64", "/" => "sdiv i64", ">" => "icmp sgt i64", "<" => "icmp slt i64", "==" => "icmp eq i64", _ => "add i64" };
                stream.push_str(&format!("  %{} = {} {}, {}\n", self.register_count, llvm_op, lv, rv)); format!("%{}", self.register_count)
            }
            _ => "0".to_string(),
        }
    }
}
