use crate::ast::Expression;

#[derive(Debug, Clone)]
pub struct Call{
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}

impl Call{
    pub fn create(function: Expression, arguments: Vec<Expression>) -> Self {
        Self{
            function: Box::new(function),
            arguments,
        }
    }
}