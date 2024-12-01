//! Interpreter for TypeScript code

use std::collections::HashMap;
use crate::parser::Statement;
use crate::Result;

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
        // print statement and variables
        println!("{:?}", stmt);
        println!("{:?}", self.variables);
        Ok(Value::Null)
    }
}