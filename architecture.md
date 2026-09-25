# Aura Language Specification v0.2.0 (Era 2: Native Speed)

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

## 3. Production Rust Compiler Architecture Pipeline
1. **Source Code (`.au`)** -> Read by Rust's file system interface.
2. **Lexer (`lexer.rs`)** -> Converts text into a stream of robust Rust Token structs.
3. **Parser (`parser.rs`)** -> Builds an Abstract Syntax Tree (AST) using safe pattern matching.
4. **LLVM IR Code Generator** -> Translates the AST directly into LLVM Intermediate Representation, bypassing slow runtimes for extreme CPU execution speed.

## 4. Historical Milestones
- [✓] **Phase 1 Complete:** Tree-walk interpreter successfully prototyped and validated in Python (v0.1.0). Proven capability to scan, parse, and enforce variable immutability constraints.

## 5. Active Era 2 Implementation Goals
- Initialize Rust project workspace via Cargo.
- Implement token structures using strongly typed Rust `enum` variants.
- Ports the verified Phase 1 Lexer over to native Rust structures.
