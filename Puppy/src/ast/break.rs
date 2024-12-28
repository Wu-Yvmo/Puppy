use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Break;

impl Break{
    /// 提供了一个传入一个token并且构造Break的构造函数
    pub fn create(_: Token) -> Self {
        Self
    }
}