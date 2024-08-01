use crate::token::Token;

#[derive(Debug)]
pub struct Continue;

impl Continue{
    pub fn create(_: Token) -> Self {
        Self
    }
}