use crate::ast::{Create, Expression};
use crate::interpreter::Interpreter;
use crate::interpreter::value::NeoObject;

impl Interpreter {
    pub fn interpret_expression_create(&mut self, create: &Create) -> NeoObject {
        // 在完善class基础设施之前也没有实现该功能的条件
        todo!()
    }
}