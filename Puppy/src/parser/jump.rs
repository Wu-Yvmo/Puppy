use crate::parser::Parser;
use crate::token::Token;

impl Parser{
    pub fn jump_newlines_and_eof(&mut self) {
        while let Token::NewLine|Token::EndOfFile = self.current() {
            self.read();
        }
    }
}