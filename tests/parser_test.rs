use ts_interpreter_rs::parser::*;
use ts_interpreter_rs::interpreter::*;

#[test]
fn test_expression_evaluation() {
    let cases = vec![
        ("5 + 3", Value::Number(8.0)),
        ("10 - 4", Value::Number(6.0)),
        ("2 * 3", Value::Number(6.0)),
        ("8 / 2", Value::Number(4.0)),
    ];

    for (input, expected) in cases {
        let mut parser = Parser::new(input);
        let expr = parser.parse_expression().unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.eval(Statement::Expression(expr)).unwrap();
        assert_eq!(result, expected);
    }
}

#[test]
fn test_statement_evaluation() {
    let cases = vec![
        ("let x = 42;", Value::Number(42.0)),
        ("let message = \"Hello\";", Value::String("Hello".to_string())),
    ];

    for (input, expected) in cases {
        let mut parser = Parser::new(input);
        let mut interpreter = Interpreter::new();
        
        while !parser.is_eof() {
            let stmt = parser.parse_statement().unwrap();
            let result = interpreter.eval(stmt).unwrap();
            assert_eq!(result, expected);
        }
    }
}
