# Aura Language Specification & Master History Blueprint v0.4.0

## 1. Core Philosophy & Mandate
Aura is a statically compiled systems language engineered to eliminate the historical trade-offs between human readability, bare-metal hardware performance, and cross-platform deployment transparency. Its architecture is governed by a singular mandate:
*"Write clean like Python, execute safe like Rust, deploy universal like JavaScript."*

### Key Tenets:
- **Zero-Boilerplate Syntax:** Inferred typing, minimal boilerplate layouts, and context-aware keywords.
- **Unified Backend Pipeline:** Emits optimized native machine code via LLVM or cross-platform web bytecode modules via WebAssembly targets.
- **Compile-Time Memory Safety:** Absolute structural memory safety enforced via compile-time static semantic analysis, banishing runtime Garbage Collection (GC) pauses entirely.

---

## 2. Syntax & Grammar Matrix
### A. Variables & Core Allocations
- `let <identifier> [ : <type> ] = <expression>` (Mutable stack tracking allocation)
- `const <identifier> [ : <type> ] = <expression>` (Immutable protection lock allocation)
- `<identifier> = <expression>` (Variable assignment mutation expression)

### B. Mathematical Operators & Precedence
- Binary arithmetic: `+`, `-`, `*`, `/`
- Binary relational comparisons: `<`, `>`, `==`
- **Pratt Precedence Hierarchy (Highest to Lowest):**
  1. `CALL` (`()`)
  2. `PRODUCT` (`*`, `/`)
  3. `SUM` (`+`, `-`)
  4. `COMPARISON` (`<`, `>`, `==`)
  5. `LOWEST`

### C. Functions & Control Flow
- Block Scopes: Defined cleanly by curly brackets `{ <statement>* }`
- Conditionals: `if <expression> <block> [ else <block> ]`
- Routines: `fn <name>(<param>: <type>, ...) [ -> <return_type> ] <block>`
- Returns: `return <expression>` (Passes computed register values back up the stack)

---

## 3. Structural Repository Layout
```text
aura-lang/
├── Cargo.toml               # Cargo package manager configuration
├── .gitignore               # Safely bypasses local binaries and intermediate caches
├── architecture.md          # This file (Single Source of Truth)
├── install.sh               # Cross-platform POSIX automated shell installer
└── src/
    ├── main.rs              # File CLI driver, Interactive REPL shell, and test harness
    ├── tokens.rs            # Token dictionary and keyword mappings
    ├── lexer.rs             # Safe character lookahead tokenizer scanner loops
    ├── ast.rs               # Boxed heap pointer abstract syntax node configurations
    ├── parser.rs            # Advanced top-down Pratt operator parsing engine
    ├── codegen.rs           # Multi-target, value-loading LLVM IR emitter module
    ├── semantic.rs          # Stack scope boundary and linear ownership sentinels
    └── apm.rs               # New: Built-in Package Manager dependency toolchain engine
```

---

## 4. Production Compiler Pipeline Architecture
┌──► [Native Target] ──► [Clang Linker] ➔ Native Executable
[Source Ingestion] ➔ [Lexer] ➔ [Parser] ➔ [Semantic Analyzer] ➔ [Code Generator Wrapper]
▲                                                         └──► [Web Target] ─────► [Wasm Linker]  ➔ Wasm Bytecode Module
│
(File Path OR Interactive REPL Shell)
1. **Flexible Source Ingestion:** Consumes code through physical `.au` scripts via standard file paths OR streams live user input strings line-by-line via an interactive REPL shell environment.
2. **Lexical Analysis (`lexer.rs`):** Streamlines linear characters into discrete tokens (`Token`), filtering comments (`//`) and whitespace.
3. **Syntactic Parsing (`parser.rs`):** Transforms tokens into an Abstract Syntax Tree (AST), using top-down Pratt parsing to handle priorities.
4. **Static Semantic Validation (`semantic.rs`):** Scans the AST before code generation to enforce scope lifecycles, verify `const` variables immutability, and track linear resource ownership states.
5. **Target-Aware Code Generation (`codegen.rs`):** Walks the verified AST to emit intermediate representation text (`.ll`). Adapts to target flags by stripping desktop C dependencies (like `printf`) and swapping target descriptors when outputting WebAssembly.
6. **Background Automation Assembler:** Automatically invokes `std::process::Command` to trigger the backend linker (`clang` or `wasm-ld`), outputting finished executable binaries or `.wasm` modules directly.

---

## 5. Completed Historical Milestones
- [✓] **Era 1 Prototyped (v0.1.0):** Tree-walk interpreter validated in Python, proving grammar parameters.
- [✓] **Era 2 Complete (v0.2.0):** Migrated entire frontend pipeline to Rust on Ubuntu, achieving 0.00s execution metrics. Added Pratt priority parsing, functional scopes, and linked external C library symbols (`printf`) to emit native binaries.
- [✓] **Cross-Functional Calling Complete:** Enabled complex multi-routine calling logic via LLVM `call` pointers.
- [✓] **Assembler Toolchain Automated:** Integrated process spawning via `std::process::Command` to handle compilation automatically.
- [✓] **Interactive REPL Engine Realized:** Implemented a live terminal environment prompt shell (`aura >>`) for instant expression evaluations.
- [✓] **Era 3 Complete (v0.3.0):** Programmed static semantic analyzer sentinels to block out-of-scope leakages, protect constants from mutate overrides, and track linear variable allocations in a **Use-After-Move** tracking map.
- [✓] **Era 4 Universal Targets Kickoff (v0.4.0):** Implemented target routing flags (`--target=wasm`) to dynamically switch target triples and compile syntax trees directly into browser-ready WebAssembly bytecode (`aura_module.wasm`).
- [✓] **Universal Web Targeting Functional:** Verified successful compilation of sandboxed WebAssembly bytecode modules (`aura_module.wasm`) executing cleanly inside local browser runtimes.
- [✓] **Global Toolchain Distribution Enabled:** Engineered an automated system installation shell script to link the Aura compiler executable directly into universal environment paths.
- [✓] **Ecosystem Package Toolchain Setup Kickoff (v0.5.0):** Programmed the core architectural structure for the built-in Aura Package Manager (`apm`).



---

## 6. Active Era 4 Implementation Goals
- Build an execution bridge to link compiled `.wasm` modules into active web runtimes.