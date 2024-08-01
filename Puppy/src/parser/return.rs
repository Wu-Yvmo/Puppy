use crate::ast::Return;
use crate::parser::Parser;
use crate::token::Token;

impl Parser{
    pub fn parse_return(&mut self) -> Return {
        self.read();
        if let Token::NewLine|Token::EndOfFile = self.current() {
            self.read();
            return Return{
                optional_expression: None,
            }
        }
        let expression = self.parse_expression();
        // 构造Return并返回
        Return{
            optional_expression: Some(expression)
        }
    }
}