import sys
import os

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))

from src.lexer import Lexer
from src.parser import Parser

def run_parser_test():
    source_code = """
    let age = 42
    const pi: Float = 3.14
    let greeting = "Hello Aura"
    """

    print("=== Initializing Aura Parser Test ===")
    lexer = Lexer(source_code)
    parser = Parser(lexer)
    
    program = parser.parse_program()
    
    # Check if any errors occurred during parsing
    if len(parser.errors) > 0:
        print(f"❌ Test Failed! Found {len(parser.errors)} parsing errors:")
        for err in parser.errors:
            print(f"   -> {err}")
        return

    expected_ast_strings = [
        "let age = 42",
        "const pi: Float = 3.14",
        "let greeting = \"Hello Aura\""
    ]

    success = True
    statements = program.statements
    
    if len(statements) != len(expected_ast_strings):
        print(f"❌ Test Failed! Expected {len(expected_ast_strings)} statements, but parsed {len(statements)} instead.")
        success = False

    for i, stmt in enumerate(statements):
        stmt_str = str(stmt)
        if stmt_str != expected_ast_strings[i]:
            print(f"❌ AST Mismatch at statement index {i}!")
            print(f"   Expected: {expected_ast_strings[i]}")
            print(f"   Parsed:   {stmt_str}")
            success = False
        else:
            print(f"✓ AST node structured perfectly: {stmt_str}")

    if success:
        print("\n🎉 SUCCESS: The Parser mapped the Abstract Syntax Tree flawlessly! Week 2 is complete.")

if __name__ == "__main__":
    run_parser_test()
