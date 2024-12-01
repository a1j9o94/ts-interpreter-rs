use ts_interpreter_rs::parser::*;
use ts_interpreter_rs::lexer::*;
use pretty_assertions::assert_eq;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_parse_number() {
    let input = "42";
    let mut parser = Parser::new(input);
    let expr = parser.parse_expression().unwrap();
    assert_eq!(expr, Expression::Number(42.0));
}

#[test]
fn test_parse_identifier() {
    let input = "x";
    let mut parser = Parser::new(input);
    let expr = parser.parse_expression().unwrap();
    assert_eq!(expr, Expression::Identifier("x".to_string()));
}

#[test]
fn test_parse_let_statement() {
    let input = "let x = 42;";
    let mut parser = Parser::new(input);
    let stmt = parser.parse_statement().unwrap();
    assert_eq!(
        stmt,
        Statement::Let {
            name: "x".to_string(),
            value: Expression::Number(42.0),
        }
    );
}

#[test]
fn test_parse_binary_expression() {
    let input = "5 + 3";
    let mut parser = Parser::new(input);
    let expr = parser.parse_expression().unwrap();
    assert_eq!(
        expr,
        Expression::Binary {
            left: Box::new(Expression::Number(5.0)),
            operator: Token::Plus,
            right: Box::new(Expression::Number(3.0)),
        }
    );
}

#[test]
fn test_parse_input_file() {
    // Get the path to test_input.ts
    let mut test_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_file_path.push("tests");
    test_file_path.push("test_input.ts");

    // Read the file contents
    let input = fs::read_to_string(test_file_path)
        .expect("Should be able to read test_input.ts");

    let mut parser = Parser::new(&input);
    let mut statements = Vec::new();

    while !parser.is_eof() {
        statements.push(parser.parse_statement().unwrap());
    }

    // For now, we only expect the first 9 statements to parse
    // (the ones that aren't commented out)
    assert_eq!(statements.len(), 9);

    // Check specific statements
    match &statements[0] {
        Statement::Let { name, value } => {
            assert_eq!(name, "x");
            assert_eq!(value, &Expression::Number(42.0));
        },
        _ => panic!("Expected let statement"),
    }

    match &statements[3] {
        Statement::Let { name, value } => {
            assert_eq!(name, "sum");
            assert_eq!(
                value,
                &Expression::Binary {
                    left: Box::new(Expression::Number(5.0)),
                    operator: Token::Plus,
                    right: Box::new(Expression::Number(3.0)),
                }
            );
        },
        _ => panic!("Expected let statement"),
    }
}

#[test]
fn test_complex_arithmetic() {
    let cases = vec![
        ("(5 + 3) * 2", 16.0),  // Parentheses
        ("2 * 3 + 4", 10.0),    // Multiplication before addition
        ("2 + 3 * 4", 14.0),    // Multiplication before addition
        ("10 / 2 * 3", 15.0),   // Left-to-right for same precedence
    ];

    for (input, expected) in cases {
        let mut parser = Parser::new(input);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr.evaluate().unwrap(), expected);
    }
}
