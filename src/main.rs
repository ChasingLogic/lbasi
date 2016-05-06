extern crate lbasi;

use lbasi::*;

fn main() {
    println!("{}", interpreter::Token::new("6".to_string()));
    println!("{}", interpreter::Token::new("+".to_string()));
    println!("{}", interpreter::Token::new("".to_string()))
}
