mod call;
mod r#use;
mod binary;
mod unary;
mod pack;
mod literal;
mod create;

// 导出所有需要使用的项
pub use call::*;
pub use r#use::*;
pub use binary::*;
pub use unary::*;
pub use pack::*;
pub use literal::*;
pub use create::*;

use crate::token;
use crate::token::Token;

#[derive(Debug)]
pub enum Expression{
    Call(Box<Call>),
    Use(Use),
    Binary(Binary),
    Unary(Unary),
    Pack(Pack),
    Literal(Literal),
    Create(Create),
}

impl Expression {
    pub fn create_call(function: Expression, arguments: Vec<Expression>) -> Self {
        Self::Call(Box::new(Call::create(function, arguments)))
    }
    pub fn create_use(name: Token) -> Self {
        Self::Use(Use::create(name))
    }
    pub fn create_binary(left: Expression, operator: Token, right: Expression) -> Self {
        Self::Binary(Binary::create(left, operator, right))
    }
    pub fn create_unary(operator: Token, right: Expression) -> Self {
        Self::Unary(Unary::create(operator, right))
    }
    pub fn create_pack(sub: Expression) -> Self {
        Self::Pack(Pack::create(sub))
    }
    pub fn create_literal(literal: token::Literal) -> Self {
        Self::Literal(Literal::create(literal))
    }
    pub fn create_create(name: Token, initializes: Vec<(Token, Expression)>) -> Self {
        Self::Create(Create::create(name, initializes))
    }
}