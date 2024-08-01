use crate::ast::Expression;
use crate::token::Token;

#[derive(Debug)]
pub struct Binary{
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

impl Binary {
    pub fn create(left: Expression, operator: Token, right: Expression) -> Self {
        Self{
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}