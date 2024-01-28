pub struct Token {}
pub struct Lexer {}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {}
    }

    pub fn next_token(&self) -> &str {
        ""
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_token_on_empty_input() {
        let lexer = Lexer::new("");
        assert_eq!(lexer.next_token(), "");
    }
}
