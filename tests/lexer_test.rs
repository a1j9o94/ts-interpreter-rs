use ts_interpreter_rs::lexer::*;
use pretty_assertions::assert_eq;

#[test]
fn test_basic_lexing() {
    let input = "let x = 42;";
    let mut lexer = Lexer::new(input);
    
    assert_eq!(lexer.next_token(), Token::Let);
    assert_eq!(lexer.next_token(), Token::Identifier("x".to_string()));
    assert_eq!(lexer.next_token(), Token::Equal);
    assert_eq!(lexer.next_token(), Token::Number(42.0));
    assert_eq!(lexer.next_token(), Token::Semicolon);
    assert_eq!(lexer.next_token(), Token::EOF);
}

#[test]
fn test_complex_expression() {
    let input = "function add(a<T>, b) { return a + b; }";
    let mut lexer = Lexer::new(input);
    
    let expected_tokens = vec![
        Token::Function,
        Token::Identifier("add".to_string()),
        Token::LParen,
        Token::Identifier("a".to_string()),
        Token::LAngle,
        Token::Identifier("T".to_string()),
        Token::RAngle,
        Token::Comma,
        Token::Identifier("b".to_string()),
        Token::RParen,
        Token::LBrace,
        Token::Return,
        Token::Identifier("a".to_string()),
        Token::Plus,
        Token::Identifier("b".to_string()),
        Token::Semicolon,
        Token::RBrace,
        Token::EOF,
    ];

    for expected in expected_tokens {
        assert_eq!(lexer.next_token(), expected);
    }
}

#[test]
fn test_float_numbers() {
    let input = "let pi = 3.14159;";
    let mut lexer = Lexer::new(input);
    
    assert_eq!(lexer.next_token(), Token::Let);
    assert_eq!(lexer.next_token(), Token::Identifier("pi".to_string()));
    assert_eq!(lexer.next_token(), Token::Equal);
    assert_eq!(lexer.next_token(), Token::Number(3.14159));
    assert_eq!(lexer.next_token(), Token::Semicolon);
    assert_eq!(lexer.next_token(), Token::EOF);
}

#[test]
fn test_skip_comments() {
    let input = "// this is a comment\nlet x = 42;";
    let mut lexer = Lexer::new(input);
    
    assert_eq!(lexer.next_token(), Token::Let);
    assert_eq!(lexer.next_token(), Token::Identifier("x".to_string()));
    assert_eq!(lexer.next_token(), Token::Equal);
    assert_eq!(lexer.next_token(), Token::Number(42.0));
    assert_eq!(lexer.next_token(), Token::Semicolon);
    assert_eq!(lexer.next_token(), Token::EOF);
}