use crate::ast::{Block, Expression};

pub struct While{
    pub condition: Expression,
    pub body: Block,
}

impl While {
    pub fn create(condition: Expression, body: Block) -> Self {
        Self{
            condition,
            body,
        }
    }
}