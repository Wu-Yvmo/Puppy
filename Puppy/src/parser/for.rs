use crate::ast::For;
use crate::parser::Parser;

impl Parser{
    pub fn parse_for(&mut self) -> For {
        // 读取for
        self.read();
        // 解析let表达式
        let initialize = self.parse_let();
        // 读取;
        self.read();
        // 解析条件表达式
        let condition = self.parse_expression();
        // 读取;
        self.read();
        // 解析步进表达式
        let step = self.parse_expression();
        // 解析循环体
        let body = self.parse_block();
        // 构造For
        For{
            initialize,
            condition,
            step,
            body,
        }
    }
}