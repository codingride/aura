from abc import ABC, abstractmethod
from src.tokens import Token

class ASTNode(ABC):
    @abstractmethod
    def __str__(self) -> str:
        pass

class Statement(ASTNode, ABC):
    """Base class for all action-oriented structures (let, const, return)."""
    pass

class Expression(ASTNode, ABC):
    """Base class for all value-producing structures (literals, math, names)."""
    pass

class Program(ASTNode):
    """The root node of the entire source code file."""
    def __init__(self):
        self.statements: list[Statement] = []

    def __str__(self) -> str:
        return "\n".join(str(stmt) for stmt in self.statements)

# --- Statements ---

class LetStatement(Statement):
    """Represents 'let x = <expression>' or 'let x: Type = <expression>'"""
    def __init__(self, token: Token, name: 'Identifier', explicit_type: 'Identifier' = None, value: Expression = None):
        self.token = token               # The TokenType.LET token
        self.name = name                 # Identifier node for the variable name
        self.explicit_type = explicit_type # Optional explicit type Identifier node
        self.value = value               # Expression node evaluated to assign

    def __str__(self) -> str:
        type_str = f": {self.explicit_type}" if self.explicit_type else ""
        return f"let {self.name}{type_str} = {self.value}"

class ConstStatement(Statement):
    """Represents 'const x = <expression>'"""
    def __init__(self, token: Token, name: 'Identifier', explicit_type: 'Identifier' = None, value: Expression = None):
        self.token = token               # The TokenType.CONST token
        self.name = name                 # Identifier node
        self.explicit_type = explicit_type 
        self.value = value

    def __str__(self) -> str:
        type_str = f": {self.explicit_type}" if self.explicit_type else ""
        return f"const {self.name}{type_str} = {self.value}"

# --- Expressions ---

class Identifier(Expression):
    """Represents a variable reference or type name (e.g., 'age', 'Int')."""
    def __init__(self, token: Token, value: str):
        self.token = token
        self.value = value               # The actual string literal name

    def __str__(self) -> str:
        return self.value

class IntegerLiteral(Expression):
    """Represents an integer value node (e.g., 42)."""
    def __init__(self, token: Token, value: int):
        self.token = token
        self.value = value

    def __str__(self) -> str:
        return str(self.value)

class FloatLiteral(Expression):
    """Represents a float value node (e.g., 3.14)."""
    def __init__(self, token: Token, value: float):
        self.token = token
        self.value = value

    def __str__(self) -> str:
        return str(self.value)

class StringLiteral(Expression):
    """Represents a string value node (e.g., "Hello Aura")."""
    def __init__(self, token: Token, value: str):
        self.token = token
        self.value = value

    def __str__(self) -> str:
        return f'"{self.value}"'
