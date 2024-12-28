//! 文法
//!
//! Program = StatementItem Program | StatementItem
//!
//! StatementItem = Class | If | While | For | Loop | Block | Let | Expression | Break | Continue | Return
//!
//! Class = ‘class’ '{' class_body '}'
//!
//! If
//!
//! While
//!
//! For
//!
//! Loop
//!
//! Block
//!
//! Let
//!
//! `Expression` = Expression_10 NewLine
//! * `Expression_10` = Expression_10 '=' Expression_9 | Expression_9
//! * `Expression_9` = Expression_9 '&&' Expression_8 | Expression_8
//! * `Expression_8` = Expression_8 '||' Expression_7 | Expression_7
//! * `Expression_7` = Expression_7 ('!=' | '==') Expression_6 | Expression_7
//! * `Expression_6` = Expression_6 ('>' | '<' | '>=' | '<=') Expression_5 | Expression_5
//! * `Expression_5` = Expression_5 ('+' | '-') Expression_5 | Expression_4
//! * `Expression_4` = Expression_4 ('*' | '/' | '%') Expression_3 | Expression_3
//! * `Expression_3` = ('-' | '!') Expression_3 | Expression_2
//! * `Expression_2` = Expression_2 '(' Parameters ')' | Expression_1
//! * `Parameters` = Parameters ',' Expression | Expression
//! * `Expression_1` = Expression_1 '.' Expression_0 | Expression_0
//! * `Expression_0` = '(' Expression ')' | Identifier | Literal
//!
//! `CreateExpression` = CreateExpression_10 NewLine
//! * `CreateExpression_10` = （下面和Expression一样，我就直接省略了）
//! * `CreateExpression_9` =
//! * `CreateExpression_8` =
//! * `CreateExpression_7` =
//! * `CreateExpression_6` =
//! * `CreateExpression_5` =
//! * `CreateExpression_4` =
//! * `CreateExpression_3` =
//! * `CreateExpression_2` =
//! * `CreateExpression_1` =
//! * `CreateExpression_0` = '(' Expression ')' | Identifier | Literal | Identifier '{' Initializes '}'
//! * `Initializes` = Initializes InitializeItem NewLine | InitializeItem NewLine
//! * `InitializeItem` = Identifier ':' CreateExpression
//!
//! Break
//!
//! Continue
//!
//! Return = 'return' \[Expression\] NewLine
//!
//! 面临的问题：如何处理;问题
//! Expression Let 末尾必须有换行符或EOF
//! 给出这样一个规定：Expression末尾必须有换行符或EOF，如果直接遇到EOF或换行符，说明这里没有表达式
//! Expression永远是要吃掉一个末尾的换行符的

use std::cmp::PartialEq;
use crate::ast::{Block, Break, Class, ClassData, ClassItem, ClassMethod, Continue, Expression, For, If, IfBranchItem, Let, Loop, Program, Return, Statement, TemplateDescribe, Type, While};
use crate::token::{Keyword, Operator, Punctuation, Token};
use crate::token_reader::TokenReader;

mod program;
mod statement_item;
mod expression;
mod create_expression;
mod reader;
mod jump;
mod r#return;
mod r#continue;
mod r#break;
mod r#let;
mod block;
mod r#loop;
mod r#for;
mod r#while;
mod r#if;
mod r#type;
mod class;

// 导入需要使用的方法
use program::*;
use statement_item::*;

pub fn parse(code: &str) -> Program{
    Parser::new(code).parse()
}

pub struct Parser{
    reader: TokenReader,
}

impl Parser{
    pub fn new(code: &str) -> Self{
        Self{
            reader: TokenReader::new(code),
        }
    }
}

impl Parser{
    pub fn parse(&mut self) -> Program {
        self.parse_program()
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn print_wher_am_i() {
        let args: Vec<_> = std::env::args().collect();
        println!("{}", args[0])
    }
    #[test]
    fn parse_code() {
        let code = std::fs::read_to_string("/Users/wuyumo/Desktop/Puppy/Puppy/puppy_test_code/main").unwrap();
        println!("{}", code);
        let program = parse(&code);
        println!("{:#?}", program)
    }
}



