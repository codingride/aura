from enum import Enum, auto
from dataclasses import dataclass

class TokenType(Enum):
    # Special Tokens
    EOF = auto()          # End of file
    ILLEGAL = auto()      # Unrecognized character

    # Identifiers & Literals
    IDENTIFIER = auto()   # variable names, function names (e.g., 'name', 'age')
    INT_LIT = auto()      # Integer literal (e.g., '42')
    FLOAT_LIT = auto()    # Float literal (e.g., '3.14')
    STRING_LIT = auto()   # String literal (e.g., '"Hello Aura"')

    # Operators
    ASSIGN = auto()       # '='
    PLUS = auto()         # '+'
    MINUS = auto()        # '-'
    ASTERISK = auto()     # '*'
    SLASH = auto()        # '/'
    ARROW = auto()        # '->'

    # Delimiters
    LPAREN = auto()       # '('
    RPAREN = auto()       # ')'
    LBRACE = auto()       # '{'
    RBRACE = auto()       # '}'
    COMMA = auto()        # ','
    COLON = auto()        # ':'

    # Keywords
    LET = auto()          # 'let'
    CONST = auto()        # 'const'
    FN = auto()           # 'fn'
    RETURN = auto()       # 'return'
    IF = auto()           # 'if'
    ELSE = auto()         # 'else'
    BORROW = auto()       # 'borrow'


# Map string keywords directly to their token types for fast lookup
KEYWORDS = {
    "let": TokenType.LET,
    "const": TokenType.CONST,
    "fn": TokenType.FN,
    "return": TokenType.RETURN,
    "if": TokenType.IF,
    "else": TokenType.ELSE,
    "borrow": TokenType.BORROW,
}


@dataclass
class Token:
    type: TokenType
    literal: str
    line: int
    column: int

    def __repr__(self) -> str:
        return f"Token({self.type.name}, '{self.literal}', line={self.line}, col={self.column})"
