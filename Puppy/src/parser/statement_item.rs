use crate::ast::StatementItem;
use crate::parser::Parser;
use crate::token::{Keyword, Punctuation, Token};

impl Parser{
    // 考虑解决return的表达式问题 b = a + b * c - d
    // b = a + b
    // 现在的方案是，表达式只能出现在一行里
    pub fn parse_statement_item(&mut self) -> StatementItem {
        match self.current() {
            Token::Keyword(keyword) => match keyword {
                Keyword::Class => StatementItem::from_class(self.parse_class()),
                Keyword::If => StatementItem::from_if(self.parse_if()),
                Keyword::While => StatementItem::from_while(self.parse_while()),
                Keyword::For => StatementItem::from_for(self.parse_for()),
                Keyword::Loop => StatementItem::from_loop(self.parse_loop()),
                Keyword::Let => StatementItem::from_let(self.parse_let()),
                Keyword::Break => StatementItem::from_break(self.parse_break()),
                Keyword::Continue => StatementItem::from_continue(self.parse_continue()),
                Keyword::Return => StatementItem::from_return(self.parse_return()),
                _ => panic!("unexpected"),
            }
            Token::Punctuation(punctuation) => match punctuation {
                Punctuation::LeftCurlyBracket => StatementItem::from_block(self.parse_block()),
                _ => panic!("unexpected"),
            }
            _ => StatementItem::from_expression(self.parse_expression()),
        }
    }
}