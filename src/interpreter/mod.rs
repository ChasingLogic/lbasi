mod token;

pub use self::token::{ Token, TokenType };

#[derive(PartialEq, Debug)]
pub struct Interpreter {
    pos: usize,
    numbers: Vec<String>,
    operators: Vec<char>,
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
    pub fn new() -> Interpreter {
        Interpreter{ 
            current_token: Token::new(' '),
            operators: Vec::new(),
            numbers: Vec::new(),
            pos: 0,
        }
    }
}

pub fn run(body: String) -> Result<i32, String> {
    let mut i = Interpreter::new();

    for char in body.chars() {
        let t = Token::new(char);

        if t.kind == TokenType::Invalid {
           return Err(format!("Invalid token: {}", t))
        }

        if t.kind != TokenType::Ignore {
            eat_token(&mut i, t);
        }
    }

    Ok(calculate(&mut i))
}

fn eat_token(intrptr: &mut Interpreter, t: Token) {
    match t.kind {
        ref x if *x == TokenType::Integer
            && intrptr.current_token.kind == TokenType::Integer
            => {
                intrptr.numbers[intrptr.pos - 1].push_str(t.value.to_string().as_str());
            }
        TokenType::Integer => {
            intrptr.pos += 1;
            intrptr.numbers.push(t.value.to_string());
        },
        _ => intrptr.operators.push(t.value),
    };


    intrptr.current_token = t;
}

fn calculate(intrptr: &mut Interpreter) -> i32 {
    let mut result: i32 = 0;
    let mut last_op = ' ';

    loop {
        let num = intrptr.numbers.pop()
            .expect("Unable to pop number")
            .parse::<i32>()
            .expect("Unable to convert number to int");

        let operator = match intrptr.operators.pop() {
            Some(c) => c,
            None    => last_op,
        };
            
        match operator {
            '+' => { 
                result = result + num;
                last_op = '+';
            },
            _   => unreachable!(),
        };

        if intrptr.numbers.len() == 0 {
            break
        }
    }

    result
}
