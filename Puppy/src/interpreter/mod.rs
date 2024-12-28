use std::collections::HashMap;
use crate::ast::{Binary, Call, Create, Expression, Literal, Pack, Program, Statement, Unary, Use};
use crate::interpreter::execute_environment::ExecuteEnvironment;
use crate::interpreter::value::{NeoDataModel, NeoObject};
use crate::token;
use crate::token::{Operator, Token};

mod value;
mod execute_environment;
mod statement;
mod expression;

pub fn interpret(program: Program) {
}

pub struct Interpreter{
    execute_environment: ExecuteEnvironment,
}

impl Interpreter{
    pub fn create(program: &Program) -> Self {
        Self{
            execute_environment: ExecuteEnvironment::create(program)
        }
    }

    pub fn interpret(&mut self, program: &Program) {
        for program_item in &program.items {
            self.interpret_statement(program_item);
        }
    }

    // pub fn interpret_statement_item(&mut self, statement_item: &Statement) {
    //     match statement_item{
    //         Statement::Class(_) => todo!("out of consideration"),
    //         Statement::If(statement_if) => todo!("if"),
    //         Statement::While(statement_while) => todo!("while"),
    //         Statement::For(statement_for) => todo!("for"),
    //         Statement::Loop(statement_loop) => todo!("loop"),
    //         Statement::Block(statement_block) => todo!("block"),
    //         Statement::Let(statement_let) => todo!("let"),
    //         Statement::Expression(statement_expression) => todo!("expression"),
    //         Statement::Break(statement_break) => todo!("break"),
    //         Statement::Continue(statement_continue) => todo!("continue"),
    //         Statement::Return(statement_return) => todo!("return"),
    //     }
    // }
}


// 有必要重申语法结构
// 一个文件就是一个class
// 目前我们只会parse出来一个Program，这个program中