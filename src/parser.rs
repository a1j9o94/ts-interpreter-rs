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
        self.parse_binary_expression(0) // Start with lowest precedence
    }

    fn get_operator_precedence(token: &Token) -> u8 {
        match token {
            Token::Plus | Token::Minus => 1,
            Token::Star | Token::Slash => 2,
            _ => 0,
        }
    }

    fn parse_binary_expression(&mut self, precedence: u8) -> Result<Expression, String> {
        let mut left = self.parse_atom()?;
        println!("Parsed left side: {:?}", left);

        while self.is_operator(&self.current_token) {
            let op_precedence = Self::get_operator_precedence(&self.current_token);
            println!("Current operator: {:?}, precedence: {}, min precedence: {}", 
                    self.current_token, op_precedence, precedence);
            
            if op_precedence <= precedence {
                break;
            }

            let operator = self.current_token.clone();
            self.next_token(); // consume operator

            let right = self.parse_binary_expression(op_precedence)?;
            println!("Parsed right side: {:?}", right);
            
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_atom(&mut self) -> Result<Expression, String> {
        match &self.current_token {
            Token::LParen => {
                self.next_token(); // consume '('
                let expr = self.parse_expression()?;
                
                if self.current_token != Token::RParen {
                    return Err("Expected ')'".to_string());
                }
                self.next_token(); // consume ')'
                Ok(expr)
            }
            _ => self.parse_primary()
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