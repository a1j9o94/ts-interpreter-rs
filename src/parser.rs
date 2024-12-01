//! Parser for TypeScript code

use crate::lexer::Token;

#[derive(Debug)]
pub enum Expression {
    Number(f64),
    String(String),
    Identifier(String),
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
}

#[derive(Debug)]
pub enum Statement {
    Let {
        name: String,
        value: Expression,
    },
    Expression(Expression),
}

pub struct Parser {
    // TODO: Implement parser
}