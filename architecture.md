# Aura Language Specification v0.2.0 (Era 2: Native Speed)

## 1. Core Philosophy
- **Human-First Syntax:** Minimal boilerplate, clean keywords, optional explicit typing.
- **Unified Compilation:** Targets Native Machine Code (via LLVM) and WebAssembly (Wasm).
- **Automated Ownership:** Compile-time memory management without a runtime garbage collector.

## 2. Syntax & Grammar Rules
### Variables
- `let <identifier> [ : <type> ] = <expression>` (Mutable)
- `const <identifier> [ : <type> ] = <expression>` (Immutable)

### Arithmetic Operations
- Binary expressions: `<expression> ( + | - | * | / ) <expression>`
- Operator Precedence: `*` and `/` carry higher priority weights than `+` and `-`.

### Functions & Blocks
- Block Statement: `{ <statement>* }`
- Function Declaration: `fn <name>(<param>: <type>, ...) [ -> <return_type> ] <block>`

### Primitive Types
- `Int` (maps to LLVM `i64`), `Float` (maps to LLVM `double`), `String`, `Bool`, `Void`

## 3. Production Rust Compiler Architecture Pipeline
1. **Source Code (`.au`)** -> Read by Rust's file system interface.
2. **Lexer (`lexer.rs`)** -> Converts text into a stream of robust Rust Token structs. [Completed]
3. **Parser (`parser.rs`)** -> Builds an Abstract Syntax Tree (AST) using safe pattern matching. [Completed]
4. **Code Generator (`codegen.rs`)** -> (Active) Translates the AST directly into LLVM Intermediate Representation text strings, bypassing slow runtimes for extreme CPU execution speed.

## 4. Historical Milestones
- [✓] **Phase 1 Complete:** Tree-walk interpreter successfully prototyped and validated in Python (v0.1.0).
- [✓] **Era 2 Baseline Complete:** Initialized Cargo workspace and validated native Rust token extraction.
- [✓] **Math Pipeline Complete:** Engineered Pratt Parser token weight structures to resolve mathematical operator precedence.
- [✓] **Function Syntax Complete:** Mapped procedural function structures and block scopes cleanly in systems code.

## 5. Active Era 2 Implementation Goals
- Create a dedicated code generation asset module (`src/codegen.rs`).
- Map AST Program nodes into raw LLVM text output wrappers ready for compilation.
