use crate::ast::{Block, Expression};

pub struct If{
    pub condition: Expression,
    pub true_branch: Block,
    pub optional_false_branch: Option<IfBranchItem>,
}

impl If {
    pub fn create(condition: Expression, true_branch: Block, optional_false_branch: Option<IfBranchItem>) -> Self {
        If{
            condition,
            true_branch,
            optional_false_branch,
        }
    }
}

pub enum IfBranchItem{
    If(Box<If>),
    Block(Block),
}

impl IfBranchItem {
    pub fn create_if(if_branch: If) -> Self {
        Self::If(Box::new(if_branch))
    }
    pub fn create_block(block: Block) -> Self {
        Self::Block(block)
    }
}