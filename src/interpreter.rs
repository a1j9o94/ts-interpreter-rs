//! Interpreter for TypeScript code

use std::collections::HashMap;
use crate::parser::{Statement, Expression};
use crate::lexer::Token;

pub struct Interpreter {
    variables: HashMap<String, Value>,
    last_value: Option<Value>,
}

#[derive(Clone, Debug, PartialEq)]
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
            last_value: None,
        }
    }

    pub fn eval(&mut self, stmt: Statement) -> std::result::Result<Value, String> {
        match stmt {
            Statement::Let { name, value } => {
                let val = self.eval_expression(value)?;
                self.variables.insert(name, val.clone());
                self.last_value = Some(val.clone());
                Ok(val)
            },
            Statement::Expression(expr) => {
                let val = self.eval_expression(expr)?;
                self.last_value = Some(val.clone());
                Ok(val)
            },
        }
    }

    fn eval_expression(&self, expr: Expression) -> std::result::Result<Value, String> {
        match expr {
            Expression::Number(n) => Ok(Value::Number(n)),
            Expression::String(s) => Ok(Value::String(s)),
            Expression::Identifier(name) => {
                self.variables.get(&name)
                    .cloned()
                    .ok_or_else(|| format!("Undefined variable: {}", name))
            },
            Expression::Binary { left, operator, right } => {
                let lhs = self.eval_expression(*left)?;
                let rhs = self.eval_expression(*right)?;
                self.eval_binary_op(lhs, operator, rhs)
            }
        }
    }

    fn eval_binary_op(&self, left: Value, op: Token, right: Value) -> std::result::Result<Value, String> {
        match (left, op, right) {
            (Value::Number(l), Token::Plus, Value::Number(r)) => Ok(Value::Number(l + r)),
            (Value::Number(l), Token::Minus, Value::Number(r)) => Ok(Value::Number(l - r)),
            (Value::Number(l), Token::Star, Value::Number(r)) => Ok(Value::Number(l * r)),
            (Value::Number(l), Token::Slash, Value::Number(r)) => Ok(Value::Number(l / r)),
            (Value::String(l), Token::Plus, Value::String(r)) => Ok(Value::String(l + &r)),
            _ => Err(String::from("Invalid operation")),
        }
    }

    pub fn get_last_value(&self) -> Option<&Value> {
        self.last_value.as_ref()
    }

    pub fn get_variables(&self) -> &HashMap<String, Value> {
        &self.variables
    }
}