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
        SemanticAnalyzer {
            scopes: vec![HashMap::new()], 
            current_depth: 0,
            errors: Vec::new(),
        }
    }

    pub fn enter_scope(&mut self) {
        self.current_depth += 1;
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        if self.current_depth > 0 {
            self.scopes.pop(); 
            self.current_depth -= 1;
        }
    }

    pub fn declare_variable(&mut self, name: String, is_immutable: bool, line: usize) {
        if let Some(current_scope) = self.scopes.last_mut() {
            if current_scope.contains_key(&name) {
                self.errors.push(format!(
                    "Semantic Error (Line {}): Identifier '{}' has already been declared in this scope boundary.",
                    line, name
                ));
                return;
            }
            
            current_scope.insert(
                name.clone(),
                VariableMeta {
                    name,
                    is_immutable,
                    scope_depth: self.current_depth,
                },
            );
        }
    }

    pub fn resolve_variable(&mut self, name: &str, line: usize) {
        for scope in self.scopes.iter().rev() {
            if scope.contains_key(name) {
                return; 
            }
        }
        
        self.errors.push(format!(
            "Compile-Time Security Error (Line {}): Use of undeclared or out-of-scope variable '{}'.",
            line, name
        ));
    }

    // New Function: Intercepts reassignment mutations and verifies immutability constraints
    pub fn check_mutability(&mut self, name: &str, line: usize) {
        for scope in self.scopes.iter().rev() {
            if let Some(meta) = scope.get(name) {
                if meta.is_immutable {
                    self.errors.push(format!(
                        "Compile-Time Immutability Error (Line {}): Cannot reassign or mutate constant variable '{}'.",
                        line, name
                    ));
                }
                return;
            }
        }
    }
}
