use crate::ast::Break;
use crate::parser::Parser;

impl Parser{
    pub fn parse_break(&mut self) -> Break {
        self.read();
        Break{}
    }
}