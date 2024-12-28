use crate::token;

#[derive(Debug, Clone)]
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