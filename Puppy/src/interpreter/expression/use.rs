use crate::ast::Use;
use crate::interpreter::Interpreter;
use crate::interpreter::value::NeoObject;

impl Interpreter {
    pub fn interpret_expression_use(&mut self, r#use: &Use) -> NeoObject {
        let name = r#use.name.text();
        self.execute_environment.query(name)
    }
}