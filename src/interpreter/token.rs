use std::fmt;

pub enum TokenType {
    Integer,
    Plus,
    EOF
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut typ = "";

        match *self {
            TokenType::Integer => typ = "Integer",
            TokenType::Plus => typ = "Plus",
            TokenType::EOF => typ = "EOF",
        };

        write!(f, "{}", typ)
    }
}

pub struct Token<T> {
    pub kind: TokenType,
    pub value: T,
}

impl<T> Token<T> {
    fn new(value: String) -> Token<T> {
        match value {
            "+" => Token{ kind: TokenType::Plus, value: value},
            "" => Token{ kind: TokenType::EOF, value: value },
            _ => Token{ kind: TokenType::Integer, value: value},
        }
    }
}

impl<T> fmt::Display for Token<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TOKEN({}, {})", self.kind, self.value)
    }
}
