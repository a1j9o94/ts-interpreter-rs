use ts_interpreter_rs::interpreter::*;
use ts_interpreter_rs::parser::*;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_evaluate_complex_arithmetic() {
    let cases = vec![
        ("(5 + 3) * 2", Value::Number(16.0)),
        ("2 * 3 + 4", Value::Number(10.0)),
        ("2 + 3 * 4", Value::Number(14.0)),
    ];

    let mut interpreter = Interpreter::new();
    for (input, expected) in cases {
        let mut parser = Parser::new(input);
        let expr = parser.parse_expression().unwrap();
        let result = interpreter.eval(Statement::Expression(expr)).unwrap();
        assert_eq!(result, expected);
    }
}

#[test]
fn test_evaluate_variables() {
    let mut interpreter = Interpreter::new();
    let mut parser = Parser::new("let x = 42; let y = x + 8;");
    
    while !parser.is_eof() {
        let stmt = parser.parse_statement().unwrap();
        interpreter.eval(stmt).unwrap();
    }
    
    assert_eq!(
        interpreter.get_last_value(),
        Some(&Value::Number(50.0))
    );
}

#[test]
fn test_string_operations() {
    let mut interpreter = Interpreter::new();
    let mut parser = Parser::new(r#"let greeting = "Hello, "; let name = "World"; let message = greeting + name;"#);
    
    while !parser.is_eof() {
        let stmt = parser.parse_statement().unwrap();
        interpreter.eval(stmt).unwrap();
    }
    
    assert_eq!(
        interpreter.get_last_value(),
        Some(&Value::String("Hello, World".to_string()))
    );
}

#[test]
fn test_evaluate_input_file() {
    let mut test_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_file_path.push("tests");
    test_file_path.push("test_input.ts");

    let input = fs::read_to_string(test_file_path)
        .expect("Should be able to read test_input.ts");

    let mut interpreter = Interpreter::new();
    let mut parser = Parser::new(&input);
    
    // Parse and evaluate all statements
    while !parser.is_eof() {
        if let Ok(stmt) = parser.parse_statement() {
            // Verify each statement can be evaluated without error
            interpreter.eval(stmt).unwrap_or_else(|e| {
                panic!("Failed to evaluate statement: {}", e);
            });
        }
    }

    // Verify we have some result
    assert!(interpreter.get_last_value().is_some(), 
        "Should have evaluated at least one expression");

    // Verify all variables in the context are valid
    for (_name, value) in interpreter.get_variables() {
        match value {
            Value::Number(n) => assert!(!n.is_nan(), "Number values should be valid"),
            Value::String(s) => assert!(!s.is_empty(), "String values should not be empty"),
            Value::Boolean(_) => {},
            Value::Null => {},
        }
    }
}
