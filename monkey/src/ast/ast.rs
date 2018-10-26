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
            builder.push('\n');
        }

        return builder;
    }
}

pub struct LetStatement {
    pub token:      token::Token,       // the token.LET token
    pub name:       Identifier,
    pub value:      Option<Box<dyn Expression>>,
}

impl LetStatement {
    pub fn new(tok: token::Token, name: Identifier, value: Option<Box<dyn Expression>>) -> LetStatement {
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
        builder.push(' '); builder.push('='); builder.push(' ');

        match self.value.as_ref().clone() {
            Some(val) => builder.push_str(&(*val).to_string()),
            None => (),
        }

        builder.push(';');

        return builder;
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

pub struct ReturnStatement {
    pub token:          token::Token,   // the token.RETURN token
    pub return_value:   Option<Box<dyn Expression>>,
}

impl ReturnStatement {
    pub fn new(tok: token::Token, ret_value: Option<Box<dyn Expression>>) -> ReturnStatement {
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

        match self.return_value.as_ref().clone() {
            Some(ret_val) => builder.push_str(&(*ret_val).to_string()),
            None => (),
        }

        builder.push(';');

        return builder;
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}

pub struct ExpressionStatement {
    pub token:          token::Token,   // the first token of the expression
    pub expression:     Option<Box<dyn Expression>>,
}

impl ExpressionStatement {
    pub fn new(tok: token::Token, exp: Option<Box<dyn Expression>>) -> ExpressionStatement {
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

        match self.expression.as_ref().clone() {
            Some(exp) => builder.push_str(&(*exp).to_string()),
            None => (),
        }

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

pub struct IntegerLiteral {
    pub token:      token::Token,       // the token.INT token
    pub value:      i64,
}

impl IntegerLiteral {
    pub fn new(tok: token::Token, value: i64) -> IntegerLiteral {
        return IntegerLiteral{
            token:      tok,
            value:      value,
        };
    }
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }

    fn to_string(&self) -> String {
        return self.token_literal();
    }
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let ls = LetStatement::new(
            token::Token::new(token::LET, String::from(token::LET)),
            Identifier::new(token::Token::new(token::IDENT, String::from("myVar")), String::from("myVar")),
            Some(Box::new(Identifier::new(token::Token::new(token::IDENT, String::from("anotherVar")), String::from("anotherVar"))))
        );
        assert_eq!(ls.to_string(), "let myVar = anotherVar;");
        let rs = ReturnStatement::new(
            token::Token::new(token::RETURN, String::from(token::RETURN)),
            Some(Box::new(Identifier::new(token::Token::new(token::IDENT, String::from("myVar")), String::from("myVar"))))
        );
        assert_eq!(rs.to_string(), "return myVar;");
        let es = ExpressionStatement::new(
            token::Token::new(token::PLUS, String::from(token::PLUS)),
            Some(Box::new(Identifier::new(token::Token::new(token::IDENT, String::from("myVar")), String::from("myVar"))))
        );
        assert_eq!(es.to_string(), "+ myVar;");
        let id = Identifier::new(
            token::Token::new(token::IDENT, String::from("myVar")),
            String::from("myVar")
        );
        assert_eq!(id.to_string(), "myVar");
        let ilit = IntegerLiteral::new(
            token::Token::new(token::INT, String::from("5")),
            5
        );
        assert_eq!(ilit.to_string(), "5");
        let mut program = Program::new();
        program.statements.push(Box::new(ls));
        program.statements.push(Box::new(rs));
        program.statements.push(Box::new(es));
        assert_eq!(program.to_string(), "let myVar = anotherVar;\nreturn myVar;\n+ myVar;\n");
    }
}

