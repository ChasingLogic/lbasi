mod interpreter;

fn main() {
    println!("{}", interpreter::token::Token{ kind: interpreter::token::TokenType::Integer, value: 6 });
    println!("{}", interpreter::token::Token{ kind: interpreter::token::TokenType::Plus, value: "+" });
    println!("{}", interpreter::token::Token{ kind: interpreter::token::TokenType::EOF, value: "EOF" })
}
