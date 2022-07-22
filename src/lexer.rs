#![allow(dead_code)]

use std::{iter::Peekable, str::CharIndices};

use crate::token::Token;

//[TODO?] - add proper documentation?

//a lexer that borrows an input and returns the tokenised version of the input
pub struct Lexer<'source> {
    //charIndices - we need to iterate over each character in the input and see what index it is, so we make it into a charIndices list.
    //peekable - we need to be able to look into the future at what character is next, so we make it a peekable iterator.
    //this cuts out most of the work that the go version of this has to do.
    //[TODO?] - make a new version of charIndices that is a struct so instead of having to do tok.1, you can do tok.value?
    input: &'source str,
    iter: Peekable<CharIndices<'source>>,
}

//so the parser can peek over the lexers list of tokens
impl<'source> Iterator for Lexer<'source> {
    type Item = Token<'source>;

    fn next(&mut self) -> Option<Self::Item> {
        let tok: Token = self.next_token();

        if tok != Token::EndOfFile {
            Some(tok)
        } else {
            None
        }
    }
}

//all of the functions that a lexer needs to have to tokenise an input
impl<'source> Lexer<'source> {
    //generates a new lexer with the correct types
    pub fn new(input: &'source str) -> Self {
        Self {
            input,
            iter: input.char_indices().peekable(),
        }
    }

    //returns either the next char, or a None - if its a None, we have iterated the input past the final line so it should return an EOF - the go tutorial does this by checking if its a blank byte, we do it by making each char an Option, and using a peekable chars list.
    //[TODO] doesn't work with UTF8 encoding - fix in future!
    fn read_char(&mut self) -> Option<(usize, char)> {
        self.iter.next()
    }

    fn peek_char(&mut self) -> Option<&(usize, char)> {
        self.iter.peek()
    }

    //checks if the char inputed is equal to the next peeked char
    fn peek_char_eq(&mut self, eq: char) -> bool {
        match self.peek_char() {
            Some(&ch) => eq == ch.1,
            None => false,
        }
    }

    //checks if the char ahead is a letter - None or bool
    fn peek_is_letter(&mut self) -> bool {
        match self.peek_char() {
            Some(&ch) => is_letter(ch.1),
            None => false,
        }
    }

    //checks if the char ahead is numeric - None or bool
    fn peek_is_number(&mut self) -> bool {
        match self.peek_char() {
            Some(&ch) => is_number(ch.1),
            None => false,
        }
    }

    //peeks at the char ahead, and if its a letter, adds one to an iterator and skips to the next char - returns a slice of the original input as the output
    //[TODO] - intergrate index as a return from self.read_char(), so an extra index isn't needed
    fn read_identifier(&mut self, tok: (usize, char)) -> &'source str {
        let startpos = tok.0;
        let mut index = startpos + 1;
        while self.peek_is_letter() {
            index += 1;
            self.read_char();
        }

