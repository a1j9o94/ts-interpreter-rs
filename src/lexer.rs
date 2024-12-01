//! Lexical analysis for TypeScript code

#[derive(Debug, PartialEq)]
pub enum Token {
    // Keywords
    Let,
    Const,
    Function,
    Return,
    If,
    Else,
    
    // Literals
    Number(f64),
    String(String),
    Identifier(String),
    
    // Symbols
    Equal,
    Plus,
    Minus,
    Star,
    Slash,
    
    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        if self.position >= self.input.len() {
            return Token::EOF;
        }

        // TODO: Implement actual lexing
        Token::EOF
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
    }
}