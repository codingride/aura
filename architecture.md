# Aura Language Specification v0.1.0

## 1. Core Philosophy
- **Human-First Syntax:** Minimal boilerplate, clean keywords, optional explicit typing.
- **Unified Compilation:** Targets Native Machine Code (via LLVM) and WebAssembly (Wasm).
- **Automated Ownership:** Compile-time memory management without a runtime garbage collector.

## 2. Syntax & Grammar Rules
### Variables
- `let <identifier> [ : <type> ] = <expression>` (Mutable)
- `const <identifier> [ : <type> ] = <expression>` (Immutable)

### Primitive Types
- `Int`, `Float`, `String`, `Bool`, `Void`

### Functions
- Syntax: `fn <name>(<param>: <type>) [ -> <return_type> ] { <body> }`

## 3. Compiler Architecture Pipeline
1. **Source Code (`.aura`)** -> Text input stream.
2. **Lexer (`lexer.py`)** -> Converts text into a stream of structured Tokens.
3. **Parser (`parser.py`)** -> Converts Tokens into an Abstract Syntax Tree (AST).
4. **Semantic Analyzer** -> Infers types, checks ownership rules, ensures safety.
5. **Code Generator** -> Emits LLVM IR / WebAssembly.

## 4. Active Token Roadmap (Phase 1)
- Keywords: `let`, `const`, `fn`, `return`, `if`, `else`
- Literals: `IDENTIFIER`, `INT_LIT`, `FLOAT_LIT`, `STRING_LIT`
- Operators: `+`, `-`, `*`, `/`, `=`, `->`
- Delimiters: `(`, `)`, `{`, `}`, `,`, `:`

## 5. Completed Structural Components
- **[Week 1 Completed]** `src/tokens.py`: Dictionary mappings and data class token validation tracking.
- **[Week 1 Completed]** `src/lexer.py`: Character consumer, string, number and multi-token (`->`) look-ahead processor.
- **[Week 2 Active]** `src/ast.py`: Implementation of statements, variable definitions, and primitive literals.
