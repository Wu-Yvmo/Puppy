use crate::ast::Pack;
use crate::interpreter::Interpreter;
use crate::interpreter::value::NeoObject;

impl Interpreter {
    pub fn interpret_expression_pack(&mut self, pack: &Pack) -> NeoObject {
        self.interpret_expression(pack.sub.as_ref())
    }
}