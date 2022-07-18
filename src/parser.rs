#![allow(dead_code)]
#![allow(unused_imports)]

use crate::ast::{Expression, Program, Statement};
use crate::lexer::Lexer;
use crate::token::Token;

use std::io::Error;
use std::iter::{Iterator, Peekable};

// type Result<'source, T> = std::result::Result<T, ParserError<'source>>;


//[TODO] -  make error system more verbose and thorough
#[derive(Debug)]
pub enum ParserError<'source> {
    ExpectedLet(Token<'source>),
    ExpectedIdentifier(Token<'source>),
    ExpectedAssign(Token<'source>),
    ExpectedLParenthesis(Token<'source>),
    ExpectedRParenthesis(Token<'source>),
    ExpectedLeftBrace(Token<'source>),
    ExpectedRightBrace(Token<'source>),
    ExpectedSemiColon(Token<'source>),
    ExpectedComma(Token<'source>),
    ExpectedToken {
        expected: Token<'source>,
        got: Token<'source>,
    },
    Unknown(usize),
}

pub struct Parser<'source> {
    input: &'source str,
    iter: Peekable<Lexer<'source>>,
    errors: Vec<ParserError<'source>>,
}

impl<'source> Parser<'source> {
    pub fn new(input: &'source str) -> Self {
        Self {
            input,
            iter: Lexer::new(input).peekable(),
            errors: Vec::new(),
        }
    }

    pub fn parse_program(&mut self) -> Program<'source> {
        let mut program: Program<'source> = Program::new();

        // while let Some(&peek) = self.iter.peek() {
        //     //no whitespace is more common, so that should be checked first
        //     if peek != Token::EndOfFile {
        //         let statement: Statement = self.parse_statement();
        //         program.statements.push(statement);
        //     } else {
        //         break;
        //     }
        //     self.iter.next();
        // }

        while let Some(&peek) = self.peek_token() {
            if peek != Token::EndOfFile {
                match self.parse_statement() {
                    Ok(statement) => program.statements.push(statement),
                    Err(err) => { self.errors.push(err); },
                }
            } else {
                break;
            }
        }

        program
    }

    pub fn parse_statement(&mut self) -> Result<Statement<'source>, ParserError<'source>> {
        let tok: Option<Token<'source>> = self.read_token();
        match tok {
            Some(Token::Let) => self.parse_let_statement(),
            Some(tok) =>
            /*todo!()*/
            {
                println!("statement: {:?}",tok);
                Err(ParserError::Unknown(1))
            }
            None => todo!(),
        }
    }

    fn read_token(&mut self) -> Option<Token<'source>> {
        self.iter.next()
    }

    fn peek_token(&mut self) -> Option<&Token<'source>> {
        self.iter.peek()
    }

    // fn peek_token_eq(&mut self, eq: Token<'source>) -> bool {
    //     match self.peek_token() {
    //         Some(&tok) => eq == tok,
    //         None => false,
    //     }
    // }

    fn expect_peek<F>(
        &mut self,
        eq: Token<'source>,
        parser_error: F,
    ) -> Result<(), ParserError<'source>>
    where
        F: Fn(Token<'source>) -> ParserError<'source>,
    {
        if let Some(&peek) = self.peek_token().clone() {
            if peek == eq {
                self.read_token();
                Ok(())
            } else {
                self.read_token();
                Err(parser_error(peek))
            }
        } else {
            Err(ParserError::Unknown(2))
        }
    }

    // fn parse_let_statement(&mut self) -> Result<Statement<'source>> {
    //     match self.read_token() {
    //         Some(tok) => {
    //             if let Token::Identifier(ident) = tok {
    //                 self.expect_peek(Token::Assign, ParserError::ExpectedAssign)?;

    //                 //[TODO] - add expression
    //                 Ok(Statement::Let(ident, Expression::Identifier("hii")))
    //             } else {
    //                 return Err(ParserError::ExpectedIdentifier(tok));
    //             }
    //         }
    //         None => return Err(ParserError::Unknown),
    //     }
    // }

    fn parse_let_statement(&mut self) -> Result<Statement<'source>, ParserError<'source>> {
        let identifier: &'source str;

        if let Some(tok) = self.read_token().clone() {
            if let Token::Identifier(ident) = tok {
                identifier = ident;

                //[TODO] - add expression
            } else {
                self.read_token();
                return Err(ParserError::ExpectedIdentifier(tok));
            }
        } else {
            return Err(ParserError::Unknown(3));
        }

        self.expect_peek(Token::Assign, ParserError::ExpectedAssign)?;

        while Some(Token::Semicolon) != self.read_token() {}

        Ok(Statement::Let(identifier, Expression::Identifier("hii")))
    }

    fn check_parser_errors(&self) {
        if self.errors.is_empty() {
            return;
        }

        eprintln!("Parser has {} error(s):", self.errors.len());
        self.errors.iter().enumerate().for_each(|(i, error)| {
            eprintln!("\t{}. {:?}", i, error);
        });
    }
}

#[test]
fn visible_test_parser() {
    let mut parser: Parser = Parser::new("let b ");

    let program: Program = parser.parse_program();

    program
        .statements
        .into_iter()
        .for_each(|statement: Statement| {
            println!("{:?}", statement);
        })
}

#[test]
fn test_let_statements() {
    let input: &str = "let x = 5;
    let y = 10;
    let foobar = 838383;";

    // let lex = Lexer::new(input);

    let mut parser: Parser = Parser::new(input);
    let program: Program = parser.parse_program();
    parser.check_parser_errors();

    //[TODO] - create general parser error catcher
    //[TODO] - test more types of expression
    /*
    let tests = vec![
        Statement::Let("x".to_string(), Expression::Integer(5)),
        Statement::Let("y".to_string(), Expression::Boolean(true)),
        Statement::Let("z".to_string(), Expression::Identifier("y")),
    ];
    */

    // let tests: Vec<Statement> = vec![
    //     Statement::Let("x", Expression::Integer(5)),
    //     Statement::Let("y", Expression::Integer(10)),
    //     Statement::Let("foobar", Expression::Integer(838383)),
    // ];

    let tests: Vec<&str> = vec!["x", "y", "foobar"];

    // assert_eq!(program.statements, tests);
    tests
        .into_iter()
        .enumerate()
        .for_each(|statement: (usize, &str)| {
            if let Some(Statement::Let(x, _)) = Some(program.statements[statement.0]) {
                assert_eq!(x, statement.1);
            }

            // assert_eq!(program.statements[statement.0], statement.1);
        });
}

//[TODO] - add more errors to test
//[TODO]! - errors don't error properly
#[test]
fn test_errors() {
    let input: &str = "let x 5;";

    // let lex = Lexer::new(input);

    let mut parser: Parser = Parser::new(input);
    parser.parse_program();
    parser.check_parser_errors();
    println!("{:?}", parser.errors)
}
