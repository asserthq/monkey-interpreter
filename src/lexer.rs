use crate::token::Token;

pub struct Lexer{
    input: String,
    pos: usize,
    sym: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        println!("input:");
        println!("{input}");
        Self {
            input: input.to_string(),
            pos: 0,
            sym: input.chars().next(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        let mut tok: Token = Token::Eof;
        self.skip_whitespaces();
        match self.sym {
            None => (),
            Some(sym) if can_be_in_identifier(sym) => {
                let ident_literal = self.read_identifier();
                tok = Token::from_string(ident_literal);
            }
            Some(sym) if sym.is_ascii_digit() => {
                let int_literal = self.read_integer();
                tok = Token::Int(int_literal);
            }
            Some(sym) => {
                tok = Token::from_single_char(sym);
                self.read_sym();
            }
        };
        tok
    }

    fn skip_whitespaces(&mut self) {
        while let Some(sym) = self.sym {
            if !sym.is_whitespace() {
                break;
            }
            self.read_sym();
        }
    }

    fn read_sym(&mut self) {
        self.pos += 1;
        self.sym = self.input.chars().nth(self.pos);
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(sym) = self.sym {
            if !can_be_in_identifier(sym) {
                break;
            }
            ident.push(sym);
            self.read_sym();
        }
        ident
    }

    fn read_integer(&mut self) -> String {
        let mut num = String::new();
        while let Some(sym) = self.sym {
            if !sym.is_ascii_digit() {
                break;
            }
            num.push(sym);
            self.read_sym();
        }
        num
    }
}

fn can_be_in_identifier(sym: char) -> bool {
    sym.is_alphabetic() || sym == '_'     
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
    fn always_eof_after_lexing() {
        let mut lexer = Lexer::new("let a = 5;");
        while lexer.next_token() != Token::Eof {}
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn tokenize_single_character_tokens() {
        use Token::*;
        let input = "=; ({^ })+-,%";
        let check = vec![
            Assign, 
            Semicolon,
            LParen,
            LBrace,
            Unknown,
            RBrace,
            RParen,
            Plus,
            Minus,
            Comma,
            Unknown
        ];
        let mut tokens: Vec<Token> = Vec::with_capacity(11);
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
