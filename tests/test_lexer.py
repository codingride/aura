import sys
import os

# Append the project root directory to the system path to allow local imports
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))

from src.lexer import Lexer
from src.tokens import TokenType

def run_lexer_test():
    # A robust sample snippet of Aura source code
    source_code = """
    // This is an Aura comment
    let age = 42
    const pi: Float = 3.14
    let greeting = "Hello Aura"
    
    fn add(a: Int, b: Int) -> Int {
        return a + b
    }
    """

    print("=== Initializing Aura Lexer Test ===")
    lexer = Lexer(source_code)
    
    # The exact sequence of tokens we expect the lexer to extract from the source code above
    expected_tokens = [
        (TokenType.LET, "let"),
        (TokenType.IDENTIFIER, "age"),
        (TokenType.ASSIGN, "="),
        (TokenType.INT_LIT, "42"),
        
        (TokenType.CONST, "const"),
        (TokenType.IDENTIFIER, "pi"),
        (TokenType.COLON, ":"),
        (TokenType.IDENTIFIER, "Float"),
        (TokenType.ASSIGN, "="),
        (TokenType.FLOAT_LIT, "3.14"),
        
        (TokenType.LET, "let"),
        (TokenType.IDENTIFIER, "greeting"),
        (TokenType.ASSIGN, "="),
        (TokenType.STRING_LIT, "Hello Aura"),
        
        (TokenType.FN, "fn"),
        (TokenType.IDENTIFIER, "add"),
        (TokenType.LPAREN, "("),
        (TokenType.IDENTIFIER, "a"),
        (TokenType.COLON, ":"),
        (TokenType.IDENTIFIER, "Int"),
        (TokenType.COMMA, ","),
        (TokenType.IDENTIFIER, "b"),
        (TokenType.COLON, ":"),
        (TokenType.IDENTIFIER, "Int"),
        (TokenType.RPAREN, ")"),
        (TokenType.ARROW, "->"),
        (TokenType.IDENTIFIER, "Int"),
        (TokenType.LBRACE, "{"),
        
        (TokenType.RETURN, "return"),
        (TokenType.IDENTIFIER, "a"),
        (TokenType.PLUS, "+"),
        (TokenType.IDENTIFIER, "b"),
        
        (TokenType.RBRACE, "}"),
        (TokenType.EOF, "")
    ]

    success = True
    for i, (expected_type, expected_literal) in enumerate(expected_tokens):
        token = lexer.next_token()
        
        # Verify both token categorization and string matches
        if token.type != expected_type or token.literal != expected_literal:
            print(f"❌ Test Failed at token position {i}!")
            print(f"   Expected: Token({expected_type.name}, '{expected_literal}')")
            print(f"   Received: {token}")
            success = False
            break
        else:
            print(f"✓ Validated: {token}")

    if success:
        print("\n🎉 SUCCESS: The Lexer extracted all tokens flawlessly! Week 1 is complete.")

if __name__ == "__main__":
    run_lexer_test()
