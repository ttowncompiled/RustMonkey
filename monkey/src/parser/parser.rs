use ast::*;
use lexer::*;
use token::*;

#[derive(PartialEq)]
enum Precedence {
    LOWEST,
    EQUALS,         // ==
    LESSGREATER,    // > or <
    SUM,            // +
    PRODUCT,        // *
    PREFIX,         // -X or !X
    CALL,           // myFunction(X)
}

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

    pub fn precedences(&mut self, ttype: token::TokenType) -> i32 {
        return match ttype {
            "==" => Precedence::EQUALS as i32,
            "!=" => Precedence::EQUALS as i32,
            "<" => Precedence::LESSGREATER as i32,
            ">" => Precedence::LESSGREATER as i32,
            "+" => Precedence::SUM as i32,
            "-" => Precedence::SUM as i32,
            "/" => Precedence::PRODUCT as i32,
            "*" => Precedence::PRODUCT as i32,
            "(" => Precedence::CALL as i32,
            _ => Precedence::LOWEST as i32,
        };
    }

    pub fn cur_precedence(&mut self) -> i32 {
        return match self.cur_token.as_ref().cloned() {
            Some(tok) => self.precedences(tok.ttype),
            None => Precedence::LOWEST as i32,
        };
    }

    pub fn peek_precedence(&mut self) -> i32 {
        return match self.peek_token.as_ref().cloned() {
            Some(tok) => self.precedences(tok.ttype),
            None => Precedence::LOWEST as i32,
        };
    }

    pub fn prefix_parse_fns(&mut self, ttype: token::TokenType) -> Option<Box<ast::Expression>> {
        return match ttype {
            "IDENT" => self.parse_identifier(),
            "INT" => self.parse_integer_literal(),
            "!" => self.parse_prefix_expression(),
            "-" => self.parse_prefix_expression(),
            _ => None,
        };
    }

    pub fn infix_parse_fns(&mut self, ttype: token::TokenType) -> bool {
        return match ttype {
            "+" =>  true,
            "-" =>  true,
            "/" =>  true,
            "*" =>  true,
            "==" => true,
            "!=" => true,
            "<" =>  true,
            ">" =>  true,
            _ =>    false,
        };
    }

    pub fn parse_identifier(&mut self) -> Option<Box<ast::Expression>> {
        return match self.cur_token.as_ref().cloned() {
            Some(tok) => Some(Box::new(ast::Identifier::new(tok.clone(), tok.literal.clone()))),
            None => None,
        };
    }

    pub fn parse_integer_literal(&mut self) -> Option<Box<ast::Expression>> {
        return match self.cur_token.as_ref().cloned() {
            Some(tok) => match tok.literal.parse::<i64>() {
                Ok(val) => Some(Box::new(ast::IntegerLiteral::new(tok, val))),
                Err(err) => {
                    self.errors.push(err.to_string());
                    None
                },
            },
            None => None,
        };
    }

    pub fn peek_error(&mut self, ttype: token::TokenType) {
        match self.peek_token.as_ref().cloned() {
            Some(tok) => self.errors.push(format!("expected next token to be {}, got {} instead", ttype, tok.ttype)),
            None => self.errors.push(format!("expected next token to be {}, got None instead", ttype)),
        }
    }

    pub fn no_prefix_parse_fn_error(&mut self) {
        match self.cur_token.as_ref().cloned() {
            Some(tok) => self.errors.push(format!("no prefix parse function for {} found", tok.ttype)),
            None => self.errors.push(format!("no prefix parse function for None found")),
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
                    return self.parse_let_statement();
                } else if tok.ttype == token::RETURN {
                    return self.parse_return_statement();
                } else {
                    return self.parse_expression_statement();
                }
            },
            None => return None,
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<Box<ast::Statement>> {
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
                        return Some(Box::new(ast::LetStatement::new(token, name, None)));
                    }
                },
                None => (),
            }
            self.next_token();
        }

        return None;
    }

    pub fn parse_return_statement(&mut self) -> Option<Box<ast::Statement>> {
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
                        return Some(Box::new(ast::ReturnStatement::new(token, None)));
                    }
                },
                None => (),
            }
            self.next_token();
        }

        return None;
    }

    pub fn parse_expression_statement(&mut self) -> Option<Box<ast::Statement>> {
        let token: token::Token;
        match self.cur_token.as_ref().cloned() {
            Some(tok) => token = tok,
            None => return None,
        }

        let expression: Option<Box<ast::Expression>> = self.parse_expression(Precedence::LOWEST as i32);

        if self.peek_token_is(token::SEMICOLON) {
            self.next_token();
        }

        return Some(Box::new(ast::ExpressionStatement::new(token, expression)));
    }

    pub fn parse_expression(&mut self, precedence: i32) -> Option<Box<ast::Expression>> {
        let prefix: Option<Box<ast::Expression>> = match self.cur_token.as_ref().cloned() {
            Some(tok) => self.prefix_parse_fns(tok.ttype),
            None => None,
        };

        if prefix.is_none() {
            self.no_prefix_parse_fn_error();
            return None;
        }

        let mut leftExp: Option<Box<ast::Expression>> = prefix;

        while ! self.peek_token_is(token::SEMICOLON) && precedence < self.peek_precedence() {
            let flag: bool = match self.peek_token.as_ref().cloned() {
                Some(tok) => self.infix_parse_fns(tok.ttype),
                None => false,
            };

            if ! flag {
                return leftExp;
            }

            self.next_token();

            leftExp = self.parse_infix_expression(leftExp);
        }

        return leftExp;
    }

    pub fn parse_prefix_expression(&mut self) -> Option<Box<ast::Expression>> {
        let token: token::Token;
        match self.cur_token.as_ref().cloned() {
            Some(tok) => token = tok,
            None => return None,
        }

        self.next_token();

        let right: Option<Box<ast::Expression>> = self.parse_expression(Precedence::PREFIX as i32);

        return Some(Box::new(ast::PrefixExpression::new(token.clone(), token.literal.clone(), right)));
    }

    pub fn parse_infix_expression(&mut self, left: Option<Box<ast::Expression>>) -> Option<Box<ast::Expression>> {
        let token: token::Token;
        match self.cur_token.as_ref().cloned() {
            Some(tok) => token = tok,
            None => return None,
        }

        let precedence: i32 = self.cur_precedence();
        self.next_token();
        let right: Option<Box<ast::Expression>> = self.parse_expression(precedence);

        return Some(Box::new(ast::InfixExpression::new(token.clone(), left, token.literal.clone(), right)));
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

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";

        let mut l = lexer::Lexer::new(input);
        let mut p = Parser::new(&mut l);

        let program = p.parse_program();
        check_parser_errors(&p);

        match program {
            Some(prog) => {
                if prog.statements.len() != 1 {
                    assert!(false, "program.statements does not contain {} statements, got={}", 1, prog.statements.len());
                }
                assert_eq!((*prog.statements[0]).token_literal(), "foobar", "tests[{}]", 0);
            },
            None => assert!(false, "parse_program() returns None"),
        }
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";

        let mut l = lexer::Lexer::new(input);
        let mut p = Parser::new(&mut l);

        let program = p.parse_program();
        check_parser_errors(&p);

        match program {
            Some(prog) => {
                if prog.statements.len() != 1 {
                    assert!(false, "program.statements does not contain {} statements, got={}", 1, prog.statements.len());
                }
                assert_eq!((*prog.statements[0]).token_literal(), "5", "tests[{}]", 0);
            },
            None => assert!(false, "parse_program() returns None"),
        }
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        let input = "
!5;
-5";

        let mut l = lexer::Lexer::new(input);
        let mut p = Parser::new(&mut l);

        let program = p.parse_program();
        check_parser_errors(&p);

        match program {
            Some(prog) => {
                if prog.statements.len() != 2 {
                    assert!(false, "program.statements does not contain {} statements, got={}", 2, prog.statements.len());
                }
                assert_eq!((*prog.statements[0]).to_string(), "(!5);", "tests[{}]", 0);
                assert_eq!((*prog.statements[1]).to_string(), "(-5);", "tests[{}]", 1);
            },
            None => assert!(false, "parse_program() returns None"),
        }
    }

    #[test]
    fn test_parsing_infix_expressions() {
        let infix_tests: [String; 8] = [
            String::from("5 + 5;"),
            String::from("5 - 5;"),
            String::from("5 * 5;"),
            String::from("5 / 5;"),
            String::from("5 > 5;"),
            String::from("5 < 5;"),
            String::from("5 == 5;"),
            String::from("5 != 5;")
        ];

        let mut i = 0;
        for input in infix_tests.iter() {
            let mut l = lexer::Lexer::new(input);
            let mut p = Parser::new(&mut l);

            let program = p.parse_program();
            check_parser_errors(&p);

            match program {
                Some(prog) => {
                    if prog.statements.len() != 1 {
                        assert!(false, "program.statements does not contain {} statements, got={}", 1, prog.statements.len());
                    }
                    assert_eq!((*prog.statements[0]).to_string(), format!("({});", &(*input)[..((*input).len()-1)]), "tests[{}]", i);
                },
                None => assert!(false, "parse_program() returns None"),
            }

            i += 1;
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

