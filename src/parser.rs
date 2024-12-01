//! Parser for TypeScript code

use crate::lexer::{Lexer, Token};

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let {
        name: String,
        value: Expression,
    },
    Expression(Expression),
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        Self {
            lexer,
            current_token,
        }
    }

    fn next_token(&mut self) -> Token {
        let token = self.lexer.next_token();
        std::mem::replace(&mut self.current_token, token)
    }

    pub fn parse_statement(&mut self) -> Result<Statement, String> {
        println!("Parsing statement, current token: {:?}", self.current_token);
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            _ => {
                println!("Attempting to parse as expression statement");
                Ok(Statement::Expression(self.parse_expression()?))
            }
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, String> {
        self.next_token(); // consume 'let'

        let name = match &self.current_token {
            Token::Identifier(name) => name.clone(),
            _ => return Err("Expected identifier after 'let'".to_string()),
        };

        self.next_token(); // consume identifier

        if self.current_token != Token::Equal {
            return Err("Expected '=' after identifier in let statement".to_string());
        }

        self.next_token(); // consume '='

        let value = self.parse_expression()?;

        if self.current_token == Token::Semicolon {
            self.next_token(); // consume semicolon
        }

        Ok(Statement::Let { name, value })
    }

    pub fn parse_expression(&mut self) -> Result<Expression, String> {
        let left = self.parse_primary()?;

        if self.is_operator(&self.current_token) {
            let operator = self.current_token.clone();
            self.next_token(); // consume operator
            let right = self.parse_primary()?;
            
            Ok(Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            })
        } else {
            Ok(left)
        }
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        println!("Parsing primary expression, current token: {:?}", self.current_token);
        let expr = match &self.current_token {
            Token::Number(n) => Expression::Number(*n),
            Token::String(s) => Expression::String(s.clone()),
            Token::Identifier(name) => Expression::Identifier(name.clone()),
            token => {
                println!("Unexpected token in primary expression: {:?}", token);
                return Err(format!("Expected expression, got {:?}", token));
            }
        };

        self.next_token(); // consume the token
        Ok(expr)
    }

    fn is_operator(&self, token: &Token) -> bool {
        matches!(token, 
            Token::Plus | Token::Minus | Token::Star | Token::Slash
        )
    }

    pub fn is_eof(&self) -> bool {
        self.current_token == Token::EOF
    }
}