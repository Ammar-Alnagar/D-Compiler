# D-Compiler 🚀

![Compiler Pipeline]([https://en.wikipedia.org/wiki/Compiler](https://upload.wikimedia.org/wikipedia/commons/thumb/6/6b/Compiler.svg/800px-Compiler.svg.png))

## 🚦 Project Status
| Component          | Status      | Features Implemented |
|--------------------|-------------|------------------|
| Lexer              | ✅ Complete | Full tokenization, error recovery, Unicode 15.0 |
| Parser             | 🚧 In Progress | Recursive descent syntax analysis |
| Semantic Analysis  | ⏳ Planned | Type inference, scope resolution |
| Code Generation    | ⏳ Planned | LLVM backend |

## 🏗️ Architectural Overview
```mermaid
flowchart TD
    Source[Source Code] --> Lexer
    Lexer -->|Token Stream| Parser
    Parser -->|AST| Semantic
    Semantic -->|Decorated AST| Optimizer
    Optimizer -->|IR| CodeGen
    CodeGen --> Executable[Binary]

    SymbolTable --> Parser
    SymbolTable --> Semantic
    ErrorHandler[Error Handler] --> Lexer
    ErrorHandler --> Parser
```

## 📂 Project Structure

```
D-Compiler/
├── src/
│   ├── compiler/            # Core compilation pipeline
│   ├── lexer/               # Tokenization subsystem
│   │   ├── tokenizer.rs      # Character scanning
│   │   ├── token_stream.rs   # Buffered token streaming
│   │   └── error.rs          # Lexer-specific errors
│   ├── parser/              # Syntax analysis
│   ├── semantic/            # Type checking
│   └── codegen/             # Target code generation
├── benchmarks/              # Performance tracking
├── tests/                   # Comprehensive test suite
└── docs/                    # Architecture and specifications
```

## 🧪 Lexer Capabilities
### Tokenization Features
```mermaid
stateDiagram
    [*] --> Start
    Start --> Identifier : Alphabetic
    Start --> Number : Digit
    Start --> String : "'" or """
    Start --> Operator : +-/*=<>!
    Start --> Punctuation : (){}[],;

    Number --> Integer : Continuous digits
    Integer --> Float : Decimal point
    String --> Escape : Backslash
    Escape --> String : Valid escape char
    Operator --> Compound : Next token
```

### Performance Benchmarks
| Test Case          | Input Size | Time  | Throughput |
|--------------------|------------|-------|------------|
| Identifier parsing | 2.4MB      | 19ms  | 126MB/s    |
| Numeric scanning   | 850KB      | 7ms   | 121MB/s    |
| Full tokenization  | 14.7MB     | 128ms | 114MB/s    |

## 🧠 Advanced Parsing Examples
### Source Input
```c
struct Vector<T> {
    x: T,
    y: T,

    fn length(&self) -> f64 {
        sqrt(self.x*self.x + self.y*self.y)
    }
}
```

### Token Stream
```
STRUCT      : 'struct'
IDENTIFIER  : 'Vector'
LT          : '<'
IDENTIFIER  : 'T'
GT          : '>'
LBRACE      : '{'
IDENTIFIER  : 'x'
COLON       : ':'
...
```

## 🚀 Building the Compiler
### Prerequisites
- Rust 1.78+
- LLVM 18+ (future backend)
- CMake 3.20+

### Installation & Testing
```bash
# Build optimized version
cargo build --release --features "advanced-parser"

# Run test suite
cargo test --all-features

# Benchmark lexer performance
cargo bench --bench lexer-benchmarks

# Generate documentation
cargo doc --open
```

## 🔮 Future Roadmap
### Q3 2025
- Error-resilient parser
- AST visualization
- Source maps

### Q4 2025
- Type inference engine
- Control flow analysis
- Borrow checker

### 2026
- LLVM backend
- JIT compilation
- Language Server Protocol

## 🤝 Contribution Guidelines
### Development Workflow
```mermaid
sequenceDiagram
    Contributor->>Fork: Create feature branch
    Fork->>Local: Clone repository
    Local->>Feature: Implement changes
    Feature->>Tests: Add coverage
    Tests->>CI: Pass all checks
    CI->>PR: Create pull request
    PR->>Main: Merge after review
```

### Quality Standards
1. 95%+ test coverage
2. Benchmarks for performance changes
3. Documentation updates
4. Clippy lint standards
5. Semantic versioning compliance

---
**Happy Compiling!** 🔧
_The D-Compiler Team_
```
