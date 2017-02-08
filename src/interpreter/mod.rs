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
            current_token: Token::eof(),
            chars: body.chars(),
        }
    }

    fn current_char(&self) -> char {
        self.current_char.unwrap()
    }

    fn error(&self) -> ! {
        panic!("Error parsing input: character {} @ {}",
               self.current_token,
               self.pos);
    }

    fn advance(&mut self) {
        self.pos += 1;
        self.current_char = self.chars.next();
    }

    fn skip_whitespace(&mut self) {
        while self.current_char.is_some() && is_whitespace(self.current_char()) {
            self.advance();
        }
    }

    fn integer(&mut self) -> String {
        let mut i = "".to_string();

        while is_digit(self.current_char()) {
            i.push(self.current_char());
            self.advance();
        }

        i
    }

    fn get_next_token(&mut self) -> Token {
        while self.current_char.is_some() {
            if is_whitespace(self.current_char()) {
                self.skip_whitespace();
                continue;
            }

            if is_digit(self.current_char()) {
                let t = Token {
                    kind: TokenType::Integer,
                    value: self.integer(),
                };

                self.advance();
                return t;
            }

            let t = Token::op(self.current_char());
            self.advance();
            return t;
        }

        Token::eof()
    }

    fn eat(&mut self, t: TokenType) {
        if self.current_token.kind == t {
            self.current_token = self.get_next_token();
            return;
        }

        self.error();
    }

    fn term(&self) -> i32 {}

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

pub fn is_whitespace(c: char) -> bool {
    " \n\t".contains(c)
}

pub fn is_digit(c: char) -> bool {
    "0123456789".contains(c)
}

pub fn run(body: String) -> Result<i32, String> {
    let mut i = Interpreter::new(&body);

    i.expr()
}
