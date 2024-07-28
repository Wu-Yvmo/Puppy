use crate::ast::Expression;
use crate::token::Token;

pub struct Unary{
    pub operator: Token,
    pub right: Box<Expression>,
}

impl Unary{
    pub fn create(operator: Token, right: Expression) -> Self {
        Self{
            operator,
            right: Box::new(right),
        }
    }
}