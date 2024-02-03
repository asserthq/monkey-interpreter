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
    Asterisk,
    Slash,

    GT,
    LT,

    Eq,
    NotEq,

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
            '*' => Asterisk,
            '/' => Slash,
            
            '>' => GT,
            '<' => LT,

            _ => Unknown,
        }
    }

    pub fn from_two_chars(first: char, second: char) -> Token {
        use Token::*;
        match (first, second) {
            ('=', '=') => Eq,
            ('!', '=') => NotEq,

            _ => Unknown,
        }
    }

    pub fn from_string(literal: String) -> Token {    
        use Token::*;
        match literal.as_str() {
            "let" => Let,
            "fn" => Function,
            _ => Ident(literal)
        }
    }
}
