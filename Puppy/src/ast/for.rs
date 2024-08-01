use crate::ast::{Block, Expression, Let};

#[derive(Debug)]
pub struct For{
    pub initialize: Let,
    pub condition: Expression,
    pub step: Expression,
    pub body: Block,
}

impl For {
    pub fn create_for(initialize: Let, condition: Expression, step: Expression, body: Block) -> Self {
        Self{
            initialize,
            condition,
            step,
            body
        }
    }
}