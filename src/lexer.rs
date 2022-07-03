#![allow(dead_code)]

use std::{iter::Peekable, str::Chars};

use crate::token::Token;

//a lexer that takes an input and returns the tokenised version of the input
pub struct Lexer<'a> {
    //chars - we need to iterate over each character in the input, so we make it into a chars list.
    //peekable - we need to be able to look into the future at what character is next, so we make it a peekable iterator.
    //this cuts out most of the work that the go version of this has to do.
    //[LEARN] needs a lifetime - not entirely sure why, but it won't work without it
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    //generates a new lexer with the correct type
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
        }
    }

    //returns either the next char, or a None - if its a None, we have iterated the input past the final line so it should return an EOF - the go tutorial does this by checking if its a blank byte, we do it by making each char an Option, and using a peekable chars list.
    //[TODO] doesn't work with UTF8 encoding - fix in future!
    pub fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    pub fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    pub fn peek_char_eq(&mut self, eq: char) -> bool {
        match self.peek_char() {
            Some(&ch) => eq == ch,
            None => false,
        }
    }

    //checks if the char ahead is a letter - None or bool
    pub fn peek_is_letter(&mut self) -> bool {
        match self.peek_char() {
            Some(&ch) => is_letter(ch),
            None => false,
        }
    }

    //checks if the char ahead is numeric - None or bool
    pub fn peek_is_number(&mut self) -> bool {
        match self.peek_char() {
            Some(&ch) => is_number(ch),
            None => false,
        }
    }

    //peeks at the char ahead, and if its a valid letter (or an _) adds it to a string - this repeats until an invalid char is found, then returns the string.
    //[TODO?] - make it return a string slice instead? (benchmark)
    fn read_identifier(&mut self, ch: char) -> String {
        let mut res: String = String::from(ch);

        while self.peek_is_letter() {
            res.push(self.read_char().unwrap())
        }
        res
    }

    //peeks at the char ahead, and until
    fn read_number(&mut self, ch: char) -> String {
        let mut res: String = String::from(ch);

        while self.peek_is_number() {
            res.push(self.read_char().unwrap())
        }
        res
    }

    //the lexer should ignore all whitespace, as it shouldn't matter (except in checking for identifers, where it doens't use this function)
    pub fn skip_whitespace(&mut self) {
        while let Some(&peek) = self.peek_char() {
            //no whitespace is more common, so that should be checked first
            if !peek.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    //returns the next token from the lexer - e.g. "=" => Token::Assign, "five" => Token::Identifier("five")
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok: Option<char> = self.read_char();

        //matches the next symbol (as a Some(char)) to its token - None is the EOF
        match tok {
            Some('=') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('!') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            Some('*') => Token::Asterisk,
            Some('/') => Token::Slash,
            Some('<') => Token::LessThan,
            Some('>') => Token::GreaterThan,
            Some(',') => Token::Comma,
            Some(';') => Token::Semicolon,
            Some('(') => Token::LeftParenthesis,
            Some(')') => Token::RightParenthesis,
            Some('{') => Token::LeftBrace,
            Some('}') => Token::RightBrace,
            // Some('') => Token::,

            //catches all other options - must be an integer or an identifier - else, its an illegal token.
            Some(ch) => {
                if is_letter(ch) {
                    let literal: String = self.read_identifier(ch);
                    crate::token::lookup_ident(&literal)
                } else if is_number(ch) {
                    Token::Integer(self.read_number(ch))
                } else {
                    Token::Illegal
                }
            }
            None => Token::EndOfFile,
        }
    }
}

//checks if a char is an accepted identifier character - edit this function to change what can be in an identifer
fn is_letter(ch: char) -> bool {
    //[TODO] add other valid identifier chars (e.g. numbers that aren't the first char)
    ch.is_alphabetic() || ch == '_'
}

//checks if a char is an accepted integer character - edit this function to change what can be in an integer
fn is_number(ch: char) -> bool {
    //[TODO] add hex, oct, ect
    ch.is_numeric()
}

//[TODO] add more lexer tests
//tests the tokens against a pre-written list of tokens that it should equal to
#[test]
fn test_next_token() {
    let input = "let five = 5;
        let ten = 10;

        let add = fn(x, y) {
          x + y
        };
        
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;
        
        if (5 < 10) {
            return true;
        } else {
            return false;
        }
        
        10 == 10;
        10 != 9;";

    let tests: Vec<Token> = vec![
        Token::Let,
        Token::Identifier("five".to_string()),
        Token::Assign,
        Token::Integer("5".to_string()),
        Token::Semicolon,
        Token::Let,
        Token::Identifier("ten".to_string()),
        Token::Assign,
        Token::Integer("10".to_string()),
        Token::Semicolon,
        Token::Let,
        Token::Identifier("add".to_string()),
        Token::Assign,
        Token::Function,
        Token::LeftParenthesis,
        Token::Identifier("x".to_string()),
        Token::Comma,
        Token::Identifier("y".to_string()),
        Token::RightParenthesis,
        Token::LeftBrace,
        Token::Identifier("x".to_string()),
        Token::Plus,
        Token::Identifier("y".to_string()),
        Token::RightBrace,
        Token::Semicolon,
        Token::Let,
        Token::Identifier("result".to_string()),
        Token::Assign,
        Token::Identifier("add".to_string()),
        Token::LeftParenthesis,
        Token::Identifier("five".to_string()),
        Token::Comma,
        Token::Identifier("ten".to_string()),
        Token::RightParenthesis,
        Token::Semicolon,
        Token::Bang,
        Token::Minus,
        Token::Slash,
        Token::Asterisk,
        Token::Integer("5".to_string()),
        Token::Semicolon,
        Token::Integer("5".to_string()),
        Token::LessThan,
        Token::Integer("10".to_string()),
        Token::GreaterThan,
        Token::Integer("5".to_string()),
        Token::Semicolon,
        Token::If,
        Token::LeftParenthesis,
        Token::Integer("5".to_string()),
        Token::LessThan,
        Token::Integer("10".to_string()),
        Token::RightParenthesis,
        Token::LeftBrace,
        Token::Return,
        Token::True,
        Token::Semicolon,
        Token::RightBrace,
        Token::Else,
        Token::LeftBrace,
        Token::Return,
        Token::False,
        Token::Semicolon,
        Token::RightBrace,
        Token::Integer("10".to_string()),
        Token::Equal,
        Token::Integer("10".to_string()),
        Token::Semicolon,
        Token::Integer("10".to_string()),
        Token::NotEqual,
        Token::Integer("9".to_string()),
        Token::Semicolon,
        Token::EndOfFile,
    ];

    let mut lex: Lexer = Lexer::new(input);
    tests.into_iter().for_each(|test| {
        let token: Token = lex.next_token();
        assert_eq!(token, test);
    });
}

//a test to verbosely check that code is being tokenised correctly by the lexer
#[test]
fn visible_test_token() {
    let input: &str = "let five = 5;
    let ten = 10;

    let add = fn(x, y) {
        x + y
    };
    
    let result = add(five, ten);
    !-/*5;
    5 < 10 > 5;
    
    if (5 < 10) {
        return true;
    } else {
        return false;
    }

    10 == 10;
    10 != 9;";

    let mut lex: Lexer = Lexer::new(input);
    while let Some(x) = Some(lex.next_token()) {
        if x != Token::EndOfFile {
            println!("{:?}", x);
        } else {
            break;
        }
    }
}
