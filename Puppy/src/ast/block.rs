use crate::ast::StatementItem;

#[derive(Debug)]
pub struct Block{
    pub items: Vec<StatementItem>,
}

impl Block{
    pub fn create(items: Vec<StatementItem>) -> Self {
        Self{
            items
        }
    }
}