#[derive(Debug, PartialEq)]
pub enum Token {
    Unknown,
    Eof,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Comma,
    Semicolon,

    Assign,
    Plus,
    Minus,

    Let,
    Function,

    Ident(String),
    Int(String)
}

impl Token {
    pub fn from_single_char(ch: char) -> Token {
        use Token::*;
        match ch {
            '(' => LParen,
            ')' => RParen,
            '{' => LBrace,
            '}' => RBrace,

            ',' => Comma,
            ';' => Semicolon,

            '=' => Assign,
            '+' => Plus,
            '-' => Minus,

            _ => Unknown,
        }
    }
}
