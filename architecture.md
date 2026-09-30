# Aura Language Specification v0.2.0 (Era 2: Native Speed)

## 1. Core Philosophy
- **Human-First Syntax:** Minimal boilerplate, clean keywords, optional explicit typing.
- **Unified Compilation:** Targets Native Machine Code (via LLVM) and WebAssembly (Wasm).
- **Automated Ownership:** Compile-time memory management without a runtime garbage collector.

## 2. Syntax & Grammar Rules
### Variables & Arithmetic
- `let <identifier> [ : <type> ] = <expression>`
- Binary arithmetic operations: `+`, `-`, `*`, `/` with operator precedence weights.

### Conditionals & Logic (Updated)
- Boolean Literals: `true` | `false`
- Relational Comparisons: `<` | `>` | `==`
- If Expression: `if <expression> <block> [ else <block> ]`

## 3. Production Rust Compiler Architecture Pipeline
1. **Source Code (`.au`)** -> Read via standard Linux POSIX file streams.
2. **Lexer (`lexer.rs`)** -> Streamlines characters into Token structures. [Completed]
3. **Parser (`parser.rs`)** -> Builds an Abstract Syntax Tree (AST). [Completed]
4. **Code Generator (`codegen.rs`)** -> Emits LLVM IR assembly with custom basic block branching allocations. [Completed Branch Baseline]

## 4. Historical Milestones
- [✓] **Phase 1 Complete:** Tree-walk interpreter validated in Python (v0.1.0).
- [✓] **Era 2 Frontend Complete:** Migrated pipeline to Rust on Ubuntu. Confirmed 0.00s mathematical and functional tree-building validation.
- [✓] **LLVM Control Flow Complete:** Verified generation of raw conditional jumps (`br i1`) and isolated structural basic block segments (`then:`, `else:`, `merge:`).

## 5. Active Era 2 Implementation Goals
- Add comparison operators (`<`, `>`, `==`) to `src/tokens.rs` and `src/lexer.rs`.
- Integrate comparison tokens into our Pratt Parser operator priority tiers inside `src/parser.rs`.
