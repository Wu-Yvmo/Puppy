use crate::ast::{Program, Statement};
use crate::parser::Parser;

impl Parser{
    pub fn parse_program(&mut self) -> Program {
        let mut items: Vec<Statement> = vec![];
        self.jump_newlines_and_eof();
        while !self.end() {
            items.push(self.parse_statement_item());
            self.jump_newlines_and_eof();
        }
        Program::create(items)
    }
}