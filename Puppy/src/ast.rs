//! 提供抽象语法树
use serde::{Deserialize, Serialize};
use serde_json::Result;
use crate::ast::ClassItem::Data;
use crate::token;
use crate::token::Token;

pub struct Program{
    items: Vec<StatementItem>,
}

impl Program{
    pub fn from_items(items: Vec<StatementItem>) -> Self {
        Self{
            items
        }
    }
}

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

pub struct If{
    condition: Expression,
    true_branch: Block,
    optional_false_branch: Option<IfBranchItem>,
}

impl If {
    pub fn create(condition: Expression, true_branch: Block, optional_false_branch: Option<IfBranchItem>) -> Self {
        If{
            condition,
            true_branch,
            optional_false_branch,
        }
    }
}

pub enum IfBranchItem{
    If(Box<If>),
    Block(Block),
}

impl IfBranchItem {
    pub fn create_if(if_branch: If) -> Self {
        Self::If(Box::new(if_branch))
    }
    pub fn create_block(block: Block) -> Self {
        Self::Block(block)
    }
}

pub struct While{
    pub condition: Expression,
    pub body: Block,
}

pub struct For{
    pub initialize: Let,
    pub condition: Expression,
    pub step: Expression,
    pub body: Block,
}

pub struct Loop{
    pub body: Block,
}

pub struct Break;

pub struct Continue;

pub struct Return{
    pub optional_expression: Option<Expression>,
}

// 类定义
// 类中可能存在的东西有
// 方法
// 字段
pub struct Class{
    pub items: Vec<ClassItem>,
}

impl Class {
    pub fn create(items: Vec<ClassItem>) -> Self {
        Self{
            items
        }
    }
}

pub enum ClassItem{
    Data(ClassData),
    Method(ClassMethod),
}

impl ClassItem {
    pub fn create_data(name: Token, data_type: Type) -> Self {
        Self::Data(ClassData{
            name,
            data_type,
        })
    }
    pub fn create_method() -> Self {
        todo!()
    }
}

// 一个尝试：
// Array: Array[Number]
// Map: Map[Number, Number]
// Set: Set[Number]
// Tuple: Tuple[String, String]

// Array: []Number
// Map: {Number: Number}
// Set: ()Number
// Tuple:(Number, Number, []Number)

// 我比较喜欢下面的方案
// Vector[Number]
// Map[Number, Number]
// Set: Set[Number]
// Tuple: Tuple[Number]
// 总结一下文法：
// Type = Identifier [ TemplateDescribe ]
// TemplateDescribe = '[' TemplateDescribeContent ']'
// TemplateDescribeContent = TemplateDescribeContent ',' Type | Type

// 我简单的说一下两种范型的优缺点
// 优点是，扩展性好

pub enum Type {
    TheSelf,
    Normal(Normal),
}

pub struct Normal{
    pub name: Token,
    pub optional_template_describe:Option<TemplateDescribe>,
}

impl Type {
    pub fn create_self() -> Self {
        Self::TheSelf
    }
    pub fn create_normal(name: Token, optional_template_describe: Option<TemplateDescribe>) -> Self {
        Self::Normal(Normal{
            name,
            optional_template_describe
        })
    }
}

pub struct TemplateDescribe {
    pub items: Vec<Type>,
}

impl TemplateDescribe {
    pub fn create_template_describe(items: Vec<Type>) -> Self {
        Self {
            items
        }
    }
}

pub struct ClassData {
    pub name: Token,
    pub data_type: Type,
}

impl ClassData {
    pub fn create(name: Token, data_type: Type) -> Self {
        Self{
            name,
            data_type,
        }
    }
}

pub struct ClassMethod {
    name: Token,
    in_parameters: Vec<(Token, Type)>,
    out_parameter: Type,
    body: Block,
}

impl ClassMethod {
    pub fn create(name: Token, in_parameters: Vec<(Token, Type)>, out_parameter: Type, body: Block) -> Self {
        todo!()
    }
}

pub struct Block{
    pub items: Vec<StatementItem>,
}

pub struct Let{
    pub name: Token,
    pub initialize_value: Expression,
}

pub enum Expression{
    Call(Box<Call>),
    Use(Use),
    Binary(Binary),
    Unary(Unary),
    Pack(Pack),
    Literal(Literal),
    Create(Create),
}

impl Expression {
    pub fn create_call(function: Expression, arguments: Vec<Expression>) -> Self {
        Self::Call(Box::new(Call{
            function: Box::new(function),
            arguments,
        }))
    }
    pub fn create_use(name: Token) -> Self {
        Self::Use(Use{
            name,
        })
    }
    pub fn create_binary(left: Expression, operator: Token, right: Expression) -> Self {
        Self::Binary(Binary{
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
    pub fn create_unary(operator: Token, right: Expression) -> Self {
        Self::Unary(Unary{
            operator,
            right: Box::new(right),
        })
    }
    pub fn create_pack(sub: Expression) -> Self {
        Self::Pack(Pack{
            sub: Box::new(sub),
        })
    }
    pub fn create_literal(literal: token::Literal) -> Self {
        Self::Literal(Literal{
            literal
        })
    }
    pub fn create_create(name: Token, initializes: Vec<(Token, Expression)>) -> Self {
        Self::Create(Create{
            name,
            initializes,
        })
    }
}

pub struct Call{
    function: Box<Expression>,
    arguments: Vec<Expression>,
}

pub struct Use{
    name: Token,
}

pub struct Binary{
    left: Box<Expression>,
    operator: Token,
    right: Box<Expression>,
}

pub struct Unary{
    operator: Token,
    right: Box<Expression>,
}

pub struct Pack{
    sub: Box<Expression>
}

pub struct Literal {
    literal: token::Literal,
}

pub struct Create {
    name: Token,
    initializes: Vec<(Token, Expression)>,
}

#[derive(Serialize, Deserialize)]
struct T{
    name: String,
    age: i32,
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn json() {
        let t = T{
            name: "Hell".to_string(),
            age: 34,
        };
        let json = serde_json::to_string_pretty(&t).unwrap();
        println!("{}", json)
    }
}