use crate::ast::{TemplateDescribe, Type};
use crate::parser::Parser;
use crate::token::{Keyword, Punctuation, Token};

impl Parser {
    pub fn parse_type(&mut self) -> Type {
        // 读取类型名
        let name = self.read();
        // 解析范型参数
        let optional_template_describe = self.parse_template_describe();
        // 构造Type
        Type::create(name, optional_template_describe)
    }
    pub fn parse_template_describe(&mut self) -> Option<TemplateDescribe> {
        if let Token::Punctuation(Punctuation::LeftSquareBracket) = self.current() {
            self.read();
            let items= self.parse_template_describe_items();
            self.read();
            return Some(TemplateDescribe::create_template_describe(items))
        }
        None
    }
    pub fn parse_template_describe_items(&mut self) -> Vec<Type> {
        if let Token::Punctuation(Punctuation::RightSquareBracket) = self.current() {
            return vec![];
        }
        let mut items = vec![self.parse_type()];
        while let Token::Punctuation(Punctuation::Comma) = self.current() {
            self.read();
            items.push(self.parse_type());
        }
        items
    }
}