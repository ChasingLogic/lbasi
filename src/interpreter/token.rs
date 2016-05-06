use std::fmt;

#[derive(PartialEq, Debug)]
pub enum TokenType {
    Integer,
    Invalid,
    Plus,
    EOF
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let typ = match *self {
            TokenType::Integer => "INTEGER",
            TokenType::Invalid => "INVALID",
            TokenType::Plus    => "PLUS",
            TokenType::EOF     => "EOF",
        };

        write!(f, "{}", typ)
    }
}

#[derive(PartialEq, Debug)]
pub struct Token {
    pub kind: TokenType,
    pub value: char,
}

impl Token {
    pub fn new(value: char) -> Token {
        match value {
            '+'     => Token{ kind: TokenType::Plus,    value: value },
            'E'      => Token{ kind: TokenType::EOF,     value: value },
            c if (c >= '0' && c <= '9') => Token{ kind: TokenType::Integer, value: value},
            _       => Token{ kind: TokenType::Invalid, value: value},
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TOKEN({}, {})", self.kind, self.value)
    }
}
