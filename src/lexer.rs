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

pub struct Lexer{
    input: String,
    pos: usize,
    sym: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            pos: 0,
            sym: input.chars().next(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        let tok = match self.sym {
            None => Token::Eof,
            Some(sym) => {
                Token::from_string(sym.to_string())
            }
        };
        self.pos += 1;
        self.sym = self.next_sym();
        tok
    }

    fn next_sym(&mut self) -> Option<char> {
        self.input.chars().nth(self.pos)
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

    #[test]
    fn tokenize_single_character_tokens() {
        use Token::*;
        let input = "a=5;({ })+-,";
        let check = vec![
            Unknown,
            Assign, 
            Unknown,
            Semicolon,
            LParen,
            LBrace,
            Unknown,
            RBrace,
            RParen,
            Plus,
            Minus,
            Comma
        ];
        let mut tokens: Vec<Token> = Vec::with_capacity(12);
        let mut lexer = Lexer::new(input); 
        let mut tok = lexer.next_token();
        while tok != Token::Eof {
            tokens.push(tok);
            tok = lexer.next_token();
        }
        assert_eq!(tokens, check);
    }

    #[test]
    fn tokenize_multi_character_tokens() {
        use Token::*;
        let input = r#"
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
        "#;
        let check = vec![
            Let,
            Ident("five".into()),
            Assign,
            Int("5".into()),
            Semicolon,
            Let,
            Ident("ten".into()),
            Assign,
            Int("10".into()),
            Semicolon,
            Let,
            Ident("add".into()),
            Assign,
            Function,
            LParen,
            Ident("x".into()),
            Comma,
            Ident("y".into()),
            RParen,
            LBrace,
            Ident("x".into()),
            Plus,
            Ident("y".into()),
            Semicolon,
            RBrace,
            Semicolon,
            Let,
            Ident("result".into()),
            Assign,
            Ident("add".into()),
            LParen,
            Ident("five".into()), 
            Comma, 
            Ident("ten".into()),
            RParen,
            Semicolon
        ];
        let mut tokens: Vec<Token> = Vec::with_capacity(40);
        let mut lexer = Lexer::new(input); 
        let mut tok = lexer.next_token();
        while tok != Token::Eof {
            tokens.push(tok);
            tok = lexer.next_token();
        }
        assert_eq!(tokens, check);
    }
    
}
