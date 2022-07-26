#![allow(dead_code)]

//[TODO] - remove Token::EndOfFile and replace with just the None enum to simplify code

//every type of token that could exist in the code, so code can be broken up into chunks - e.g. let i = 2; becomes [Token::Let, Token::Identifier("i"), Token::Assign, Token::Integer("2"), Token::SemiColon, Token::EndOfFile]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token<'source> {
    Illegal,   // anything else - e.g. "£"
    EndOfFile, //no more code ""

    //Identifiers + literals
    Identifier(&'source str), //e.g. "foo", "bar", "x", "y"
    Integer(i32),             //"1343456", "7", "34"

    //Operators
    Assign,      //"="
    Plus,        //"+"
    Minus,       //"-"
    Bang,        //"!"
    Asterisk,    //"*"
    Slash,       //"/"
    LessThan,    //"<"
    GreaterThan, //">"
    Equal,       //"=="
    NotEqual,    //"!="

    //Delimiters
    Comma,            //","
    Semicolon,        //";"
    LeftParenthesis,  //"("
    RightParenthesis, //")"
    LeftBrace,        //"{"
    RightBrace,       //"}"

    //Keywords
    Function, //"fn"
    Let,      //"let"
    True,     //"true"
    False,    //"false"
    If,       //"if"
    Else,     //"else"
    Return,   //"return"
}

//[TODO] - store each Token as a value and a pos for debuging purposes and for code cleanliness
/*
pub struct Token<'source> {
    pub value: Token<'source>,
    pub pos: usize,
}
*/

pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "fn" => Token::Function,
        "let" => Token::Let,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        ident => Token::Identifier(ident),
    }
}

#[test]
fn test() {
    assert_eq!(lookup_ident("fn"), Token::Function);
    assert_eq!(lookup_ident("test"), Token::Identifier("test"));
}
