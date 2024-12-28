use crate::ast::Statement;

use super::{value::NeoObject, Interpreter};

mod class;
mod r#if;
mod r#while;
mod r#for;
mod r#loop;
mod r#block;
mod r#let;
mod r#break;
mod r#continue;
mod r#return;

impl Interpreter {
    pub fn interpret_statement(&mut self, statement: &Statement) -> NeoObject {
        match statement {
            Statement::Class(class) => todo!("class interpret not done"),
            Statement::If(statement_if) => todo!(),
            Statement::While(statement_while) => todo!(),
            Statement::For(statement_for) => todo!(),
            Statement::Loop(statement_loop) => todo!(),
            Statement::Block(statement_block) => todo!(),
            Statement::Let(statement_let) => todo!(),
            Statement::Expression(statement_expression) => self.interpret_expression(statement_expression),
            Statement::Break(statement_break) => todo!(),
            Statement::Continue(statement_continue) => todo!(),
            Statement::Return(statement_return) => todo!(),
        }
    }
}