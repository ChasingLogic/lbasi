mod token;

pub use self::token::{Token, TokenType};
use std::str;

pub struct Interpreter<'a> {
    pos: usize,
    current_token: Token,
    body: String,
    chars: str::Chars<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(body: &'a String) -> Interpreter<'a> {
        Interpreter {
            pos: 0,
            current_token: Token::new('\\'),
            body: body.clone(),
            chars: body.chars(),
        }
    }

    fn error(&self) -> ! {
        panic!("Error parsing input at character {} at {}",
               self.current_token,
               self.pos);
    }

    fn get_next_token(&mut self) -> Token {
        let mut chr = self.chars.next();

        if chr.is_none() {
            return Token::eof();
        }

        while " \n\t".contains(chr.unwrap()) {
            chr = self.chars.next();

            if chr.is_none() {
                return Token::eof();
            }
        }

        self.pos += 1;

        Token::new(chr.unwrap().clone())
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
