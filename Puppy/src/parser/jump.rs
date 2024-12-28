use crate::parser::Parser;
use crate::token::Token;

impl Parser{
    pub fn jump_newlines_and_eof(&mut self) {
        while !self.end() && self.could_jump() {
            self.read();
        }
    }
    pub(self) fn could_jump(&mut self) -> bool {
        if let Token::NewLine|Token::EndOfFile = self.current() {
            return true
        }
        return false;
    }
}