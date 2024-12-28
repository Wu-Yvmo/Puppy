use crate::ast::Literal;
use crate::interpreter::Interpreter;
use crate::interpreter::value::NeoObject;

impl Interpreter {
    pub fn interpret_expression_literal(&mut self, literal: &Literal) -> NeoObject {
        match &literal.literal {
            crate::token::Literal::String { 
                literal 
            } => NeoObject::create_string(literal.clone()),
            crate::token::Literal::Number {
                literal 
            } => NeoObject::create_number(*literal),
            crate::token::Literal::Boolean { 
                literal 
            } => NeoObject::create_boolean(*literal),
        }
    }
}