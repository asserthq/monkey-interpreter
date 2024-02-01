#[derive(Debug, PartialEq)]
pub enum Token {
    Unknown,
    Eof,
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    sym: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            sym: None,
        }
    }

    pub fn next_token(&mut self) -> Token {
        if self.pos >= self.input.len() {
            return Token::Eof;
        }
        let tok = Token::Unknown;
        self.pos += 1;
        tok
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
    fn unknown_tokens_until_eof() {
        let mut lexer = Lexer::new("some unknown mess");
        let mut tok = lexer.next_token();
        while tok != Token::Eof {
            assert_eq!(tok, Token::Unknown);
            tok = lexer.next_token();
        }
    }

    #[test]
    fn always_eof_after_lexing() {
        let mut lexer = Lexer::new("let a = 5;");
        while lexer.next_token() != Token::Eof {}
        assert_eq!(lexer.next_token(), Token::Eof);
    }
}
