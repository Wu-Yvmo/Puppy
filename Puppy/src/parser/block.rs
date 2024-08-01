use crate::ast::{Block, StatementItem};
use crate::parser::Parser;
use crate::token::{Punctuation, Token};

impl Parser {
    pub fn parse_block(&mut self) -> Block {
        // 读取{
        self.read();
        // 处理Block内部的所有Statement
        let mut items: Vec<StatementItem> = vec![];
        self.jump_newlines_and_eof();
        while let Token::Punctuation(Punctuation::RightCurlyBracket) = self.current() {
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