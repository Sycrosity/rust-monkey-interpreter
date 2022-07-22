#![allow(dead_code)]
#![allow(unused_imports)]

use crate::lexer::Lexer;
use crate::token::Token;

#[derive(Debug, PartialEq)]
pub struct Program<'source> {
    pub statements: Vec<Statement<'source>>,
}

impl<'source> Program<'source> {
    pub fn new() -> Self {
        Self {
            //unknown number of statments so should just be a vector
            statements: Vec::new(),
        }
    }
}

impl<'source> Default for Program<'source> {
    fn default() -> Self {
        Self::new()
    }
}

//[TODO] - add more types in future
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Statement<'source> {
    Let(&'source str, Expression<'source>),
}

//[TODO] - add more types in future
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Expression<'source> {
    Identifier(&'source str),
    Integer(i32),
}
