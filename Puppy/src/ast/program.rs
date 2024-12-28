use crate::ast::Statement;

#[derive(Debug)]
pub struct Program{
    pub items: Vec<Statement>,
}

impl Program{
    pub fn create(items: Vec<Statement>) -> Self {
        Self{
            items
        }
    }
}