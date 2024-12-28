use crate::interpreter::{value::NeoObject, Interpreter};

impl Interpreter {
    pub fn interpret_statement_break(&mut self) -> NeoObject {
        NeoObject::create_break()
    }
}