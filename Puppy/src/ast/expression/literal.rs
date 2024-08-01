use std::os::unix::raw::nlink_t;
use crate::token;

#[derive(Debug)]
pub struct Literal {
    pub literal: token::Literal,
}

impl Literal{
    pub fn create(literal: token::Literal) -> Self {
        Self{
            literal,
        }
    }
}