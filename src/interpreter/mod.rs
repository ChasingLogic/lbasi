mod token;

pub use self::token::{ Token, TokenType };

#[derive(PartialEq, Debug)]
pub struct Interpreter {
    pos: usize,
    numbers: Vec<String>,
    operators: Vec<char>,
    current_token: Token,
}

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
    let mut iter = intrptr.numbers.iter();
    let mut result = iter.next()
        .expect("Unable to get first number")
        .parse::<i32>()
        .expect("Unable to convert number to int");

    let mut last_op = ' ';

    println!("Initial result: {}", result);

    loop {
        let num = match iter.next() {
            Some(n) => n.parse::<i32>().expect("Unable to convert number to int"),
            None    => break,
        };
            
        let operator = intrptr.operators.pop()
            .unwrap_or(last_op);
            
        match operator {
            '+' => result = result + num,
            '-' => result = result - num,
            _   => unreachable!(),
        };

        last_op = operator;

        println!("Result is now: {}", result);
    }

    result
}
