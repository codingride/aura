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

# Aura Language Specification & Compiler Blueprint v0.3.5

## 1. Core Philosophy & Mandate
Aura is a statically compiled systems language engineered to completely eliminate the historical trade-offs between human readability, bare-metal hardware speed, and cross-platform deployment. Its architecture is governed by a singular guiding rule: 
*"Write clean like Python, execute safe like Rust, deploy universal like JavaScript."*

### Key Tenets:
- **Zero-Boilerplate Syntax:** Inferred typing, clean layouts, and context-aware keywords.
- **Bare-Metal Compilation:** Compiles directly into native machine code utilizing LLVM infrastructure.
- **Compile-Time Memory Enforcement:** Absolute safety handled via static semantic analysis, avoiding runtime Garbage Collection (GC) pauses entirely.

---

## 2. Syntax & Grammar Matrix
### A. Variables & Core Allocations
- `let <identifier> [ : <type> ] = <expression>` (Mutable allocation)
- `const <identifier> [ : <type> ] = <expression>` (Immutable protection lock)
- `<identifier> = <expression>` (Standalone variable assignment mutation)

### B. Mathematical Operators & Precedence
- Binary arithmetic: `+`, `-`, `*`, `/`
- Binary relational comparisons: `<`, `>`, `==`
- **Pratt Precedence Hierarchy (Highest to Lowest):**
  1. `PRODUCT` (`*`, `/`)
  2. `SUM` (`+`, `-`)
  3. `COMPARISON` (`<`, `>`, `==`)
  4. `LOWEST`

### C. Functions & Control Flow
- Block Scopes: Defined cleanly by curly brackets `{ <statement>* }`
- Conditionals: `if <expression> <block> [ else <block> ]`
- Routines: `fn <name>(<param>: <type>, ...) [ -> <return_type> ] <block>`

---

## 3. Structural Repository Layout
```text
aura-lang/
├── Cargo.toml               # Cargo package manager configuration
├── .gitignore               # Safely bypasses local /target caches
├── architecture.md          # This file (Single Source of Truth)
└── src/
    ├── main.rs              # Unified testing and entry framework
    ├── tokens.rs            # Token dictionary and keyword mappings
    ├── lexer.rs             # Safe char-by-char source code scanner
    ├── ast.rs               # Abstract Syntax Tree pointer models
    ├── parser.rs            # Top-Down Pratt operator parser
    └── codegen.rs           # Multi-block LLVM IR code generator
```

---

## 4. Production Compiler Pipeline Architecture
[Source File (.au)] ➔ [Lexer] ➔ [Parser] ➔ [Semantic Analyzer] ➔ [Code Generator] ➔ [Clang Compiler] ➔ [Native Executable]
1. **POSIX File Ingestion:** Source streams are consumed natively within the Linux workspace environment.
2. **Lexical Analysis (`lexer.rs`):** Converts linear strings into unique data packages (`Token`), tracking line and column metrics while dynamically filtering comments (`//`) and whitespace.
3. **Syntactic Parsing (`parser.rs`):** Transforms the token sequence into an Abstract Syntax Tree (AST), handling expression trees using top-down precedence rules.
4. **Static Semantic Validation (`semantic.rs`):** Analyzes the AST before compilation to enforce lifecycle boundaries, catch out-of-scope leakages, and block illegal modifications to `const` variables.
5. **Intermediate Representation Generation (`codegen.rs`):** Walks the verified AST to emit clean, unoptimized LLVM IR assembly text files (`.ll`).
6. **Machine Assembler (Clang Toolchain):** Compiles, links, and packages the generated `.ll` files directly into standalone native machine binary programs (`aura_program`).

---

## 5. Completed Historical Milestones
- [✓] **Phase 1 Complete (v0.1.0):** Tree-walk interpreter validated in Python, proving core variable scope mechanics and grammar parameters.
- [✓] **Era 2 Frontend Upgraded (v0.2.0):** Ported the scanning loop and token matrix to Rust, achieving 0.00s frontend validation speeds.
- [✓] **Pratt Mathematical Engine Active:** Configured token hierarchy weights to natively resolve complex arithmetic operations.
- [✓] **LLVM Basic Blocks Integrated:** Implemented control flow branching to generate conditional jumps (`br i1`) and basic blocks (`then:`, `else:`, `merge:`).
- [✓] **Pointer Dereferencing Fixed:** Solved type mismatch bounds by implementing structural memory `load` routines inside the variable stream.
- [✓] **Multi-Block Procedural Functions Realized:** Enabled the generation of isolated LLVM function scopes (`define`) alongside the primary `@main` loop.
- [✓] **Native Screen Output Achieved:** Bound external C `printf` declarations to print physical registers directly into the terminal screen.
- [✓] **Static Sentinels Active (v0.3.0):** Programmed a compile-time Symbol Table to block memory leaks and protect constants from mutate reassignments.
- [✓] **Cross-Functional Calling Complete:** Enabled the compilation of explicit multi-routine execution loops utilizing low-level LLVM `call` pointers.

---

## 6. Active Era 2/3 Integration Goals
- Expand the Expression enum inside `src/ast.rs` to support Function Call expressions.
- Update `src/parser.rs` to read parenthesized function arguments separated by commas.
- Extend `src/codegen.rs` to generate low-level LLVM `call` assembly blocks with matching parameter structures.
- Overwrite the empty root `main()` driver inside `src/main.rs` to process command-line file arguments natively via the standard Linux filesystem.
