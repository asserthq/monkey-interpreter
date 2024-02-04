use crate::token::Token;

trait Node {
    fn token_literal(&self) -> String;
}

trait Statement where Self: Node {
    fn statement_node(&self);
}

trait Expression where Self: Node {
    fn expression_node(&self);
}

struct Program {
    statements: Vec<Box<dyn Statement>>
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            "".into()
        }
    }
}

struct LetStatement {
    token: Token,
    name: Box<Identifier>,
    value: Box<dyn Expression>
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        "".into()
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

#[derive(Debug)]
struct Identifier {
    token: Token,
    value: String
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        if let Token::Ident(literal) = &self.token {
            literal.to_owned()
        } else {
            "".into()
        }
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simply_way_construct_ast() {
        let ident = Identifier {
            token: Token::Ident("ident".into()),
            value: "".into()
        };
         
        let expr = Identifier {
            token: Token::Ident("expr".into()),
            value: "".into()
        };

        let let_stmt = LetStatement {
            token: Token::Let,
            name: Box::new(ident),
            value: Box::new(expr)            
        };

        let mut stmts: Vec<Box<dyn Statement>> = Vec::new();
        stmts.push(Box::new(let_stmt));
                    
        let root = Program {
            statements: stmts
        };
    }
}
