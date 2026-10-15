use std::collections::HashMap;

#[allow(dead_code)]
pub struct VariableMeta {
    pub name: String,
    pub is_immutable: bool,
    pub scope_depth: usize,
}

pub struct SemanticAnalyzer {
    scopes: Vec<HashMap<String, VariableMeta>>,
    current_depth: usize,
    pub errors: Vec<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer { scopes: vec![HashMap::new()], current_depth: 0, errors: Vec::new() }
    }
    pub fn enter_scope(&mut self) { self.current_depth += 1; self.scopes.push(HashMap::new()); }
    pub fn exit_scope(&mut self) { if self.current_depth > 0 { self.scopes.pop(); self.current_depth -= 1; } }
    pub fn declare_variable(&mut self, name: String, is_immutable: bool, line: usize) {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name) {
                self.errors.push(format!("Semantic Error (Line {}): '{}' already declared.", line, name));
                return;
            }
            scope.insert(name.clone(), VariableMeta { name, is_immutable, scope_depth: self.current_depth });
        }
    }
    pub fn resolve_variable(&mut self, name: &str, line: usize) {
        for scope in self.scopes.iter().rev() { if scope.contains_key(name) { return; } }
        self.errors.push(format!("Compile-Time Security Error (Line {}): Out-of-scope variable '{}'.", line, name));
    }
    pub fn check_mutability(&mut self, name: &str, line: usize) {
        for scope in self.scopes.iter().rev() {
            if let Some(meta) = scope.get(name) {
                if meta.is_immutable { self.errors.push(format!("Compile-Time Immutability Error (Line {}): Cannot mutate constant '{}'.", line, name)); }
                return;
            }
        }
    }
}
