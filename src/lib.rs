//! TypeScript Interpreter in Rust
//! 
//! This library implements a TypeScript interpreter that can execute TypeScript code
//! without compilation. It supports a subset of TypeScript features and is designed
//! for learning and experimentation.

pub mod lexer;
pub mod parser;
pub mod interpreter;
pub mod error;

// Re-export main types
pub use error::Error;

/// Result type for the interpreter
pub type Result<T> = std::result::Result<T, Error>;