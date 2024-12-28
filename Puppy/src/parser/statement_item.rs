use crate::ast::Statement;
use crate::parser::Parser;
use crate::token::{Keyword, Punctuation, Token};

impl Parser{
    // 考虑解决return的表达式问题 b = a + b * c - d
    // b = a + b
    // 现在的方案是，表达式只能出现在一行里
    pub fn parse_statement_item(&mut self) -> Statement {
        match self.current() {
            Token::Keyword(keyword) => match keyword {
                Keyword::Class => Statement::from_class(self.parse_class()),
                Keyword::If => Statement::from_if(self.parse_if()),
                Keyword::While => Statement::from_while(self.parse_while()),
                Keyword::For => Statement::from_for(self.parse_for()),
                Keyword::Loop => Statement::from_loop(self.parse_loop()),
                Keyword::Let => Statement::from_let(self.parse_let()),
                Keyword::Break => Statement::from_break(self.parse_break()),
                Keyword::Continue => Statement::from_continue(self.parse_continue()),
                Keyword::Return => Statement::from_return(self.parse_return()),
                _ => panic!("unexpected"),
            }
            Token::Punctuation(punctuation) => match punctuation {
                Punctuation::LeftCurlyBracket => Statement::from_block(self.parse_block()),
                _ => panic!("unexpected"),
            }
            _ => Statement::from_expression(self.parse_expression()),
        }
    }
}