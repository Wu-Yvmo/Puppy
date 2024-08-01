use crate::ast::Loop;
use crate::parser::Parser;

impl Parser{
    pub fn parse_loop(&mut self) -> Loop {
        // 读取关键字loop
        self.read();
        // 解析Block
        let body = self.parse_block();
        // 构造Loop
        Loop {
            body
        }
    }
}