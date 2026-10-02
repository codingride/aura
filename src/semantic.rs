use std::collections::HashMap;

#[allow(dead_code)]
pub struct VariableMeta {
    pub name: String,
    pub is_immutable: bool,
    pub scope_depth: usize,
}

pub struct SemanticAnalyzer {
    // A stack of symbol maps representing nested scopes (e.g., global, function, inside an if-block)
    scopes: Vec<HashMap<String, VariableMeta>>,
    current_depth: usize,
    pub errors: Vec<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            scopes: vec![HashMap::new()], // Initialize with global scope layer
            current_depth: 0,
            errors: Vec::new(),
        }
    }

    /// Enters a new code block scope (e.g., hitting a opening curly brace '{')
    pub fn enter_scope(&mut self) {
        self.current_depth += 1;
        self.scopes.push(HashMap::new());
    }

    /// Exits a code block scope (e.g., hitting a closing curly brace '}')
    /// This is where we catch memory lifecycles!
    pub fn exit_scope(&mut self) {
        if self.current_depth > 0 {
            self.scopes.pop(); // Automatically destroys all variable entries tracking inside this inner block
            self.current_depth -= 1;
        }
    }

    /// Records a new variable declaration in the current active scope
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

    /// Checks if a variable is valid and safely accessible at the current line of code
    pub fn resolve_variable(&mut self, name: &str, line: usize) {
        // Look from the inner-most scope outwards to global scope
        for scope in self.scopes.iter().rev() {
            if scope.contains_key(name) {
                return; // Safe access found!
            }
        }
        
        // If it isn't found anywhere in active memory paths, flag a security violation compile error!
        self.errors.push(format!(
            "Compile-Time Security Error (Line {}): Use of undeclared or out-of-scope variable '{}'.",
            line, name
        ));
    }
}
