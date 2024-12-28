use crate::ast::{Block, Statement};
use crate::parser::Parser;
use crate::token::{Punctuation, Token};

impl Parser {
    pub fn parse_block(&mut self) -> Block {
        // 读取{
        self.read();
        // 处理Block内部的所有Statement
        let mut items: Vec<Statement> = vec![];
        self.jump_newlines_and_eof();
        loop {
            if let Token::Punctuation(Punctuation::RightCurlyBracket) = self.current() {
                break
            }
            items.push(self.parse_statement_item());
            self.jump_newlines_and_eof()
        }
        // 读取}
        self.read();
        // 构造Block
        Block{
            items
        }
    }
}