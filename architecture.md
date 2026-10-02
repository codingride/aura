# Aura Language Specification v0.3.0 (Era 3: Memory Safety)

## 1. Core Philosophy
- **Human-First Syntax:** Minimal boilerplate, clean keywords, optional explicit typing.
- **Unified Compilation:** Targets Native Machine Code (via LLVM) and WebAssembly (Wasm).
- **Automated Ownership:** Compile-time memory lifecycle tracking without a runtime garbage collector.

## 2. Syntax & Grammar Rules
### Variables & Arithmetic
- `let <identifier> [ : <type> ] = <expression>` (Mutable stack tracking)
- `const <identifier> [ : <type> ] = <expression>` (Immutable protection tracking)

### Conditionals & Logic
- If Expression: `if <expression> <block> [ else <block> ]`
- Relational Comparisons: `<`, `>`, `==`

## 3. Production Rust Compiler Architecture Pipeline
1. **Source Code (`.au`)** -> Read via standard Linux POSIX file streams.
2. **Lexer (`lexer.rs`)** -> Streamlines characters into Token structures. [Completed]
3. **Parser (`parser.rs`)** -> Builds an Abstract Syntax Tree (AST). [Completed]
4. **Semantic Analyzer (`semantic.rs`)** -> Tracks variable lifecycles, scopes, and memory ownership rules before compilation. [Completed Scope Validation]
5. **Code Generator (`codegen.rs`)** -> Emits LLVM IR assembly with loaded pointers and branching blocks. [Completed Baseline]
6. **Machine Assembler (Clang)** -> Packages intermediate streams into native binary executables. [Completed Baseline]

## 4. Historical Milestones
- [✓] **Era 1 Prototyped:** Tree-walk interpreter validated in Python (v0.1.0).
- [✓] **Era 2 Frontend Upgraded:** Scaled tokenizer, Pratt Parser operator precedence, and functional structures to Rust.
- [✓] **LLVM Generation & Tooling Achieved:** Engineered custom pointer loading loops and basic block conditional branching.
- [✓] **Native Screen Output Realized:** Linked external C `printf` declarations to emit real hardware register outputs directly onto the terminal monitor stream.
- [✓] **Static Memory Sentinel Verified:** Successfully built a compile-time scope block tracker capable of blocking out-of-scope leakages.
- [✓] **Scope Lifecycle Sentinel Verified:** Blocked out-of-scope leakages at compile time.
- [✓] **Immutability Sentinel Verified:** Successfully verified symbol-table protection parameters intercepting unauthorized constant variable overwrites.

## 5. Active Era 3 Implementation Goals
- Integrate standalone assignment parsing syntax inside `src/ast.rs` and `src/parser.rs`.
