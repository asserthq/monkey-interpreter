use crate::token::Token;

pub struct Lexer {
    input_chars: Vec<char>,
    pos: usize,
    sym: Option<char>,
    next_sym: Option<char>
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input_chars: input.chars().collect(),
            pos: 0,
            sym: input.chars().next(),
            next_sym: input.chars().nth(1)
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespaces();
        match self.sym {
            None => Token::Eof,
            Some(sym) if can_be_in_identifier(sym) => {
                let ident_literal = self.read_identifier();
                Token::from_string(ident_literal)
            }
            Some(sym) if sym.is_ascii_digit() => {
                let int_literal = self.read_integer();
                Token::Int(int_literal)
            }
            Some(sym) => {
                let mut tok = Token::Unknown;
                if can_begin_two_chars_token(sym) {
                    if let Some(next_sym) = self.next_sym {
                        tok = Token::from_two_chars(sym, next_sym);
                    }
                }
                if tok != Token::Unknown {
                    self.read_sym();
                } else {
                    tok = Token::from_single_char(sym);
                }
                self.read_sym();
                tok
            }
        }
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
        self.sym = self.next_sym;
        self.next_sym = self.input_chars.get(self.pos + 1).copied();
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

fn can_begin_two_chars_token(ch: char) -> bool {
    ch == '=' || ch == '!'
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
        let input = "=; ({^ <>/*})+-,%";
        let check = vec![
            Assign, 
            Semicolon,
            LParen,
            LBrace,
            Unknown,
            LT,
            GT,
            Slash,
            Asterisk,
            RBrace,
            RParen,
            Plus,
            Minus,
            Comma,
            Unknown
        ];
        let mut tokens: Vec<Token> = Vec::with_capacity(16);
        let mut lexer = Lexer::new(input); 
        let mut tok = lexer.next_token();
        while tok != Token::Eof {
            tokens.push(tok);
            tok = lexer.next_token();
        }
        assert_eq!(tokens, check);
    }

    #[test]
    fn tokenize_two_characters_tokens() {
        use Token::*;
        let input = "== !=";
        let check = vec![
            Eq,
            NotEq,
        ];
        let mut tokens: Vec<Token> = Vec::with_capacity(2);
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
            let five 5
            fn ten 10
        "#;
        let check = vec![
            Let,
            Ident("five".into()),
            Int("5".into()),
            Function,
            Ident("ten".into()),
            Int("10".into())
        ];
        let mut tokens: Vec<Token> = Vec::with_capacity(64);
        let mut lexer = Lexer::new(input); 
        let mut tok = lexer.next_token();
        while tok != Token::Eof {
            tokens.push(tok);
            tok = lexer.next_token();
        }
        assert_eq!(tokens, check);
    }

    #[test]
    fn tokenize_all_tokens() {
        use Token::*;
        let input = r#"
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
            let a = 10<y >z/-*==;
            24 != 23
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
            Semicolon,
            Let,
            Ident("a".into()),
            Assign,
            Int("10".into()),
            LT,
            Ident("y".into()),
            GT,
            Ident("z".into()),
            Slash,
            Minus,
            Asterisk,
            Eq,
            Semicolon,
            Int("24".into()),
            NotEq,
            Int("23".into())
        ];
        let mut tokens: Vec<Token> = Vec::with_capacity(64);
        let mut lexer = Lexer::new(input); 
        let mut tok = lexer.next_token();
        while tok != Token::Eof {
            tokens.push(tok);
            tok = lexer.next_token();
        }
        assert_eq!(tokens, check);
    }
    
}
