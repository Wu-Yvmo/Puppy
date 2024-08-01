use crate::ast::While;
use crate::parser::Parser;

impl Parser{
    pub fn parse_while(&mut self) -> While {
        // 读取关键字while
        self.read();
        // 读取条件
        let condition = self.parse_expression();
        // 解析body
        let body = self.parse_block();
        // 构造while
        While{
            condition,
            body
        }
    }
}