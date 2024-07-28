use crate::ast::Expression;
use crate::token::Token;

pub struct Create {
    pub name: Token,
    pub initializes: Vec<(Token, Expression)>,
}

impl Create{
    pub fn create(name: Token, initializes: Vec<(Token, Expression)>) -> Self {
        Self{
            name,
            initializes,
        }
    }
}