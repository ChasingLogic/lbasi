mod token;

pub use self::token::{Token, TokenType};
use std::str;
use std::option::Option;

pub struct Interpreter<'a> {
    pos: usize,
    body: String,
    current_char: Option<char>,
    current_token: Token,
    chars: str::Chars<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(body: &'a String) -> Interpreter<'a> {
        Interpreter {
            pos: 0,
            body: body.clone(),
            current_char: None,
            current_token: Token::new('\\'),
            chars: body.chars(),
        }
    }

    fn error(&self) -> ! {
        panic!("Error parsing input at character {} at {}",
               self.current_token,
               self.pos);
    }

    fn advance(&mut self) {
        self.pos += 1;
        self.current_char = self.chars.next();
    }

    fn skip_whitespace(&mut self) {
        while self.current_char.is_some() && " \n\t".contains(self.current_char.unwrap()) {
            self.advance();
        }
    }

    fn get_next_token(&mut self) {
        while self.current_char.is_some() {
            self.current_token = Token::new(self.current_char.unwrap().clone());
        }

    }

    fn eat_token(&mut self, t: TokenType) {
        if self.current_token.kind == t {
            let next_token = self.get_next_token();

            if next_token.kind == self.current_token.kind &&
               self.current_token.kind == TokenType::Integer {

                // if it's an integer and the next token is an integer make a multi-digit integer
                self.current_token
                    .value
                    .push_str(next_token.value.clone().as_str());
            }

            self.current_token = next_token.clone();
            return;
        }

        self.error();
    }

    fn expr(&mut self) -> Result<i32, String> {
        self.current_token = self.get_next_token();

        while self.current_token.kind != TokenType::EOF {
            let left = self.current_token.clone();
            self.eat_token(TokenType::Integer);


            let op = self.current_token.clone();
            match op.kind {
                TokenType::Add => self.eat_token(TokenType::Add),
                TokenType::Subtract => self.eat_token(TokenType::Subtract),
                TokenType::Multiply => self.eat_token(TokenType::Multiply),
                TokenType::Divide => self.eat_token(TokenType::Divide),
                _ => return Err(format!("Unrecognized operator: {}", op.value)),
            };

            let right = self.current_token.clone();
            self.eat_token(TokenType::Integer);

            match op.kind {
                TokenType::Add => {
                    Ok(left.value.parse::<i32>().unwrap() + right.value.parse::<i32>().unwrap())
                }
                TokenType::Subtract => {
                    Ok(left.value.parse::<i32>().unwrap() - right.value.parse::<i32>().unwrap())
                }
                TokenType::Multiply => {
                    Ok(left.value.parse::<i32>().unwrap() * right.value.parse::<i32>().unwrap())
                }
                TokenType::Divide => {
                    Ok(left.value.parse::<i32>().unwrap() / right.value.parse::<i32>().unwrap())
                }
                _ => Err("Op not correct type!".to_string()),
            }
        }
    }
}

pub fn run(body: String) -> Result<i32, String> {
    let mut i = Interpreter::new(&body);

    i.expr()
}
