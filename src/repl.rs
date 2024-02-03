use crate::{ 
    lexer::Lexer, 
    token::Token 
};
use std::io::{self, Write};

const PROMPT: &str = ">> ";

pub fn start() -> io::Result<()> {
    loop {
        print!("{PROMPT}");
        io::stdout().flush()?;
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line)?;

        let mut lexer = Lexer::new(&input_line);
        let mut tok = lexer.next_token();
        while tok != Token::Eof {
            println!("{:?}", tok);
            tok = lexer.next_token();
        }
    }
}
