#[derive(Debug, PartialEq)]
pub enum Token {
    Unknown,
    Eof,
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    pub fn next_token(&mut self) -> Token {
        if self.pos < self.input.len() {
            self.pos += self.input.len();
            Token::Unknown
        } else {
            Token::Eof
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eof_token_on_empty_input() {
        let mut lexer = Lexer::new("");
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn unknown_token_then_eof() {
        let mut lexer = Lexer::new("some unknown mess");
        assert_eq!(lexer.next_token(), Token::Unknown);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn always_eof_after_lexing() {
        let mut lexer = Lexer::new("let a = 5;");
        while lexer.next_token() != Token::Eof {}
        assert_eq!(lexer.next_token(), Token::Eof);
    }
}
