use crate::ast::Expression;

#[derive(Debug, Clone)]
pub struct Pack{
    pub sub: Box<Expression>
}

impl Pack{
    pub fn create(sub: Expression) -> Self {
        Self{
            sub: Box::new(sub),
        }
    }
}