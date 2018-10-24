use std::str::Chars;
use token::*;

pub struct Lexer<'a> {
    pub input:          &'a str,
    pub chars:          Chars<'a>,
    pub position:       i32,            // current position in input (points to current char)
    pub read_position:  i32,            // current reading position in input (after current char)
    pub ch:             Option<char>,   // current char under examination
}

pub fn new<'a>(input: &'a str) -> Lexer<'a> {
    let mut l = Lexer{
        input:          input,
        chars:          input.chars(),
        position:       0,
        read_position:  0,
        ch:             None,
    };
    l.read_char();
    return l;
}

impl<'a> Lexer<'a> {
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() as i32 {
            self.ch = None;
        } else {
            self.ch = self.chars.next();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Option<token::Token> {
        let tok: Option<token::Token>;

        self.skip_whitespace();

        match self.ch {
            Some('=') => tok = Some(token::Token::new(token::ASSIGN,        '='.to_string())),
            Some('+') => tok = Some(token::Token::new(token::PLUS,          '+'.to_string())),
            Some(',') => tok = Some(token::Token::new(token::COMMA,         ','.to_string())),
            Some(';') => tok = Some(token::Token::new(token::SEMICOLON,     ';'.to_string())),
            Some('(') => tok = Some(token::Token::new(token::LPAREN,        '('.to_string())),
            Some(')') => tok = Some(token::Token::new(token::RPAREN,        ')'.to_string())),
            Some('{') => tok = Some(token::Token::new(token::LBRACE,        '{'.to_string())),
            Some('}') => tok = Some(token::Token::new(token::RBRACE,        '}'.to_string())),
            Some(_) => {
                if is_letter(self.ch) {
                    let ident: String = self.read_identifier();
                    let ttype: token::TokenType = token::lookup_ident(&ident);
                    tok = Some(token::Token{ ttype: ttype, literal: ident });
                    return tok;
                } else if is_digit(self.ch) {
                    let ident: String = self.read_number();
                    let ttype: token::TokenType = token::INT;
                    tok = Some(token::Token{ ttype: ttype, literal: ident });
                    return tok;
                } else {
                    match self.ch {
                        Some(ch) => tok = Some(token::Token{ ttype: token::ILLEGAL, literal: ch.to_string() }),
                        None => tok = None,
                    }
                }
            },
            None => tok = None,
        }

        self.read_char();

        return tok;
    }

    pub fn read_identifier(&mut self) -> String {
        let mut buffer: String = String::new();
        while is_letter(self.ch) {
            match self.ch {
                Some(ch) => buffer.push(ch),
                None => return buffer,
            }
            self.read_char();
        }
        return buffer;
    }

    pub fn read_number(&mut self) -> String {
        let mut buffer: String = String::new();
        while is_digit(self.ch) {
            match self.ch {
                Some(ch) => buffer.push(ch),
                None => return buffer,
            }
            self.read_char();
        }
        return buffer;
    }

    pub fn skip_whitespace(&mut self) {
        while is_whitespace(self.ch) {
            self.read_char();
        }
    }
}

pub fn is_letter(ch: Option<char>) -> bool {
    match ch {
        Some(ord) => return 'a' <= ord && ord <= 'z' || 'A' <= ord && ord <= 'Z' || ord == '_',
        None => return false,
    }
}

pub fn is_digit(ch: Option<char>) -> bool {
    match ch {
        Some(ord) => return '0' <= ord && ord <= '9',
        None => return false,
    }
}

pub fn is_whitespace(ch: Option<char>) -> bool {
    match ch {
        Some(ord) => return ord == ' ' || ord == '\t' || ord == '\n' || ord == '\r',
        None => return false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
";

        let tests = [
            token::Token{ ttype:    token::LET,         literal:   String::from("let")         },
            token::Token{ ttype:    token::IDENT,       literal:   String::from("five")        },
            token::Token{ ttype:    token::ASSIGN,      literal:   String::from("=")           },
            token::Token{ ttype:    token::INT,         literal:   String::from("5")           },
            token::Token{ ttype:    token::SEMICOLON,   literal:   String::from(";")           },
            token::Token{ ttype:    token::LET,         literal:   String::from("let")         },
            token::Token{ ttype:    token::IDENT,       literal:   String::from("ten")         },
            token::Token{ ttype:    token::ASSIGN,      literal:   String::from("=")           },
            token::Token{ ttype:    token::INT,         literal:   String::from("10")          },
            token::Token{ ttype:    token::SEMICOLON,   literal:   String::from(";")           },
            token::Token{ ttype:    token::LET,         literal:   String::from("let")         },
            token::Token{ ttype:    token::IDENT,       literal:   String::from("add")         },
            token::Token{ ttype:    token::ASSIGN,      literal:   String::from("=")           },
            token::Token{ ttype:    token::FUNCTION,    literal:   String::from("fn")          },
            token::Token{ ttype:    token::LPAREN,      literal:   String::from("(")           },
            token::Token{ ttype:    token::IDENT,       literal:   String::from("x")           },
            token::Token{ ttype:    token::COMMA,       literal:   String::from(",")           },
            token::Token{ ttype:    token::IDENT,       literal:   String::from("y")           },
            token::Token{ ttype:    token::RPAREN,      literal:   String::from(")")           },
            token::Token{ ttype:    token::LBRACE,      literal:   String::from("{")           },
            token::Token{ ttype:    token::IDENT,       literal:   String::from("x")           },
            token::Token{ ttype:    token::PLUS,        literal:   String::from("+")           },
            token::Token{ ttype:    token::IDENT,       literal:   String::from("y")           },
            token::Token{ ttype:    token::SEMICOLON,   literal:   String::from(";")           },
            token::Token{ ttype:    token::RBRACE,      literal:   String::from("}")           },
            token::Token{ ttype:    token::SEMICOLON,   literal:   String::from(";")           },
            token::Token{ ttype:    token::LET,         literal:   String::from("let")         },
            token::Token{ ttype:    token::IDENT,       literal:   String::from("result")      },
            token::Token{ ttype:    token::ASSIGN,      literal:   String::from("=")           },
            token::Token{ ttype:    token::IDENT,       literal:   String::from("add")         },
            token::Token{ ttype:    token::LPAREN,      literal:   String::from("(")           },
            token::Token{ ttype:    token::IDENT,       literal:   String::from("five")        },
            token::Token{ ttype:    token::COMMA,       literal:   String::from(",")           },
            token::Token{ ttype:    token::IDENT,       literal:   String::from("ten")         },
            token::Token{ ttype:    token::RPAREN,      literal:   String::from(")")           },
            token::Token{ ttype:    token::SEMICOLON,   literal:   String::from(";")           },
        ];

        let mut l = new(&input);

        let mut i = 0;
        for test in tests.iter() {
            match l.next_token() {
                Some(tok) => assert_eq!(&tok, test, "tests[{}]", i),
                None => assert!(false),
            }
            i += 1;
        }
    }
}

