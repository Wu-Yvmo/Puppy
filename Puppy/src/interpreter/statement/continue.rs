use crate::interpreter::{value::NeoObject, Interpreter};

impl Interpreter {
    pub fn interpret_statement_continue(&mut self) -> NeoObject {
        NeoObject::create_continue()
    }
}