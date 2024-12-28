use crate::tokenizer::lex;
use crate::token::Token;

pub struct TokenReader{
    tokens: Vec<Token>,
    index: usize,
}

impl TokenReader{
    pub fn new(code: &str) -> Self {
        Self{
            tokens: lex(code),
            index: 0,
        }
    }
    pub fn current(&mut self) -> Token {
        self.tokens[self.index].clone()
    }
    pub fn peek(&mut self) -> Token {
        self.tokens[self.index+1].clone()
    }
    pub fn read(&mut self) -> Token {
        let t = self.tokens[self.index].clone();
        self.index += 1;
        t
    }
    pub fn end(&self) -> bool {
        self.index >= self.tokens.len()
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    pub fn test() {
        let mut reader = TokenReader::new("abc while for");
        println!("{}", reader.read().dump());
        println!("{}", reader.read().dump());
        println!("{}", reader.read().dump());
        assert!(reader.end())
    }
}