extern crate lbasi;
use lbasi::*;

#[test]
fn test_token_creation() {
    assert!(interpreter::Token{ kind: interpreter::TokenType::Integer, value: '6' } == interpreter::Token::new('6'));
    assert!(interpreter::Token{ kind: interpreter::TokenType::Plus, value: '+' }    == interpreter::Token::new('+'));
    assert!(interpreter::Token{ kind: interpreter::TokenType::Subtract, value: '-' }    == interpreter::Token::new('-'));
}

#[test]
fn test_interpreter() {
    interpreter_test("5 - 3".to_string(), 2);
    interpreter_test("4 - 2 - 4".to_string(), -2);
    interpreter_test("10 - 2 - 4".to_string(), 4);
    interpreter_test("5 + 3".to_string(), 8);
    interpreter_test("3 + 5 + 6 + 9".to_string(), 23);
    interpreter_test("33 + 25 + 10".to_string(), 68);
    interpreter_test("22 - 11 + 10 - 21".to_string(), 0);
    interpreter_test("3 + 4 - 2".to_string(), 5);
}

fn interpreter_test(expr: String, expected: i32) {
    let answer = interpreter::run(expr.clone())
        .expect("Got an error running interpreter");
    
    assert!(answer == expected, "expr = {} answer = {}", expr, answer)
}