        //[TODO?] - add this for potentially impossible edge case of index being out of range?
        // if let Some(slice) = self.input.get(startpos..index) {
        //     return slice;
        // } else {
        //     panic!("slice from read_identifier() ");
        // }
        &self.input[startpos..index]
    }

    //peeks at the char ahead, and if its a number, adds one to an iterator and skips to the next char - returns a slice of the original input as the output
    //[TODO] - intergrate index as a return from self.read_char(), so an extra index isn't needed
    fn read_number(&mut self, tok: (usize, char)) -> &'source str {
        let startpos = tok.0;
        let mut index = startpos + 1;
        while self.peek_is_number() {
            index += 1;
            self.read_char();
        }
        &self.input[startpos..index]
    }

    //the lexer should ignore all whitespace, as it shouldn't matter (except in checking for identifers, where it doens't use this function)
    // fn skip_whitespace(&mut self) {
    //     while let Some(&peek) = self.peek_char() {
    //         //no whitespace is more common, so that should be checked first
    //         if !peek.1.is_whitespace() {
    //             break;
    //         }
    //         self.read_char();
    //     }
    // }

    fn skip_whitespace(&mut self) {
        while let Some(&peek) = self.peek_char() {
            //no whitespace is more common, so that should be checked first
            if !peek.1.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    //returns the next token from the lexer - e.g. "=" => Token::Assign, "five" => Token::Identifier("five")
    pub fn next_token(&mut self) -> Token<'source> {
        self.skip_whitespace();

        let tok: Option<(usize, char)> = self.read_char();
        // let Some(tok2) = self.read_char();

        //[DEPRECATED] - really fucking aids, but it makes the match code below look nice and not have to use Some((_, ''))
        /*
        let tok = if let Some(tok) = self.read_char() {
            (Some(tok.0), Some(tok.1))
        } else {
            (None, None)
        };
        */

        //[TODO] - make this clean (not sure how else to word that)
        //matches the next symbol (as a Some(char)) to its token - None is the EOF
        match tok {
            Some((_, '=')) => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            Some((_, '+')) => Token::Plus,
            Some((_, '-')) => Token::Minus,
            Some((_, '!')) => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            Some((_, '*')) => Token::Asterisk,
            Some((_, '/')) => Token::Slash,
            Some((_, '<')) => Token::LessThan,
            Some((_, '>')) => Token::GreaterThan,
            Some((_, ',')) => Token::Comma,
            Some((_, ';')) => Token::Semicolon,
            Some((_, '(')) => Token::LeftParenthesis,
            Some((_, ')')) => Token::RightParenthesis,
            Some((_, '{')) => Token::LeftBrace,
            Some((_, '}')) => Token::RightBrace,
            // Some((_, '') => Token::,

            //catches all other options - must be an integer or an identifier - else, its an illegal token.
            //[TODO?] - make read_identifier and read_number take the first part of the Some tuple (the index) as an input instead of having to lend the whole tok
            Some((_, ch)) => {
                if is_letter(ch) {
                    let literal: &str = self.read_identifier(tok.unwrap());
                    crate::token::lookup_ident(literal)
                } else if is_number(ch) {
                    Token::Integer(self.read_number(tok.unwrap()))
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
        Token::Identifier("five"),
        Token::Assign,
        Token::Integer("5"),
        Token::Semicolon,
        Token::Let,
        Token::Identifier("ten"),
        Token::Assign,
        Token::Integer("10"),
        Token::Semicolon,
        Token::Let,
        Token::Identifier("add"),
        Token::Assign,
        Token::Function,
        Token::LeftParenthesis,
        Token::Identifier("x"),
        Token::Comma,
        Token::Identifier("y"),
        Token::RightParenthesis,
        Token::LeftBrace,
        Token::Identifier("x"),
        Token::Plus,
        Token::Identifier("y"),
        Token::RightBrace,
        Token::Semicolon,
        Token::Let,
        Token::Identifier("result"),
        Token::Assign,
        Token::Identifier("add"),
        Token::LeftParenthesis,
        Token::Identifier("five"),
        Token::Comma,
        Token::Identifier("ten"),
        Token::RightParenthesis,
        Token::Semicolon,
        Token::Bang,
        Token::Minus,
        Token::Slash,
        Token::Asterisk,
        Token::Integer("5"),
        Token::Semicolon,
        Token::Integer("5"),
        Token::LessThan,
        Token::Integer("10"),
        Token::GreaterThan,
        Token::Integer("5"),
        Token::Semicolon,
        Token::If,
        Token::LeftParenthesis,
        Token::Integer("5"),
        Token::LessThan,
        Token::Integer("10"),
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
        Token::Integer("10"),
        Token::Equal,
        Token::Integer("10"),
        Token::Semicolon,
        Token::Integer("10"),
        Token::NotEqual,
        Token::Integer("9"),
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
    while let Some(token) = Some(lex.next_token()) {
        if token != Token::EndOfFile {
            println!("{:?}", token);
        } else {
            break;
        }
    }
}
