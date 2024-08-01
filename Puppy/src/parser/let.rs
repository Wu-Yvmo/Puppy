use crate::ast::Let;
use crate::parser::Parser;

impl Parser{
    pub fn parse_let(&mut self) -> Let {
        // 读取let关键字
        self.read();
        // 读取变量名
        let name = self.read();
        // 读取=
        self.read();
        // 读取初始化表达式
        let initialize_value = self.parse_expression();
        // 构造Let
        Let{
            name,
            initialize_value,
        }
    }
}