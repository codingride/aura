from src.tokens import Token, TokenType, KEYWORDS

class Lexer:
    def __init__(self, source_code: str):
        self.source = source_code
        self.position = 0      # Current position in source (points to current char)
        self.read_position = 0 # Current reading position (after current char)
        self.ch = ""           # Current char under examination
        
        # Tracking for clear error locations
        self.line = 1
        self.column = 0
        
        self.read_char()

    def read_char(self):
        """Advances the pointer by one character in the source text."""
        if self.read_position >= len(self.source):
            self.ch = "" # EOF (End of File) marker
        else:
            self.ch = self.source[self.read_position]
            
        self.position = self.read_position
        self.read_position += 1
        self.column += 1

    def peek_char(self) -> str:
        """Looks at the next character without advancing the lexer's position."""
        if self.read_position >= len(self.source):
            return ""
        return self.source[self.read_position]

    def skip_whitespace_and_comments(self):
        """Cleans up space, line breaks, and handles Aura's double-slash '//' comments."""
        while True:
            # Handle standard whitespace
            if self.ch in [' ', '\t', '\r', '\n']:
                if self.ch == '\n':
                    self.line += 1
                    self.column = 0
                self.read_char()
            # Handle comments: ignore everything until the end of the line
            elif self.ch == '/' and self.peek_char() == '/':
                while self.ch != '\n' and self.ch != "":
                    self.read_char()
            else:
                break

    def next_token(self) -> Token:
        """The main engine loop: Scans characters and outputs the next valid Token."""
        self.skip_whitespace_and_comments()
        
        token_type = TokenType.ILLEGAL
        literal = self.ch
        start_col = self.column

        if self.ch == "":
            return Token(TokenType.EOF, "", self.line, self.column)

        # Single-character tokens & operators
        elif self.ch == '=':
            token_type = TokenType.ASSIGN
        elif self.ch == '+':
            token_type = TokenType.PLUS
        elif self.ch == '-':
            # Look ahead to see if it's a thin arrow return '->'
            if self.peek_char() == '>':
                ch = self.ch
                self.read_char()
                literal = ch + self.ch
                token_type = TokenType.ARROW
            else:
                token_type = TokenType.MINUS
        elif self.ch == '*':
            token_type = TokenType.ASTERISK
        elif self.ch == '/':
            token_type = TokenType.SLASH
        elif self.ch == '(':
            token_type = TokenType.LPAREN
        elif self.ch == ')':
            token_type = TokenType.RPAREN
        elif self.ch == '{':
            token_type = TokenType.LBRACE
        elif self.ch == '}':
            token_type = TokenType.RBRACE
        elif self.ch == ',':
            token_type = TokenType.COMMA
        elif self.ch == ':':
            token_type = TokenType.COLON
            
        # String Literals
        elif self.ch == '"':
            literal = self.read_string()
            return Token(TokenType.STRING_LIT, literal, self.line, start_col)

        # Identifiers (variable names) & Keywords
        elif self.ch.isalpha() or self.ch == '_':
            literal = self.read_identifier()
            token_type = KEYWORDS.get(literal, TokenType.IDENTIFIER)
            return Token(token_type, literal, self.line, start_col)

        # Numeric Literals (Integers and Floats)
        elif self.ch.isdigit():
            literal, is_float = self.read_number()
            token_type = TokenType.FLOAT_LIT if is_float else TokenType.INT_LIT
            return Token(token_type, literal, self.line, start_col)

        # Unrecognized character fallback
        else:
            token_type = TokenType.ILLEGAL

        self.read_char()
        return Token(token_type, literal, self.line, start_col)

    def read_identifier(self) -> str:
        """Reads a continuous alphanumeric sequence for variable or keyword names."""
        start_position = self.position
        while self.ch.isalnum() or self.ch == '_':
            self.read_char()
        return self.source[start_position:self.position]

    def read_number(self) -> tuple[str, bool]:
        """Reads digits and checks if it's a Float containing a decimal point."""
        start_position = self.position
        is_float = False
        
        while self.ch.isdigit() or self.ch == '.':
            if self.ch == '.':
                if is_float: # Found a second decimal point (invalid)
                    break
                is_float = True
            self.read_char()
            
        return self.source[start_position:self.position], is_float

    def read_string(self) -> str:
        """Extracts text contained within double quotes."""
        self.read_char() # Advance past opening quote
        start_position = self.position
        
        while self.ch != '"' and self.ch != "":
            if self.ch == '\n':
                self.line += 1
                self.column = 0
            self.read_char()
            
        string_value = self.source[start_position:self.position]
        self.read_char() # Advance past closing quote
        return string_value
