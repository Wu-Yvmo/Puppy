use crate::ast::Expression;
use crate::token::Token;

#[derive(Debug)]
pub struct Let{
    pub name: Token,
    pub initialize_value: Expression,
}

impl Let{
    pub fn create(name: Token, initialize_value: Expression) -> Self {
        Self{
            name,
            initialize_value,
        }
    }
}