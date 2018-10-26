use token::*;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn new() -> Program {
        let program: Program = Program{
            statements: Vec::new(),
        };
        return program;
    }
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        } else {
            return String::new();
        }
    }
}

pub struct LetStatement {
    pub token:      token::Token,       // the token.LET token
    pub name:       Identifier,
    pub value:      Box<dyn Expression>,
}

impl LetStatement {
    pub fn new(tok: token::Token, name: Identifier, value: Box<dyn Expression>) -> LetStatement {
        return LetStatement{
            token:      tok,        // the token.LET token
            name:       name,
            value:      value,
        };
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

pub struct Identifier {
    pub token:      token::Token,       // the token.IDENT token
    pub value:      String,
}

impl Identifier {
    pub fn new(tok: token::Token, value: String) -> Identifier {
        return Identifier{
            token:      tok,
            value:      value,
        };
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

