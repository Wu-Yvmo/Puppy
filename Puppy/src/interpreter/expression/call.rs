use crate::ast::{Call, Expression};
use crate::interpreter::Interpreter;
use crate::interpreter::value::NeoObject;

impl Interpreter {
    pub fn interpret_expression_call(&mut self, call: &Call) -> NeoObject {
        // 暂时没有提供call的支持
        // 查询方法
        let method = self.interpret_expression(call.function.as_ref());
        // 对传入的参数进行求值操作
        let arguments: Vec<_> = call.arguments.iter().map(|argument| self.interpret_expression(argument)).collect();
        // 做到这里
        // todo 这里是真的不好做。先把.语法完善了吧
        // 我感觉现在的基础设施其实没有执行方法的条件，甚至没有执行函数的条件
        todo!()
    }
}