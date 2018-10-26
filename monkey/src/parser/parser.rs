use ast::*;
use lexer::*;
use token::*;

pub struct Parser<'a> {
    pub l:              &'a mut lexer::Lexer<'a>,
    pub errors:         Vec<String>,
    pub cur_token:      Option<token::Token>,
    pub peek_token:     Option<token::Token>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut lexer::Lexer<'a>) -> Parser<'a> {
        let mut p: Parser<'a> = Parser{
            l:              lexer,
            errors:         Vec::new(),
            cur_token:      None,
            peek_token:     None,
        };

        // Read two tokens, so cur_token and peek_token are both set
        p.next_token();
        p.next_token();

        return p;
    }

    pub fn peek_error(&mut self, ttype: token::TokenType) {
        match self.peek_token.as_ref().cloned() {
            Some(tok) => self.errors.push(format!("expected next token to be {}, got {} instead", ttype, tok.ttype)),
            None => self.errors.push(format!("expected next token to be {}, got None instead", ttype)),
        }
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
                } else if tok.ttype == token::RETURN {
                    return match self.parse_return_statement() {
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

    pub fn parse_return_statement(&mut self) -> Option<ast::ReturnStatement> {
        let token: token::Token;
        match self.cur_token.as_ref().cloned() {
            Some(tok) => token = tok,
            None => return None,
        }

        self.next_token();

        // TODO: return ReturnStatement with expression
        while self.cur_token.is_some() {
            match self.cur_token.as_ref().cloned() {
                Some(tok) => {
                    if tok.ttype == token::SEMICOLON {
                        return Some(ast::ReturnStatement::new(token, Box::new(ast::Identifier::new(tok.clone(), tok.literal.clone()))));
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
            self.peek_error(ttype);
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
        check_parser_errors(&p);

        match program {
            Some(prog) => {
                if prog.statements.len() != 3 {
                    assert!(false, "program.statements does not contain {} statements, got={}", 3, prog.statements.len());
                }
                let mut i = 0;
                for stmt in prog.statements.iter() {
                    assert_eq!((**stmt).token_literal(), token::LET, "tests[{}]", i);
                    i += 1;
                }
            },
            None => assert!(false, "parse_program() returns None"),
        }
    }

    #[test]
    fn test_return_statement() {
        let input = "
return 5;
return 10;
return 838383;
";

        let mut l = lexer::Lexer::new(input);
        let mut p = Parser::new(&mut l);

        let program = p.parse_program();
        check_parser_errors(&p);

        match program {
            Some(prog) => {
                if prog.statements.len() != 3 {
                    assert!(false, "program.statements does not contain {} statements, got={}", 3, prog.statements.len());
                }
                let mut i = 0;
                for stmt in prog.statements.iter() {
                    assert_eq!((**stmt).token_literal(), token::RETURN, "tests[{}]", i);
                    i += 1;
                }
            },
            None => assert!(false, "parse_program() returns None"),
        }
    }

    fn check_parser_errors(p: &Parser) {
        if p.errors.len() == 0 {
            return;
        }
        let mut output: String = String::new();
        output.push_str(&format!("\n\nparser has {} errors", p.errors.len()));
        for err in p.errors.iter() {
            output.push_str(&format!("\nparser error: {}", err));
        }
        output.push_str("\n\n");
        assert!(false, output);
    }
}

