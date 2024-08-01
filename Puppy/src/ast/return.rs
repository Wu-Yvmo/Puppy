use crate::ast::Expression;

#[derive(Debug)]
pub struct Return{
    pub optional_expression: Option<Expression>,
}

impl Return{
    pub fn create(optional_expression: Option<Expression>) -> Self {
        Self{
            optional_expression
        }
    }
}