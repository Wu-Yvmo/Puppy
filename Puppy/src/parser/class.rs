use crate::ast::{Class, ClassData, ClassItem, ClassMethod, Expression, ParameterItem, Type};
use crate::parser::Parser;
use crate::token::{Keyword, Punctuation, Token};

impl Parser{
    // 这门语言应该是静态还是动态类型？
    // 静态类型！必须是静态类型
    // class item分为：ClassData和ClassMethod
    // 问题：如何实现rust那样的Class{初始化}的语法？
    // 是不是创造两套expression文法？let使用的是可能构造变量的expression
    // 问题出在：
    // if Param{...} {...}
    // if Param {...}
    // 如何在语法分析阶段得知Param后的括号是要初始化还是Block的一部分？
    pub fn parse_class(&mut self) -> Class {
        // 读取class关键字
        self.read();
        // 读取类名
        let name = self.read();
        // 读取{
        self.read();
        // 读取类内的所有项
        let mut items: Vec<ClassItem> = vec![];
        self.jump_newlines_and_eof();
        loop {
            if let Token::Punctuation(Punctuation::RightCurlyBracket) = self.current() {
                break
            }
            items.push(self.parse_class_item());
            self.jump_newlines_and_eof();
        }
        // 读取}
        self.read();
        // 构造class
        Class::create(name, items)
    }
    pub(self) fn parse_class_item(&mut self) -> ClassItem {
        if let Token::Keyword(Keyword::Fn) = self.current() {
            return ClassItem::Method(self.parse_class_item_class_method());
        }
        return ClassItem::Data(self.parse_class_item_class_data());
    }
    // name: String
    pub(self) fn parse_class_item_class_data(&mut self) -> ClassData {
        // 读取变量名
        let name = self.read();
        // 读取':'
        self.read();
        // 解析变量类型
        let data_type = self.parse_type();
        // 创建ClassData
        ClassData::create(name, data_type)
    }
    pub(self) fn parse_class_item_class_method(&mut self) -> ClassMethod {
        // 读取'fn'
        self.read();
        // 读取方法名
        let name = self.read();
        // 读取左括号
        self.read();
        // 解析形参列表
        let in_paramaeters = self.parse_class_item_class_method_parameters();
        // 读取右括号
        self.read();
        // 读取返回值
        let out_parameter = self.parse_type();
        // 读取方法体
        let body = self.parse_block();
        // 构造方法
        ClassMethod::create(name, in_paramaeters, out_parameter, body)
    }
    pub(self) fn parse_class_item_class_method_parameters(&mut self) -> Vec<ParameterItem> {
        if let Token::Punctuation(Punctuation::RightRoundBracket) = self.current() {
            return vec![];
        }
        let mut parameters = vec![self.parse_class_item_class_method_parameter()];
        while let Token::Punctuation(Punctuation::Comma) = self.current() {
            self.read();
            parameters.push(self.parse_class_item_class_method_parameter());
        }
        parameters
    }
    // 注意，这里证明class的结构需要大改
    pub(self) fn parse_class_item_class_method_parameter(&mut self) -> ParameterItem {
        // 如果当前token是Self，说明它是self
        if let Token::Keyword(Keyword::TheSelf) = self.current() {
            self.read();
            return ParameterItem::create_self()
        }
        // 读取名字
        let name = self.read();
        // 读取':'
        self.read();
        // 解析类型
        let data_type = self.parse_type();
        // 构造parameter_item
        ParameterItem::create_normal(name, data_type)
    }
}
