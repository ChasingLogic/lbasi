mod token;

pub use self::token::{ Token, TokenType };

pub struct Interpreter {
    body: String,
    numbers: &[String],
    operators: &[String],
    current_token: Token,
}

/* 
 * TODO: Finish this
 * My current plan is concatenate integer strings together if the previous 
 * token was also an integer that way you can have multi-digit numbers 
 * (i.e. 321) then once we find something that isn't an integer add that 
 * concantenated integer into the numbers vector. Keep operators in the 
 * operators array then we can process through both arrays applying the given 
 * operator to every other number in reverse order. i.e:
 *
 * let num2 = numbers.pop();
 * let num1 = numbers.pop();
 * let op = operators.pop();
 *
 * match op {
 *  "+" => num1 + num2,
 * }
 *
 * etc. This doesn't follow PEMDAS but I'm not worried about that right now.
 */
impl Interpreter {
    pub fn new(body: String) -> Interpreter {
        Interpreter{ body: body, current_token: token::Token::new("") }
    }

    pub fn run() -> Result<i32, &'static str> {
        for char in self.body.chars() {
            let t = token::Token::new(char);

            if t.kind == token::TokenType::Invalid {
               return Err(format!("Invalid token: {}", t))
            }

            self.eat_token(t);
        }
    }

    fn eat_token(t: token::Token) {
        
    }
}
