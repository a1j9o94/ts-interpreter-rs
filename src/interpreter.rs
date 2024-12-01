//! Interpreter for TypeScript code

use std::collections::HashMap;
use crate::parser::{Expression, Statement};
use crate::error::Result;

pub struct Interpreter {
    variables: HashMap<String, Value>,
}

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn eval(&mut self, stmt: Statement) -> Result<Value> {
        // TODO: Implement evaluation
        Ok(Value::Null)
    }
}