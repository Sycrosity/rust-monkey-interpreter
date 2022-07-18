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
            statements: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Statement<'source> {
    Let(&'source str, Expression<'source>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Expression<'source> {
    Identifier(&'source str),
    Integer(i32),
}
