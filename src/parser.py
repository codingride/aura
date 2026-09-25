from src.lexer import Lexer
from src.tokens import Token, TokenType
from src.ast import Program, LetStatement, ConstStatement, Identifier, IntegerLiteral, FloatLiteral, StringLiteral, Expression, Statement

# Define operator precedence levels
LOWEST = 1
EQUALS = 2  # =
SUM = 3     # + or -
PRODUCT = 4 # * or /

class Parser:
    def __init__(self, lexer: Lexer):
        self.l = lexer
        self.errors: list[str] = []
        
        self.cur_token: Token = None
        self.peek_token: Token = None
        
        # Read two tokens so cur_token and peek_token are both set
        self.next_token()
        self.next_token()

    def next_token(self):
        """Advances both the current and peek tokens."""
        self.cur_token = self.peek_token
        self.peek_token = self.l.next_token()

    def cur_token_is(self, t: TokenType) -> bool:
        return self.cur_token.type == t

    def peek_token_is(self, t: TokenType) -> bool:
        return self.peek_token.type == t

    def expect_peek(self, t: TokenType) -> bool:
        """Enforces grammar structure. If the next token matches, advance. Otherwise, log an error."""
        if self.peek_token_is(t):
            self.next_token()
            return True
        else:
            self.peek_error(t)
            return False

    def peek_error(self, t: TokenType):
        msg = f"Line {self.peek_token.line}, Col {self.peek_token.column}: Expected next token to be {t.name}, got {self.peek_token.type.name} instead"
        self.errors.append(msg)

    def parse_program(self) -> Program:
        """Root parsing loop that structures the tokens into a program node."""
        program = Program()
        
        while not self.cur_token_is(TokenType.EOF):
            stmt = self.parse_statement()
            if stmt is not None:
                program.statements.append(stmt)
            self.next_token()
            
        return program

    def parse_statement(self) -> Statement:
        """Determines the path of statement parsing based on the current token."""
        if self.cur_token_is(TokenType.LET):
            return self.parse_let_statement()
        elif self.cur_token_is(TokenType.CONST):
            return self.parse_const_statement()
        else:
            # For Phase 1, we only focus on let/const declarations.
            # Unhandled standalone tokens are skipped or treated as invalid statements.
            return None

    def parse_let_statement(self) -> LetStatement:
        """Parses: let <identifier> [ : <type> ] = <expression>"""
        token = self.cur_token # The 'let' token
        
        if not self.expect_peek(TokenType.IDENTIFIER):
            return None
            
        name = Identifier(self.cur_token, self.cur_token.literal)
        explicit_type = None
        
        # Check if there is an optional explicit type declaration (e.g., ': Int')
        if self.peek_token_is(TokenType.COLON):
            self.next_token() # Current token is now ':'
            if not self.expect_peek(TokenType.IDENTIFIER):
                return None
            explicit_type = Identifier(self.cur_token, self.cur_token.literal)
            
        if not self.expect_peek(TokenType.ASSIGN):
            return None
            
        self.next_token() # Skip the '=' to get to the expression value
        
        value = self.parse_expression(LOWEST)
        
        return LetStatement(token=token, name=name, explicit_type=explicit_type, value=value)

    def parse_const_statement(self) -> ConstStatement:
        """Parses: const <identifier> [ : <type> ] = <expression>"""
        token = self.cur_token # The 'const' token
        
        if not self.expect_peek(TokenType.IDENTIFIER):
            return None
            
        name = Identifier(self.cur_token, self.cur_token.literal)
        explicit_type = None
        
        if self.peek_token_is(TokenType.COLON):
            self.next_token()
            if not self.expect_peek(TokenType.IDENTIFIER):
                return None
            explicit_type = Identifier(self.cur_token, self.cur_token.literal)
            
        if not self.expect_peek(TokenType.ASSIGN):
            return None
            
        self.next_token()
        
        value = self.parse_expression(LOWEST)
        
        return ConstStatement(token=token, name=name, explicit_type=explicit_type, value=value)

    def parse_expression(self, precedence: int) -> Expression:
        """Dispatches expression routing based on literal sub-types."""
        if self.cur_token_is(TokenType.IDENTIFIER):
            return Identifier(self.cur_token, self.cur_token.literal)
        elif self.cur_token_is(TokenType.INT_LIT):
            return IntegerLiteral(self.cur_token, int(self.cur_token.literal))
        elif self.cur_token_is(TokenType.FLOAT_LIT):
            return FloatLiteral(self.cur_token, float(self.cur_token.literal))
        elif self.cur_token_is(TokenType.STRING_LIT):
            return StringLiteral(self.cur_token, self.cur_token.literal)
        else:
            self.errors.append(f"Line {self.cur_token.line}: No expression parsing function found for {self.cur_token.type.name}")
            return None
