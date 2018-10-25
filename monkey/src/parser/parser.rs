use ast::*;
use lexer::*;
use token::*;

pub struct Parser<'a> {
    pub l:              &'a mut lexer::Lexer<'a>,
    pub cur_token:      Option<token::Token>,
    pub peek_token:     Option<token::Token>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut lexer::Lexer<'a>) -> Parser<'a> {
        let mut p: Parser<'a> = Parser{
            l:              lexer,
            cur_token:      None,
            peek_token:     None,
        };

        // Read two tokens, so cur_token and peek_token are both set
        p.next_token();
        p.next_token();

        return p;
    }

    pub fn next_token(&mut self) {
        if self.peek_token.is_some() {
            self.cur_token = self.peek_token.as_ref().cloned();
        } else {
            self.cur_token = None;
        }
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Option<ast::Program> {
        let mut program: ast::Program = ast::Program::new();

        while self.cur_token.is_some() {
            match self.cur_token.as_ref().cloned() {
                Some(tok) => {
                    if tok.ttype != token::EOF {
                        match self.parse_statement() {
                            Some(statement) => program.statements.push(statement),
                            None => (),
                        }
                    }
                },
                None => (),
            }
            self.next_token();
        }

        return Some(program);
    }

    pub fn parse_statement(&mut self) -> Option<Box<ast::Statement>> {
        match self.cur_token.as_ref().cloned() {
            Some(tok) => {
                if tok.ttype == token::LET {
                    return match self.parse_let_statement() {
                        Some(statement) => Some(Box::new(statement)),
                        None => None,
                    }
                } else {
                    return None;
                }
            },
            None => return None,
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<ast::LetStatement> {
        let token: token::Token;
        match self.cur_token.as_ref().cloned(){
            Some(tok) => token = tok,
            None => return None,
        }

        if ! self.expect_peek(token::IDENT) {
            return None;
        }

        let name: ast::Identifier;
        match self.cur_token.as_ref().cloned() {
            Some(tok) => name = ast::Identifier::new(tok.clone(), tok.literal.clone()),
            None => return None,
        }

        if ! self.expect_peek(token::ASSIGN) {
            return None;
        }

        // TODO: return LetStatement with expression
        while self.cur_token.is_some() {
            match self.cur_token.as_ref().cloned() {
                Some(tok) => {
                    if tok.ttype == token::SEMICOLON {
                        return Some(ast::LetStatement::new(token, name, Box::new(ast::Identifier::new(tok.clone(), tok.literal.clone()))));
                    }
                },
                None => (),
            }
            self.next_token();
        }

        return None;
    }

    pub fn expect_peek(&mut self, ttype: token::TokenType) -> bool {
        if self.peek_token_is(ttype) {
            self.next_token();
            return true;
        } else {
            return false;
        }
    }

    pub fn peek_token_is(&self, ttype: token::TokenType) -> bool {
        match self.peek_token.as_ref().cloned() {
            Some(tok) => return tok.ttype == ttype,
            None => return false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_statement() {
        let input = "
let x = 5;
let y = 10;
let foobar = 838383;
";

        let mut l = lexer::Lexer::new(input);
        let mut p = Parser::new(&mut l);

        let program = p.parse_program();
        if program.is_none() {
            assert!(false, "parse_program() returns None");
        }
    }
}

