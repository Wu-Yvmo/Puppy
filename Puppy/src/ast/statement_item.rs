use crate::ast::{Block, Break, Class, Continue, Expression, For, If, Let, Loop, Return, While};

// ProgramItem中可能包括：
// class定义
// 表达式
// let语句
#[derive(Debug, Clone)]
pub enum Statement{
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

impl Statement{
    pub fn from_class(r#class: Class) -> Self {
        Statement::Class(r#class)
    }

    pub fn from_if(r#if: If) -> Self {
        Statement::If(r#if)
    }

    pub fn from_while(r#while: While) -> Self {
        Statement::While(r#while)
    }

    pub fn from_for(r#for: For) -> Self {
        Statement::For(r#for)
    }

    pub fn from_loop(r#loop: Loop) -> Self {
        Statement::Loop(r#loop)
    }

    pub fn from_block(block: Block) -> Self {
        Statement::Block(block)
    }

    pub fn from_let(r#let: Let) -> Self {
        Statement::Let(r#let)
    }

    pub fn from_expression(expression: Expression) -> Self {
        Statement::Expression(expression)
    }

    pub fn from_break(r#break: Break) -> Self {
        Statement::Break(r#break)
    }

    pub fn from_continue(r#continue: Continue) -> Self {
        Statement::Continue(r#continue)
    }

    pub fn from_return(r#return: Return) -> Self {
        Statement::Return(r#return)
    }
}