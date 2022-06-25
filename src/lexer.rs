#![allow(dead_code)]

use std::{iter::Peekable, str::Chars};

use crate::token::Token;

pub struct Lexer<'a> {

    input: Peekable<Chars<'a>>,
    /*
    position: usize, // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: Option<char> // current char under examination
    */
    
}

impl<'a> Lexer<'a> {

    //generates a new lexer - Self refers to the lexer strut, and unlike in the 
    pub fn new(input: &'a str) -> Self { 
        Self { 
            input: input.chars().peekable(),
            /* 
            position: 0, 
            read_position: 0, 
            ch: input.chars().nth(0)
            */
        }
    }

    //returns either the next char, or a None - if its a None, we have iterated the input past the
    //final line so it should return an EOF - the go tutorial does this by checking if its a blank
    //byte, we do it by making each char an Option, and using a peekable chars list.
    //[issue] - doesn't work with UTF8 encoding - fix in future!
    pub fn read_char(&mut self) -> Option<char> {

        self.input.next()

    }

    pub fn peek_char(&mut self) -> Option<&char> {

        self.input.peek()

    }

    //checks if the char ahead is a letter
    pub fn peek_is_letter(&mut self) -> bool {

        match self.peek_char() {
                
            Some(&ch) => is_letter(ch),
            None => false
        }
    }

    
    // peeks at the letter ahead, and if its a valid letter (or _) adds it to a string - repeats until an invalid char is found, then returns the string.
    // TODO? - make it return a string slice instead?
    fn read_identifier(&mut self, ch: char) -> String {

        let mut res: String = String::from(ch);

        while self.peek_is_letter() {

            res.push(self.read_char().unwrap())
        }
        res
    }

    fn read_number(&mut self, ch: char) -> String {

        let mut number = String::from(ch);

        while let Some(&peek) = self.peek_char() {

            if !peek.is_numeric() {
                break;
            }
            number.push(self.read_char().unwrap());
        }
        number
    }

    fn skip_whitespace(&mut self) {

        while let Some(&peek) = self.peek_char() {

            //no whitespace is more common, so that should be checked first
            if !peek.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    /*
    func (l *Lexer) readIdentifier() string {
        position := l.position
        for isLetter(l.ch) {
            l.readChar()
    }
    */

    //
    pub fn next_token(&mut self) -> Token {

        self.skip_whitespace();

        let tok: Option<char> = self.read_char();

        match tok {
            
            Some('=') => Token::ASSIGN,
            Some(';') => Token::SEMICOLON,
            Some('(') => Token::LPAREN,
            Some(')') => Token::RPAREN,
            Some(',') => Token::COMMA,
            Some('+') => Token::PLUS,
            Some('{') => Token::LBRACE,
            Some('}') => Token::RBRACE,
            // Some('') => Token::,
            
            //catches all other options - must be a keyword, an identifier or illegal
            Some(ch @ _) => {
                if is_letter(ch) {

                    let literal = self.read_identifier(ch);
                    crate::token::lookup_ident(&literal)

                } else if ch.is_numeric() {

                    Token::INT(self.read_number(ch))

                } else {

                    Token::ILLEGAL

                }
            },
            None => Token::EOF,
        }
    }
}

fn is_letter(ch: char) -> bool {

    ch.is_alphabetic() || ch == '_'
}

//TODO - add tests for everything

//tests the tokens against a pre-written list of tokens that it should equal to
#[test]
fn test_next_token() {

    let input =
        "let five = 5;
        let ten = 10;
        let add = fn(x, y) {
        x + y;
        };
        let result = add(five, ten);";

    let tests: Vec<Token> = vec![

        Token::LET,
        Token::IDENT("five".to_string()),
        Token::ASSIGN,
        Token::INT("5".to_string()),
        Token::SEMICOLON,
        Token::LET,
        Token::IDENT("ten".to_string()),
        Token::ASSIGN,
        Token::INT("10".to_string()),
        Token::SEMICOLON,
        Token::LET,
        Token::IDENT("add".to_string()),
        Token::ASSIGN,
        Token::FUNCTION,
        Token::LPAREN,
        Token::IDENT("x".to_string()),
        Token::COMMA,
        Token::IDENT("y".to_string()),
        Token::RPAREN,
        Token::LBRACE,
        Token::IDENT("x".to_string()),
        Token::PLUS,
        Token::IDENT("y".to_string()),
        Token::SEMICOLON,
        Token::RBRACE,
        Token::SEMICOLON,
        Token::LET,
        Token::IDENT("result".to_string()),
        Token::ASSIGN,
        Token::IDENT("add".to_string()),
        Token::LPAREN,
        Token::IDENT("five".to_string()),
        Token::COMMA,
        Token::IDENT("ten".to_string()),
        Token::RPAREN,
        Token::SEMICOLON,
        Token::EOF,

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

    let input: &str = 
    "let x = 5;
    let y = 2;
    fn double_add(a, b) {
        a + b + a + b
    }
    let res = double_add(x, y);";

    let mut lex: Lexer = Lexer::new(input);
    while let Some(x) = Some(lex.next_token()) {

        if x != Token::EOF {
            println!("{:?}", x);
        } else {

            break;
        }
    }
}