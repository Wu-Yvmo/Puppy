use crate::parser::Parser;
use crate::token::Token;

// 转发对reader的方法调用
impl Parser{
    pub fn current(&mut self) -> Token{
        self.reader.current()
    }
    pub fn peek(&mut self) -> Token{
        self.reader.peek()
    }
    pub fn read(&mut self) -> Token{
        self.reader.read()
    }
    pub fn end(&mut self) -> bool {
        self.reader.end()
    }
}

