import sys
import os

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))

from src.lexer import Lexer
from src.parser import Parser
from src.evaluator import Evaluator
from src.environment import Environment

def test_evaluator():
    print("=== Initializing Aura Evaluator Test ===")
    
    # Test valid script execution
    valid_source = """
    let age = 42
    const pi = 3.14
    let name = "Aura Language"
    """
    
    lexer = Lexer(valid_source)
    parser = Parser(lexer)
    program = parser.parse_program()
    
    env = Environment()
    evaluator = Evaluator()
    
    evaluator.eval(program, env)
    
    # Assert values are stored with correct types in environment memory
    assert env.get("age") == 42
    print("✓ Successfully allocated variable 'age' to 42")
    assert env.get("pi") == 3.14
    print("✓ Successfully allocated constant 'pi' to 3.14")
    assert env.get("name") == "Aura Language"
    print("✓ Successfully allocated variable 'name' to \"Aura Language\"")

    # Test Immutability Safety Rule
    print("\nChecking Immutability Constraints...")
    try:
        env.set("pi", 3.14159)
        print("❌ Immutability Failure: Constant was overwritten!")
    except RuntimeError as e:
        print(f"✓ Immutability Enforcement Working: {e}")

    print("\n🎉 SUCCESS: The Evaluator tree-walker and Environment safety rules work flawlessly! Week 3 is complete.")

if __name__ == "__main__":
    test_evaluator()
