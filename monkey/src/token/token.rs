pub type TokenType = &'static str;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub ttype:      TokenType,
    pub literal:    String,
}

impl Token {
    pub fn new(ttype: TokenType, literal: String) -> Token {
        return Token{ ttype: ttype, literal: literal };
    }
}

// Metacharacters
pub static ILLEGAL:     TokenType       = "ILLEGAL";    // unrecognized character
pub static EOF:         TokenType       = "EOF";        // end-of-file

// Identifiers + Literals
pub static IDENT:       TokenType       = "IDENT";      // add, foobar, x, y, ...
pub static INT:         TokenType       = "INT";        // 1343456
pub static LET:         TokenType       = "let";
pub static FUNCTION:    TokenType       = "fn";

// Operators
pub static ASSIGN:      TokenType       = "=";
pub static PLUS:        TokenType       = "+";

// Delimiters
pub static COMMA:       TokenType       = ",";
pub static SEMICOLON:   TokenType       = ";";

// Collections + Scopes
pub static LPAREN:      TokenType       = "(";
pub static RPAREN:      TokenType       = ")";
pub static LBRACE:      TokenType       = "{";
pub static RBRACE:      TokenType       = "}";

pub fn lookup_ident(ident: &String) -> TokenType {
    match ident.as_str() {
        "fn"        => FUNCTION,
        "let"       => LET,
        _           => IDENT,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_ident() {
        assert_eq!(lookup_ident(&String::from("fn")), FUNCTION);
        assert_eq!(lookup_ident(&String::from("let")), LET);
    }
}

