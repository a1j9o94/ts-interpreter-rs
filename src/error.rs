//! Error types for the interpreter

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Syntax error: {0}")]
    Syntax(String),
    
    #[error("Type error: {0}")]
    Type(String),
    
    #[error("Runtime error: {0}")]
    Runtime(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}