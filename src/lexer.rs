#[derive(Debug, PartialEq)]
pub enum Token {
    Unknown,
    Eof,
}

pub struct Lexer {}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {}
    }

    pub fn next_token(&self) -> Token {
        Token::Eof
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eof_token_on_empty_input() {
        let lexer = Lexer::new("");
        assert_eq!(lexer.next_token(), Token::Eof);
    }
}
