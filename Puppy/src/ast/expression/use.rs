use crate::token::Token;

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