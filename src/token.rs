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
pub enum TokenType {

    ILLEGAL,
    EOF,
    
    // Identifiers + literals
    IDENT, // add, foobar, x, y, ...
    INT, // 1343456

    // Operators
    ASSIGN,
    PLUS,

    // Delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

}


pub struct Token {

    // #[serde(rename = "type")]
    kind: TokenType,
    literal: String,

}

#[test]

fn test() {

    

}