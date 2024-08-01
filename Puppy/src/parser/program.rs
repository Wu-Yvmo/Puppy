use crate::ast::{Program, StatementItem};
use crate::parser::Parser;

impl Parser{
    pub fn parse_program(&mut self) -> Program {
        let mut items: Vec<StatementItem> = vec![];
        while !self.end() {
            items.push(self.parse_statement_item());
        }
        Program::create(items)
    }
}