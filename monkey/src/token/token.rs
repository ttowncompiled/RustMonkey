pub type TokenType = &'static str;

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub ttype:      TokenType,
    pub literal:    &'a str,
}

pub static ILLEGAL: &'static str        = "ILLEGAL";
pub static EOF: &'static str            = "EOF";

pub static IDENT: &'static str          = "IDENT";
pub static INT: &'static str            = "INT";

pub static ASSIGN: &'static str         = "=";
pub static PLUS: &'static str           = "+";

pub static COMMA: &'static str          = ",";
pub static SEMICOLON: &'static str      = ";";

pub static LPAREN: &'static str         = "(";
pub static RPAREN: &'static str         = ")";
pub static LBRACE: &'static str         = "{";
pub static RBRACE: &'static str         = "}";

