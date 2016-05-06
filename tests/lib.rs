extern crate lbasi;
use lbasi::*;

#[test]
fn test_token_creation() {
    assert!(interpreter::Token{ kind: interpreter::TokenType::Integer, value: "6".to_string() } == interpreter::Token::new("6".to_string()));
    assert!(interpreter::Token{ kind: interpreter::TokenType::Plus, value: "+".to_string() }    == interpreter::Token::new("+".to_string()));
    assert!(interpreter::Token{ kind: interpreter::TokenType::EOF, value: "".to_string() }      == interpreter::Token::new("".to_string()));
}
