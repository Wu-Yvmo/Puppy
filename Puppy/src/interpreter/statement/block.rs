use crate::{ast::Block, interpreter::{value::NeoObject, Interpreter}};

impl Interpreter {
    pub fn interpret_statement_block(&mut self, block: &Block) -> NeoObject {
        self.execute_environment.push_frame();
        for statement in &block.items {
            let result = self.interpret_statement(statement);
            let result_neo_data_model = result.neo_data_model();
            if result_neo_data_model.is_break() || result_neo_data_model.is_continue() || result_neo_data_model.is_return() {
                self.execute_environment.pop_frame();
                return result.clone()
            }
        }
        self.execute_environment.pop_frame();
        NeoObject::create_nil()
    }
}