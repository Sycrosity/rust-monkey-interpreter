#![allow(dead_code)]

#[derive(Debug, PartialEq)]
pub enum Token {

    Illegal, // anything else - e.g. "£"
    EndOfFile,
    
    //Identifiers + literals
    Identifier(String), //e.go foo, bar, x, y
    Integer(String), //1343456, 7, 34
    
    //Operators
    Assign, //"="
    Plus, //"+"
    Minus, //"-"
    Bang, //"!"
    Asterisk, //"*"
    Slash, //"/"
    LessThan, //"<"
    GreaterThan, //">"
    Equal, //"=="
    NotEqual, //"!="
    
    //Delimiters
    Comma, //"
    Semicolon, //";"
    LeftParenthesis, //"("
    RightParenthesis, //")"
    LeftBrace, //"{"
    RightBrace, //"}"
    
    //Keywords
    Function, //"fn"
    Let, //"let"
    True, //"true"
    False, //"false"
    If, //"if"
    Else, //"else"
    Return, //"return"
}

pub fn lookup_ident(ident: &str) -> Token {

    match ident {

        "fn" => Token::Function,
        "let" => Token::Let,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        ident @ _ => Token::Identifier(ident.to_string())

    }
}

#[test]
fn test() {

    

}