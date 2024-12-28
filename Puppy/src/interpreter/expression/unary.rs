use core::panic;

use crate::ast::Unary;
use crate::interpreter::Interpreter;
use crate::interpreter::value::NeoObject;

impl Interpreter {
    pub fn interpret_expression_unary(&mut self, unary: &Unary) -> NeoObject {
        match &unary.operator {
            crate::token::Token::Operator(operator) => match operator {
                crate::token::Operator::Add => self.interpret_expression(unary.right.as_ref()),
                crate::token::Operator::Sub => {
                    let neo_object = self.interpret_expression(unary.right.as_ref());
                    let neo_data_model = neo_object.handle.borrow();
                    if neo_data_model.is_number() {
                        NeoObject::create_number(-neo_data_model.as_number())
                    } else {
                        panic!("unexpected")
                    }
                },
                crate::token::Operator::Not => {
                    let neo_object = self.interpret_expression(unary.right.as_ref());
                    let neo_data_model = neo_object.handle.borrow();
                    if neo_data_model.is_boolean() {
                        NeoObject::create_boolean(!neo_data_model.as_boolean())
                    } else {
                        panic!("unexpected")
                    }
                }
                _ =>panic!("unexpected operator")
            }
            _ => panic!("unexpected")
        }
    }

}