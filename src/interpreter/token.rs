use std::fmt;

#[derive(PartialEq, Debug)]
pub enum TokenType {
    Integer,
    Invalid,
    Ignore,
    Plus
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let typ = match *self {
            TokenType::Integer => "INTEGER",
            TokenType::Invalid => "INVALID",
            TokenType::Ignore  => "IGNORE",
            TokenType::Plus    => "PLUS",
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
            ' '     => Token{ kind: TokenType::Ignore, value: value },
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
