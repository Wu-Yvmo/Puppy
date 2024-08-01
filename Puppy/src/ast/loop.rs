use crate::ast::Block;

#[derive(Debug)]
pub struct Loop{
    pub body: Block,
}

impl Loop{
    pub fn create(body: Block) -> Self {
        Self{
            body
        }
    }
}