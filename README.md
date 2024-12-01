# TypeScript Interpreter in Rust (ts-interpreter-rs)

A TypeScript interpreter written in Rust that can execute TypeScript code without compilation. This project is designed for learning and experimentation with both Rust and TypeScript.

## Features (Planned)

- [ ] Basic TypeScript syntax parsing
- [ ] Variable declarations and assignments
- [ ] Basic arithmetic operations
- [ ] Function declarations and calls
- [ ] Control flow (if/else)
- [ ] Basic type checking
- [ ] REPL (Read-Eval-Print Loop)

## Getting Started

### Prerequisites

- Rust toolchain (install via [rustup](https://rustup.rs/))
- Cargo (comes with Rust)

### Installation

```bash
# Clone the repository
git clone https://github.com/a1j9o94/ts-interpreter-rs.git
cd ts-interpreter-rs

# Build the project
cargo build

# Run the REPL
cargo run
```

### Usage

```bash
# Start the REPL
cargo run

# Execute a TypeScript file
cargo run -- path/to/your/file.ts

# Run with debug output
cargo run -- -d path/to/your/file.ts
```

## Project Structure

- `src/main.rs`: Entry point and REPL implementation
- `src/lexer.rs`: Tokenization of TypeScript code
- `src/parser.rs`: Parsing tokens into an AST
- `src/interpreter.rs`: Executing the parsed AST
- `src/error.rs`: Error handling utilities

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.