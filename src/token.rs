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
    pub fn from_string(input: String) -> Token {
        use Token::*;
        match input.as_str() {
            "(" => LParen,
            ")" => RParen,
            "{" => LBrace,
            "}" => RBrace,

            "," => Comma,
            ";" => Semicolon,

            "=" => Assign,
            "+" => Plus,
            "-" => Minus,

            _ => Unknown,
        }
    }
}
