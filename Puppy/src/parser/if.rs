use crate::ast::{If, IfBranchItem};
use crate::parser::Parser;
use crate::token::{Keyword, Token};

impl Parser{
    pub fn parse_if(&mut self) -> If {
        // 读取if关键字
        self.read();
        // 解析条件表达式
        let condition = self.parse_expression();
        // 解析true分支
        let true_branch = self.parse_block();
        // 解析可选的else分支
        let optional_false_branch = ||->Option<IfBranchItem>{
            // 有else分支
            if let Token::Keyword(Keyword::Else) = self.current() {
                // 读取关键字else
                self.read();
                let false_branch = Some(self.parse_if_branch_item());
                return false_branch
            }
            // 没有else分支
            return None
        }();
        // 构造If
        If::create(condition, true_branch, optional_false_branch)
    }
    pub fn parse_if_branch_item(&mut self) -> IfBranchItem {
        // 当前token是if
        if let Token::Keyword(Keyword::If) = self.current() {
            return IfBranchItem::create_if(self.parse_if());
        }
        // 当前token是 {
        return IfBranchItem::create_block(self.parse_block())
    }
}