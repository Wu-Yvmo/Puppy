use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Use{
    pub name: Token,
}

impl Use {
    pub fn create(name: Token) -> Self {
        Use{
            name
        }
    }
}