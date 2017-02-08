use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    Integer,
    Subtract,
    Add,
    Multiply,
    Divide,
    Invalid,
    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let typ = match *self {
            TokenType::Integer => "Integer",
            TokenType::Add => "Add",
            TokenType::Subtract => "Subtract",
            TokenType::Multiply => "Multiply",
            TokenType::Divide => "Divide",
            TokenType::Invalid => "Invalid",
            TokenType::EOF => "EOF",
        };

        write!(f, "{}", typ)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub value: String,
}

fn parse_token(value: char) -> TokenType {
    match value {
        '+' => TokenType::Add,
        '-' => TokenType::Subtract,
        '*' => TokenType::Multiply,
        '/' => TokenType::Divide,
        _ => TokenType::Invalid,
    }
}

impl Token {
    pub fn op(value: char) -> Token {
        Token {
            kind: parse_token(value),
            value: value.to_string(),
        }
    }


    pub fn eof() -> Token {
        Token {
            kind: TokenType::EOF,
            value: "".to_string(),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TOKEN({}, {})", self.kind, self.value)
    }
}
