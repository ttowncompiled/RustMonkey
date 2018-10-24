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
        match self.ch {
            Some('=') => tok = Some(token::Token{ ttype: token::ASSIGN,         literal: "=" }),
            Some('+') => tok = Some(token::Token{ ttype: token::PLUS,           literal: "+" }),
            Some(',') => tok = Some(token::Token{ ttype: token::COMMA,          literal: "," }),
            Some(';') => tok = Some(token::Token{ ttype: token::SEMICOLON,      literal: ";" }),
            Some('(') => tok = Some(token::Token{ ttype: token::LPAREN,         literal: "(" }),
            Some(')') => tok = Some(token::Token{ ttype: token::RPAREN,         literal: ")" }),
            Some('{') => tok = Some(token::Token{ ttype: token::LBRACE,         literal: "{" }),
            Some('}') => tok = Some(token::Token{ ttype: token::RBRACE,         literal: "}" }),
            Some(_) => tok = None,
            None => tok = None,
        }
        self.read_char();
        return tok;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let tests = [
            token::Token{ ttype:    token::ASSIGN,          literal:   "=" },
            token::Token{ ttype:    token::PLUS,            literal:   "+" },
            token::Token{ ttype:    token::LPAREN,          literal:   "(" },
            token::Token{ ttype:    token::RPAREN,          literal:   ")" },
            token::Token{ ttype:    token::LBRACE,          literal:   "{" },
            token::Token{ ttype:    token::RBRACE,          literal:   "}" },
            token::Token{ ttype:    token::COMMA,           literal:   "," },
            token::Token{ ttype:    token::SEMICOLON,       literal:   ";" },
        ];

        let mut l = new(&input);

        for test in tests.iter() {
            match l.next_token() {
                Some(tok) => assert_eq!(&tok, test),
                None => assert!(false),
            }
        }
    }
}

