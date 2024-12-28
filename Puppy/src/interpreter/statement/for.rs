use crate::{ast::For, interpreter::{value::NeoObject, Interpreter}};

impl Interpreter {
    pub fn interpret_statement_for(&mut self, statement_for: &For) -> NeoObject {
        // 1.新建栈帧
        self.execute_environment.push_frame();
        // 2.执行初始化语句
        self.interpret_statement_let(&statement_for.initialize);
        loop {
            // 3.执行条件语句
            let condition = self.interpret_expression(&statement_for.condition);
            let condition_neo_data_model = condition.neo_data_model();
            if condition_neo_data_model.is_boolean() {
                if !condition_neo_data_model.as_boolean() {
                    break;
                }
            } else {
                panic!("unexpected")
            }
            // 4.执行循环体
            let body = self.interpret_statement_block(&statement_for.body);
            let body_neo_data_model = body.neo_data_model();
            // case: body执行结果是break
            if body_neo_data_model.is_break() {
                // 直接break
                break;
            }
            // case: body执行结果是continue
            if body_neo_data_model.is_continue() {
                // 提前执行步进逻辑
                self.interpret_expression(&statement_for.step);
                // 继续循环
                continue;
            }
            // case: body执行结果是return
            if body_neo_data_model.is_return() {
                // 销毁栈帧，直接返回body的执行结果
                self.execute_environment.pop_frame();
                return body.clone();
            }
            // 5.执行步进逻辑
            self.interpret_expression(&statement_for.step);
        }
        // 6.执行结束，进行现场清理。如果body执行中没有发生return，那么就销毁栈帧，返回nil
        self.execute_environment.pop_frame();
        NeoObject::create_nil()
    }
}