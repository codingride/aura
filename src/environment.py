class Environment:
    def __init__(self):
        # Stores tuples of (value, is_immutable)
        self.store: dict[str, tuple[any, bool]] = {}

    def declare(self, name: str, value: any, is_immutable: bool) -> any:
        """Declares a variable or constant in the current scope."""
        if name in self.store:
            raise RuntimeError(f"Compile/Runtime Error: Identifier '{name}' has already been declared in this scope.")
        
        self.store[name] = (value, is_immutable)
        return value

    def get(self, name: str) -> any:
        """Retrieves a value from the store."""
        if name not in self.store:
            raise RuntimeError(f"Runtime Error: Undefined identifier '{name}'.")
        return self.store[name][0]

    def set(self, name: str, value: any) -> any:
        """Updates an existing variable's value, enforcing immutability constraints."""
        if name not in self.store:
            raise RuntimeError(f"Runtime Error: Cannot assign to undefined variable '{name}'.")
        
        _, is_immutable = self.store[name]
        if is_immutable:
            raise RuntimeError(f"Safety/Immutability Error: Cannot assign to constant '{name}'.")
            
        self.store[name] = (value, False)
        return value
