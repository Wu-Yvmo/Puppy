use crate::ast::{Block, Type};
use crate::token::Token;

#[derive(Debug)]
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

#[derive(Debug)]
pub enum ClassItem{
    Data(ClassData),
    Method(ClassMethod),
}

impl ClassItem{
    pub fn create_data(name: Token, data_type: Type) -> Self {
        Self::Data(ClassData::create(name, data_type))
    }
    pub fn create_method(name: Token, in_parameters: Vec<ParameterItem>, out_parameter: Type, body: Block) -> Self {
        Self::Method(ClassMethod::create(name, in_parameters, out_parameter, body))
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ClassMethod {
    pub name: Token,
    pub in_parameters: Vec<ParameterItem>,
    pub out_parameter: Type,
    pub body: Block,
}

impl ClassMethod {
    pub fn create(name: Token, in_parameters: Vec<ParameterItem>, out_parameter: Type, body: Block) -> Self {
        Self{
            name,
            in_parameters,
            out_parameter,
            body
        }
    }
}

#[derive(Debug)]
pub enum ParameterItem{
    TheSelf,
    Normal(Token, Type)
}

impl ParameterItem{
    pub fn create_self() -> Self {
        Self::r#TheSelf
    }
    pub fn create_normal(name: Token, data_type: Type) -> Self {
        Self::Normal(name, data_type)
    }
}
