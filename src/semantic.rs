use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResourceState {
    Active,
    Moved, // Marks that ownership has been transferred and this variable is now unsafe dead memory
}

#[allow(dead_code)]
pub struct VariableMeta {
    pub name: String,
    pub is_immutable: bool,
    pub scope_depth: usize,
    pub state: ResourceState, // <-- Track current life state flag parameters
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

    // pub fn enter_scope(&mut self) {
    //     self.current_depth += 1;
    //     self.scopes.push(HashMap::new());
    // }
    // pub fn exit_scope(&mut self) {
    //     if self.current_depth > 0 {
    //         self.scopes.pop();
    //         self.current_depth -= 1;
    //     }
    // }

    pub fn declare_variable(&mut self, name: String, is_immutable: bool, line: usize) {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name) {
                self.errors.push(format!(
                    "Semantic Error (Line {}): '{}' already declared.",
                    line, name
                ));
                return;
            }
            scope.insert(
                name.clone(),
                VariableMeta {
                    name,
                    is_immutable,
                    scope_depth: self.current_depth,
                    state: ResourceState::Active, // Initial state is always Active
                },
            );
        }
    }

    // pub fn resolve_variable(&mut self, name: &str, line: usize) {
    //     for scope in self.scopes.iter().rev() {
    //         if let Some(meta) = scope.get(name) {
    //             // Security Check: If a variable is read but its status is marked as Moved, flag an extraction breach!
    //             if meta.state == ResourceState::Moved {
    //                 self.errors.push(format!(
    //                     "Compile-Time Use-After-Move Violation (Line {}): Cannot access variable '{}' because its ownership value has already been transferred.", 
    //                     line, name
    //                 ));
    //             }
    //             return;
    //         }
    //     }
    //     self.errors.push(format!(
    //         "Compile-Time Security Error (Line {}): Out-of-scope variable '{}'.",
    //         line, name
    //     ));
    // }

    pub fn check_mutability(&mut self, name: &str, line: usize) {
        for scope in self.scopes.iter().rev() {
            if let Some(meta) = scope.get(name) {
                if meta.is_immutable {
                    self.errors.push(format!(
                        "Compile-Time Immutability Error (Line {}): Cannot mutate constant '{}'.",
                        line, name
                    ));
                }
                return;
            }
        }
    }

    // New Function: Dynamically triggers a linear memory ownership transfer move action
    pub fn move_resource(&mut self, name: &str, line: usize) {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(meta) = scope.get_mut(name) {
                if meta.state == ResourceState::Moved {
                    self.errors.push(format!(
                        "Compile-Time Ownership Error (Line {}): Cannot move variable '{}' because it has already been moved.", 
                        line, name
                    ));
                } else {
                    meta.state = ResourceState::Moved; // Freeze the identifier out of existence!
                }
                return;
            }
        }
    }
}
