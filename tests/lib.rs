extern crate lbasi;
use lbasi::*;

#[test]
fn test_operator_identification() {
    assert_eq!(interpreter::Token {
                   kind: interpreter::TokenType::Add,
                   value: "+".to_string(),
               },
               interpreter::Token::new('+'));

    assert_eq!(interpreter::Token {
                   kind: interpreter::TokenType::Subtract,
                   value: "-".to_string(),
               },
               interpreter::Token::new('-'));

    assert_eq!(interpreter::Token {
                   kind: interpreter::TokenType::Multiply,
                   value: "*".to_string(),
               },
               interpreter::Token::new('*'));

    assert_eq!(interpreter::Token {
                   kind: interpreter::TokenType::Divide,
                   value: "/".to_string(),
               },
               interpreter::Token::new('/'));
}

#[test]
fn test_interpreter() {
    interpreter_test("5 - 3", 2);
    interpreter_test("4 - 2 - 4", -2);
    interpreter_test("10 - 2 - 4", 4);
    interpreter_test("5 + 3", 8);
    interpreter_test("3 + 5 + 6 + 9", 23);
    interpreter_test("33 + 25 + 10", 68);
    interpreter_test("22 - 11 + 10 - 21", 0);
    interpreter_test("3 + 4 - 2", 5);
    interpreter_test("2 * 3", 6);
    interpreter_test("1 * 3", 3);
    interpreter_test("4 / 2", 2);
    interpreter_test("20 / 2", 10);
}

fn interpreter_test(expr: &str, expected: i32) {
    let answer = interpreter::run(expr.to_string().clone())
        .expect("Got an error running interpreter");

    assert!(answer == expected,
            "expr = {} answer = {} expected = {}",
            expr,
            answer,
            expected)
}
