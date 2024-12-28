use crate::ast::{Binary, Call, Create, Expression, Literal, Pack, Unary, Use};
use crate::interpreter::Interpreter;
use crate::interpreter::value::NeoObject;

mod binary;
mod call;
mod r#use;
mod unary;
mod pack;
mod literal;
mod create;

impl Interpreter {
    pub fn interpret_expression(&mut self, expression: &Expression) -> NeoObject {
        match expression {
            Expression::Call(expression_call) => {
                self.interpret_expression_call(expression_call)
            }
            Expression::Use(expression_use) => {
                self.interpret_expression_use(expression_use)
            }
            Expression::Binary(expression_binary) => {
                self.interpret_expression_binary(expression_binary.left.as_ref(), &expression_binary.operator, expression_binary.right.as_ref())
            }
            Expression::Unary(expression_unary) => {
                self.interpret_expression_unary(expression_unary)
            }
            Expression::Pack(expression_pack) => {
                self.interpret_expression_pack(expression_pack)
            }
            Expression::Literal(expression_literal) => {
                self.interpret_expression_literal(expression_literal)
            }
            Expression::Create(expression_create) => {
                self.interpret_expression_create(expression_create)
            }
        }
    }
}