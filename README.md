# D-Compiler

A simple compiler project written in Rust, focusing on building compiler components from the ground up.

## Project Status

This project is currently in the early stages of development. The primary focus so far has been on implementing a lexical analyzer (lexer) for a C-like language.

## Features

- **Lexical Analysis:** A hand-written lexer capable of recognizing:
    - Identifiers
    - Numbers (integers and floating-point)
    - Strings (with basic escape sequences)
    - Reserved keywords
    - Operators (arithmetic, comparison, assignment, logical)
    - Punctuation
    - Comments (single-line `//` and multi-line `/* ... */`)
    - Handles whitespace and newlines.

## Technologies Used

- **Rust:** The primary programming language for the compiler implementation.
- **Cargo:** Rust's package manager and build system.
- **Regex crate:** Used for regular expression matching (although the current lexer implementation seems hand-rolled, the dependency is present).

## Getting Started

### Prerequisites

- Rust and Cargo installed on your system. You can install them from [rustup.rs](https://rustup.rs/).

### Building the Project

Navigate to the project's root directory (`D-Compiler`) in your terminal and run:

```bash
cargo build
```

This will compile the project and its dependencies.

### Running the Lexer

The main program currently demonstrates the lexer by tokenizing a sample C-like code snippet. To run it, use:

```bash
cargo run
```

### Running Tests

The project includes unit tests for the lexer. To run them, use:

```bash
cargo test
```

## Project Plan

Based on the `plan.txt` file, the initial step for the project is to:

1.  Establish a list of keywords, reserved words, operations, and punctuations (Already largely implemented in the lexer).

Following the initial step, the project plan includes these further stages:

2.  **Parsing:** Developing a parser to build an Abstract Syntax Tree (AST). This is planned for a later stage of development.
3.  **Semantic Analysis:** Implementing checks for type correctness, variable scope, etc.
4.  **Intermediate Code Generation:** Translating the AST into an intermediate representation.
5.  **Code Optimization:** Applying optimizations to the intermediate code.
6.  **Backend Code Generation:** Generating target machine code or assembly from the optimized intermediate code.

Further details on the initial planning can be found in `plan.txt`.

## License

This project is licensed under the Apache License, Version 2.0. See the [LICENSE](D-Compiler/LICENSE) file for details.
