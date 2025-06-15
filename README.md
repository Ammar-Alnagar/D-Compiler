
# D-Compiler 🚀

A Rust-based compiler for a C-like language, implemented from scratch. This project follows a multi-phase compilation process, starting with lexical analysis and progressing to parsing, semantic analysis, and code generation.

## Project Status 📌

| Stage       | Status      | Description                                  |
|-------------|-------------|----------------------------------------------|
| Lexer       | ✅ Complete | Fully functional with extensive test coverage. |
| Parser      | ⏳ Planned  | Next major milestone.                        |
| Semantic    | ❌ Pending  | Type checking, scope resolution, etc.        |
| Codegen     | ❌ Pending  | Target: LLVM IR or direct machine code.      |

## Features 🌟

### Implemented (Lexer)
| Feature               | Example            | Notes                              |
|-----------------------|--------------------|------------------------------------|
| Identifiers           | `variable_name`    | Supports underscores and alphanums.|
| Numbers               | `42`, `3.14`       | Integers and floats.               |
| Strings               | `"hello\nworld"`   | Escape sequences (`\n`, `\t`).     |
| Operators             | `+`, `==`, `!=`    | Multi-character support.           |
| Comments              | `// line`, `/* */` | Single-line and multi-line.        |
| Punctuation           | `( ) { } [ ] ; :`  | Full punctuation support.          |

### Planned
- **Parser**: Recursive descent for expressions/statements.
- **Semantic Checks**: Type inference, scope resolution.
- **Optimizations**: Constant folding, dead code elimination.

## Technologies Used 🛠️

- **Rust**: Leveraging Rust's performance and safety for compiler implementation.
- **Cargo**: Rust's build system and package manager.
- **Regex Crate**: For pattern matching (with hand-written fallback).

## Project Structure 📂


D-Compiler/
├── src/
│   ├── main.rs               → Entry point (demo lexer)
│   ├── lexer/
│   │   ├── mod.rs            → Lexer module exports
│   │   ├── lexer.rs          → Core lexer implementation
│   │   ├── token.rs          → Token definitions
│   │   └── tests.rs          → Unit tests
│   └── tests/                → Integration tests
├── Cargo.toml                → Project manifest
├── Cargo.lock                → Dependency resolution
└── plan.txt                  → Roadmap


## Lexer Workflow 🔄

mermaid
graph LR
  A[Source Code] --> B((Lexer))
  B --> C[Token Stream]
  C --> D{{Parser}} --> E[AST]
  E --> F[Semantic Analysis] --> G[Code Generation]


## Getting Started 🏁

### Prerequisites
- Rust 1.70+ ([install via rustup](https://rustup.rs/))

### Commands
bash
# Build and run
cargo run

# Run tests
cargo test

# Generate docs
cargo doc --open


## Example Output 📝
rust
Input: "let x = 42 + (y * 3);"
Tokens:
  Line 1 Col 1  | Reserved(let)  | "let"
  Line 1 Col 5  | Identifier(x)  | "x"
  Line 1 Col 7  | Operation(=)   | "="
  Line 1 Col 9  | Number(42)     | "42"
  Line 1 Col 12 | Operation(+)   | "+"
  Line 1 Col 14 | Punctuation(() | "("
  Line 1 Col 15 | Identifier(y)  | "y"
  Line 1 Col 17 | Operation(*)   | "*"
  Line 1 Col 19 | Number(3)      | "3"
  Line 1 Col 20 | Punctuation()) | ")"
  Line 1 Col 21 | Punctuation(;) | ";"


## License 📜
Apache 2.0 - See [LICENSE](LICENSE)

## Contributing 🤝
1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Submit a PR with clear documentation

---
**Happy Compiling!** 🔧
