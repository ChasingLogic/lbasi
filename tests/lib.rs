extern crate lbasi;
use lbasi::*;

#[test]
fn test_token_creation() {
    assert!(interpreter::Token{ kind: interpreter::TokenType::Integer, value: '6' } == interpreter::Token::new('6'));
    assert!(interpreter::Token{ kind: interpreter::TokenType::Plus, value: '+' }    == interpreter::Token::new('+'));
    assert!(interpreter::Token{ kind: interpreter::TokenType::Ignore, value: ' ' }  == interpreter::Token::new(' '));
}

#[test]
fn test_simple_subtraction() {
    let answer = interpreter::run("5 - 3".to_string())
        .expect("Got an error running interpreter");
    
    assert!(answer == 2, "answer = {}", answer)
}

#[test]
fn test_complex_subtraction() {
    let answer = interpreter::run("10 - 2 - 4".to_string())
        .expect("Got an error running interpreter");
    
    assert!(answer == 4, "answer = {}", answer)
}

#[test]
fn test_simple_addition() {
    let answer = interpreter::run("3 + 5".to_string())
        .expect("Got an error running interpreter");
    
    assert!(answer == 8, "answer = {}", answer)
}

#[test]
fn test_complex_addition() {
    let answer = interpreter::run("3 + 5 + 6 + 9".to_string())
        .expect("Got an error running interpreter");
    
    assert!(answer == 23, "answer = {}", answer)
}

#[test]
fn test_complex_numbers() {
    let answer = interpreter::run("33 + 25 + 10".to_string())
        .expect("Got an error running interpreter");
    
    assert!(answer == 68, "answer = {}", answer)
}
