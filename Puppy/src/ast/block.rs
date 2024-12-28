use crate::ast::Statement;

#[derive(Debug, Clone)]
pub struct Block{
    pub items: Vec<Statement>,
}

impl Block{
    pub fn create(items: Vec<Statement>) -> Self {
        Self{
            items
        }
    }
}