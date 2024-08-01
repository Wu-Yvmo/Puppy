use crate::ast::StatementItem;

#[derive(Debug)]
pub struct Program{
    pub items: Vec<StatementItem>,
}

impl Program{
    pub fn create(items: Vec<StatementItem>) -> Self {
        Self{
            items
        }
    }
}