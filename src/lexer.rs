//! Lexical analysis for TypeScript code

#[derive(Debug, PartialEq, Clone)]
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
    LAngle,
    RAngle,
    Semicolon,
    Comma,
    
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
        self.skip_comments();
        
        if self.position >= self.input.len() {
            return Token::EOF;
        }

        let ch = self.input[self.position];
        
        // Handle single-character tokens
        let token = match ch {
            '"' => return self.read_string(),
            '=' => Token::Equal,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '<' => Token::LAngle,
            '>' => Token::RAngle,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            _ => {
                // Handle identifiers, keywords, and numbers
                if ch.is_alphabetic() || ch == '_' {
                    return self.read_identifier();
                } else if ch.is_numeric() {
                    return self.read_number();
                } else {
                    Token::EOF
                }
            }
        };

        self.position += 1;
        token
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len() && 
              (self.input[self.position].is_alphanumeric() || self.input[self.position] == '_') {
            self.position += 1;
        }
        
        let identifier: String = self.input[start..self.position].iter().collect();
        
        // Check for keywords
        match identifier.as_str() {
            "let" => Token::Let,
            "const" => Token::Const,
            "function" => Token::Function,
            "return" => Token::Return,
            "if" => Token::If,
            "else" => Token::Else,
            _ => Token::Identifier(identifier),
        }
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;
        let mut seen_dot = false;
        
        while self.position < self.input.len() {
            let ch = self.input[self.position];
            if ch.is_numeric() {
                self.position += 1;
            } else if ch == '.' && !seen_dot {
                seen_dot = true;
                self.position += 1;
            } else {
                break;
            }
        }
        
        let number_str: String = self.input[start..self.position].iter().collect();
        Token::Number(number_str.parse::<f64>().unwrap())
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
    }

    fn skip_comments(&mut self) {
        while self.position + 1 < self.input.len() {
            // Check for // comments
            if self.input[self.position] == '/' && self.input[self.position + 1] == '/' {
                // Skip to end of line
                while self.position < self.input.len() && self.input[self.position] != '\n' {
                    self.position += 1;
                }
                // Skip the newline if present
                if self.position < self.input.len() {
                    self.position += 1;
                }
                self.skip_whitespace();  // Handle any whitespace after comment
                continue;  // Check for another comment
            }
            break;  // No more comments found
        }
    }

    fn read_string(&mut self) -> Token {
        self.position += 1; // Skip opening quote
        let start = self.position;
        
        while self.position < self.input.len() && self.input[self.position] != '"' {
            self.position += 1;
        }
        
        let string = self.input[start..self.position].iter().collect();
        self.position += 1; // Skip closing quote
        Token::String(string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_lexer() {
        let input = "let x = 42;";
        let lexer = Lexer::new(input);
        assert_eq!(lexer.input.len(), input.chars().count());
        assert_eq!(lexer.position, 0);
    }

    #[test]
    fn test_skip_whitespace() {
        let mut lexer = Lexer::new("   \t\n  abc");
        lexer.skip_whitespace();
        assert_eq!(lexer.position, 7);
    }

    #[test]
    fn test_eof_on_empty_input() {
        let mut lexer = Lexer::new("");
        assert_eq!(lexer.next_token(), Token::EOF);
    }
}