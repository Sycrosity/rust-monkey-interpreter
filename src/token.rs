#![allow(dead_code)]

/*
impl TokenType {
    pub fn kind(token_type: TokenType) -> TokenType {
        match token_type {
            ""
        }
    }       
}
*/

#[derive(Debug, PartialEq)]
pub enum Token {

    ILLEGAL,
    EOF,
    
    //Identifiers + literals
    IDENT(String), //add, foobar, x, y, ...
    INT(String), //1343456

    //Operators
    ASSIGN,
    PLUS,

    //Delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    //Keywords
    FUNCTION,
    LET,
}

pub fn lookup_ident(ident: &str) -> Token {

    match ident {

        "fn" => Token::FUNCTION,
        "let" => Token::LET,
        ident @ _ => Token::IDENT(ident.to_string())

    }
}

#[test]
fn test() {

    

}