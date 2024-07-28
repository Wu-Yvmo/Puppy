use std::io::Repeat;
use crate::ast::{Block, Break, Class, ClassItem, Continue, Expression, For, If, IfBranchItem, Let, Loop, Return, While};
use crate::token::Token;

// ProgramItem中可能包括：
// class定义
// 表达式
// let语句
pub enum StatementItem{
    Class(Class),
    If(If),
    While(While),
    For(For),
    Loop(Loop),
    Block(Block),
    Let(Let),
    Expression(Expression),
    Break(Break),
    Continue(Continue),
    Return(Return),
}

impl StatementItem{
    pub fn from_class(r#class: Class) -> Self {
        StatementItem::Class(r#class)
    }
    pub fn from_if(r#if: If) -> Self {
        StatementItem::If(r#if)
    }
    pub fn from_while(r#while: While) -> Self {
        StatementItem::While(r#while)
    }
    pub fn from_for(r#for: For) -> Self {
        StatementItem::For(r#for)
    }
    pub fn from_loop(r#loop: Loop) -> Self {
        StatementItem::Loop(r#loop)
    }
    // todo!(从这里开始)
    pub fn from_block(block: Block) -> Self {
        StatementItem::Block(block)
    }
    pub fn from_let(r#let: Let) -> Self {
        StatementItem::Let(r#let)
    }
    pub fn from_expression(expression: Expression) -> Self {
        StatementItem::Expression(expression)
    }
    pub fn from_break(r#break: Break) -> Self {
        StatementItem::Break(r#break)
    }
    pub fn from_continue(r#continue: Continue) -> Self {
        StatementItem::Continue(r#continue)
    }
    pub fn from_return(r#return: Return) -> Self {
        StatementItem::Return(r#return)
    }
}