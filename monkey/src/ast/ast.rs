use token::*;

pub trait Node {
    fn token_literal(&self) -> String;
    fn to_string(&self) -> String;
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

    fn to_string(&self) -> String {
        let mut builder: String = String::new();

        for stmt in self.statements.iter() {
            builder.push_str(&(**stmt).to_string());
        }

        return builder;
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
            token:      tok,
            name:       name,
            value:      value,
        };
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }

    fn to_string(&self) -> String {
        let mut builder: String = String::new();

        builder.push_str(&self.token_literal());
        builder.push(' ');
        builder.push_str(&self.name.to_string());
        builder.push(' '); builder.push('=');

        // TODO: build value

        builder.push(';');

        return builder;
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

pub struct ReturnStatement {
    pub token:          token::Token,   // the token.RETURN token
    pub return_value:   Box<dyn Expression>,
}

impl ReturnStatement {
    pub fn new(tok: token::Token, ret_value: Box<dyn Expression>) -> ReturnStatement {
        return ReturnStatement{
            token:          tok,
            return_value:   ret_value,
        };
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }

    fn to_string(&self) -> String {
        let mut builder: String = String::new();

        builder.push_str(&self.token_literal());
        builder.push(' ');

        // TODO: build return_value

        builder.push(';');

        return builder;
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}

pub struct ExpressionStatement {
    pub token:          token::Token,   // the first token of the expression
    pub expression:     Box<dyn Expression>,
}

impl ExpressionStatement {
    pub fn new(tok: token::Token, exp: Box<dyn Expression>) -> ExpressionStatement {
        return ExpressionStatement{
            token:          tok,
            expression:     exp,
        };
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }

    fn to_string(&self) -> String {
        let mut builder: String = String::new();

        builder.push_str(&self.token_literal());
        builder.push(' ');

        // TODO: build expression

        builder.push(';');

        return builder;
    }
}

impl Statement for ExpressionStatement {
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

    fn to_string(&self) -> String {
        return self.value.clone();
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {

    }
}

