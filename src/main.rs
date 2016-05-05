mod interpreter;
mod token;

fn main() {
    println!("{}", token::Token::new("6".to_string()));
    println!("{}", token::Token::new("+".to_string()));
    println!("{}", token::Token::new("".to_string()))
}
