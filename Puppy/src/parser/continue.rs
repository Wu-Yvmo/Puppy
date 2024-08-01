use crate::ast::Continue;
use crate::parser::Parser;

impl Parser{
    pub fn parse_continue(&mut self) -> Continue {
        self.read();
        Continue{}
    }
}