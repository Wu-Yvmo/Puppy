use crate::ast::StatementItem;

pub struct Program{
    items: Vec<StatementItem>,
}

impl Program{
    pub fn create(items: Vec<StatementItem>) -> Self {
        Self{
            items
        }
    }
}