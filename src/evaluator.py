from src.ast import ASTNode, Program, LetStatement, ConstStatement, Identifier, IntegerLiteral, FloatLiteral, StringLiteral
from src.environment import Environment

class Evaluator:
    def eval(self, node: ASTNode, env: Environment) -> any:
        if isinstance(node, Program):
            last_evaluated = None
            for statement in node.statements:
                # Intercept special native function prints for our Week 4 milestone
                if hasattr(statement, 'name') and statement.name.value == "print":
                    val = self.eval(statement.value, env)
                    print(val) # Route out to underlying standard OS output stream
                    last_evaluated = val
                    continue
                    
                last_evaluated = self.eval(statement, env)
            return last_evaluated
        
        # Keep all other original if/elif conditions exactly as they were...
        elif isinstance(node, LetStatement):
            val = self.eval(node.value, env)
            return env.declare(node.name.value, val, is_immutable=False)
        elif isinstance(node, ConstStatement):
            val = self.eval(node.value, env)
            return env.declare(node.name.value, val, is_immutable=True)
        elif isinstance(node, Identifier):
            return env.get(node.value)
        elif isinstance(node, IntegerLiteral):
            return node.value
        elif isinstance(node, FloatLiteral):
            return node.value
        elif isinstance(node, StringLiteral):
            return node.value
        return None
