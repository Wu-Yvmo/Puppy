//! 文法
//!
//! Program = StatementItem Program | StatementItem
//!
//! StatementItem = Class | If | While | For | Loop | Block | Let | Expression | Break | Continue | Return
//!
//! Class = ‘class’ '{' class_body '}'
//!
//! If
//!
//! While
//!
//! For
//!
//! Loop
//!
//! Block
//!
//! Let
//!
//! `Expression` = Expression_10 NewLine
//! * `Expression_10` = Expression_10 '=' Expression_9 | Expression_9
//! * `Expression_9` = Expression_9 '&&' Expression_8 | Expression_8
//! * `Expression_8` = Expression_8 '||' Expression_7 | Expression_7
//! * `Expression_7` = Expression_7 ('!=' | '==') Expression_6 | Expression_7
//! * `Expression_6` = Expression_6 ('>' | '<' | '>=' | '<=') Expression_5 | Expression_5
//! * `Expression_5` = Expression_5 ('+' | '-') Expression_5 | Expression_4
//! * `Expression_4` = Expression_4 ('*' | '/' | '%') Expression_3 | Expression_3
//! * `Expression_3` = ('-' | '!') Expression_3 | Expression_2
//! * `Expression_2` = Expression_2 '(' Parameters ')' | Expression_1
//! * `Parameters` = Parameters ',' Expression | Expression
//! * `Expression_1` = Expression_1 '.' Expression_0 | Expression_0
//! * `Expression_0` = '(' Expression ')' | Identifier | Literal
//!
//! `CreateExpression` = CreateExpression_10 NewLine
//! * `CreateExpression_10` = （下面和Expression一样，我就直接省略了）
//! * `CreateExpression_9` =
//! * `CreateExpression_8` =
//! * `CreateExpression_7` =
//! * `CreateExpression_6` =
//! * `CreateExpression_5` =
//! * `CreateExpression_4` =
//! * `CreateExpression_3` =
//! * `CreateExpression_2` =
//! * `CreateExpression_1` =
//! * `CreateExpression_0` = '(' Expression ')' | Identifier | Literal | Identifier '{' Initializes '}'
//! * `Initializes` = Initializes InitializeItem NewLine | InitializeItem NewLine
//! * `InitializeItem` = Identifier ':' CreateExpression
//!
//! Break
//!
//! Continue
//!
//! Return = 'return' \[Expression\] NewLine
//!
//! 面临的问题：如何处理;问题
//! Expression Let 末尾必须有换行符或EOF
//! 给出这样一个规定：Expression末尾必须有换行符或EOF，如果直接遇到EOF或换行符，说明这里没有表达式
//! Expression永远是要吃掉一个末尾的换行符的

use std::cmp::PartialEq;
use crate::ast::{Block, Break, Class, ClassData, ClassItem, ClassMethod, Continue, Expression, For, If, IfBranchItem, Let, Loop, Program, Return, StatementItem, TemplateDescribe, Type, While};
use crate::token::{Keyword, Operator, Punctuation, Token};
use crate::token_reader::TokenReader;

pub fn parse(code: &str) -> Program{
    todo!()
}

pub struct Parser{
    reader: TokenReader,
}

impl Parser{
    pub fn new(code: &str) -> Self{
        Self{
            reader: TokenReader::new(code),
        }
    }
}

// 转发对reader的方法调用
impl Parser{
    pub fn current(&mut self) -> Token{
        self.reader.current()
    }
    pub fn peek(&mut self) -> Token{
        self.reader.peek()
    }
    pub fn read(&mut self) -> Token{
        self.reader.read()
    }
    pub fn end(&mut self) -> bool {
        self.reader.end()
    }
}

impl Parser{
    pub fn parse(&mut self) -> Program {
        self.parse_program()
    }
}

impl Parser{
    pub fn parse_program(&mut self) -> Program {
        let mut items: Vec<StatementItem> = vec![];
        while !self.end() {
            items.push(self.parse_statement_item());
        }
        Program::create(items)
    }
    // 考虑解决return的表达式问题 b = a + b * c - d
    // b = a + b
    // 现在的方案是，表达式只能出现在一行里
    pub fn parse_statement_item(&mut self) -> StatementItem {
        match self.current() {
            Token::Keyword(keyword) => match keyword {
                Keyword::Class => StatementItem::Class(self.parse_class()),
                Keyword::If => StatementItem::If(self.parse_if()),
                Keyword::While => StatementItem::While(self.parse_while()),
                Keyword::For => StatementItem::For(self.parse_for()),
                Keyword::Loop => StatementItem::Loop(self.parse_loop()),
                Keyword::Let => StatementItem::Let(self.parse_let()),
                Keyword::Break => StatementItem::Break(self.parse_break()),
                Keyword::Continue => StatementItem::Continue(self.parse_continue()),
                Keyword::Return => StatementItem::Return(self.parse_return()),
                _ => panic!("unexpected"),
            }
            Token::Punctuation(punctuation) => match punctuation {
                Punctuation::LeftCurlyBracket => StatementItem::Block(self.parse_block()),
                _ => panic!("unexpected"),
            }
            _ => StatementItem::Expression(self.parse_expression()),
        }
    }
}

impl PartialEq for Break {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

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
        Class::create(items)
    }
    pub fn parse_class_item(&mut self) -> ClassItem {
        if let Token::Keyword(Keyword::Fn) = self.current() {
            return ClassItem::Method(self.parse_class_item_class_method());
        }
        return ClassItem::Data(self.parse_class_item_class_data());
    }
    // name: String
    pub fn parse_class_item_class_data(&mut self) -> ClassData {
        // 读取变量名
        let name = self.read();
        // 读取':'
        self.read();
        // 解析变量类型
        let data_type = self.parse_type();
        // 创建ClassData
        ClassData::create(name, data_type)
    }
    // fn hello(self, name: String, age: Number) Number {
    //     return 1 + 2
    // }
    //
    pub fn parse_class_item_class_method(&mut self) -> ClassMethod {
        // 读取'fn'
        self.read();
        // 读取方法名
        let name = self.read();
        // 读取左括号
        self.read();
        // 解析形参列表
        todo!()
    }
}

impl Parser {
    pub fn parse_type(&mut self) -> Type {
        if let Token::Keyword(Keyword::TheSelf) = self.current() {
            self.read();
            // todo
        }
        let name = self.read();
        let optional_template_describe = self.parse_template_describe();
        // 构造Type
        Type::create_normal(name, optional_template_describe)
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

impl Parser{
    pub fn parse_while(&mut self) -> While {
        // 读取关键字while
        self.read();
        // 读取条件
        let condition = self.parse_expression();
        // 解析body
        let body = self.parse_block();
        // 构造while
        While{
            condition,
            body
        }
    }
}

impl Parser{
    pub fn parse_for(&mut self) -> For {
        // 读取for
        self.read();
        // 解析let表达式
        let initialize = self.parse_let();
        // 读取;
        self.read();
        // 解析条件表达式
        let condition = self.parse_expression();
        // 读取;
        self.read();
        // 解析步进表达式
        let step = self.parse_expression();
        // 解析循环体
        let body = self.parse_block();
        // 构造For
        For{
            initialize,
            condition,
            step,
            body,
        }
    }
}

impl Parser{
    pub fn parse_loop(&mut self) -> Loop {
        // 读取关键字loop
        self.read();
        // 解析Block
        let body = self.parse_block();
        // 构造Loop
        Loop {
            body
        }
    }
}

impl Parser {
    pub fn parse_block(&mut self) -> Block {
        // 读取{
        self.read();
        // 处理Block内部的所有Statement
        let mut items: Vec<StatementItem> = vec![];
        self.jump_newlines_and_eof();
        while let Token::Punctuation(Punctuation::RightCurlyBracket) = self.current() {
            items.push(self.parse_statement_item());
            self.jump_newlines_and_eof()
        }
        // 读取}
        self.read();
        // 构造Block
        Block{
            items
        }
    }
}

impl Parser{
    pub fn parse_let(&mut self) -> Let{
        // 读取let关键字
        self.read();
        // 读取变量名
        let name = self.read();
        // 读取=
        self.read();
        // 读取初始化表达式
        let initialize_value = self.parse_expression();
        // 构造Let
        Let{
            name,
            initialize_value,
        }
    }
}

impl Parser{
    pub fn parse_expression(&mut self) -> Expression {
        let expression = self.parse_expression_10();
        self.read();
        expression
    }
    pub fn parse_expression_10(&mut self) -> Expression {
        let mut left = self.parse_expression_9();
        while let Token::Operator(Operator::Assign) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_expression_9());
        }
        left
    }
    pub fn parse_expression_9(&mut self) -> Expression {
        let mut left = self.parse_expression_8();
        while let Token::Operator(Operator::BooleanAnd) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_expression_8());
        }
        left
    }
    pub fn parse_expression_8(&mut self) -> Expression {
        let mut left = self.parse_expression_7();
        while let Token::Operator(Operator::BooleanOr) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_expression_7());
        }
        left
    }
    pub fn parse_expression_7(&mut self) -> Expression {
        let mut left = self.parse_expression_6();
        while let Token::Operator(Operator::EQ)|Token::Operator(Operator::NE) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_expression_6());
        }
        left
    }
    pub fn parse_expression_6(&mut self) -> Expression {
        let mut left = self.parse_expression_5();
        while let Token::Operator(Operator::GT)|Token::Operator(Operator::LT)|Token::Operator(Operator::GE)|Token::Operator(Operator::LE) = self.current() {
            let operator = self.current();
            left = Expression::create_binary(left, operator, self.parse_expression_5());
        }
        left
    }
    pub fn parse_expression_5(&mut self) -> Expression {
        let mut left = self.parse_expression_4();
        while let Token::Operator(Operator::Add)|Token::Operator(Operator::Sub) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_expression_4());
        }
        left
    }
    pub fn parse_expression_4(&mut self) -> Expression {
        let mut left = self.parse_expression_3();
        while let Token::Operator(Operator::Mul)|Token::Operator(Operator::Div)|Token::Operator(Operator::Rem) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_expression_3());
        }
        left
    }
    pub fn parse_expression_3(&mut self) -> Expression {
        if let Token::Operator(Operator::Sub)|Token::Operator(Operator::Not) = self.current() {
            let operator = self.read();
            return Expression::create_unary(operator, self.parse_expression_3());
        }
        self.parse_expression_2()
    }
    pub fn parse_expression_2(&mut self) -> Expression {
        let mut function = self.parse_expression_1();
        while let Token::Punctuation(Punctuation::LeftRoundBracket) = self.current() {
            self.read();
            let arguments = self.parse_arguments();
            self.read();
            function = Expression::create_call(function, arguments);
        }
        function
    }
    pub fn parse_arguments(&mut self) -> Vec<Expression> {
        match self.current() {
            Token::Punctuation(Punctuation::RightRoundBracket) => vec![],
            _ => {
                let mut arguments = vec![self.parse_expression()];
                while let Token::Punctuation(Punctuation::Comma) = self.current() {
                    self.read();
                    arguments.push(self.parse_expression());
                }
                arguments
            }
        }
    }
    pub fn parse_expression_1(&mut self) -> Expression {
        let mut left = self.parse_expression_0().unwrap();
        while let Token::Punctuation(Punctuation::Dot) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_expression_0().unwrap());
        }
        left
    }
    pub fn parse_expression_0(&mut self) -> Result<Expression, String> {
        match self.current() {
            Token::Punctuation(Punctuation::LeftRoundBracket) => {
                self.read();
                let sub = self.parse_expression();
                self.read();
                Ok(Expression::create_pack(sub))
            }
            Token::Identifier {
                literal
            } => Ok(Expression::create_use(self.read())),
            Token::Literal(literal) => {
                self.read();
                Ok(Expression::create_literal(literal))
            }
            _ => Err("unexpected".to_string())
        }
    }
}

impl Parser {
    pub fn parse_create_expression(&mut self) -> Expression {
        self.parse_create_expression_10()
    }
    pub fn parse_create_expression_10(&mut self) -> Expression {
        let mut left = self.parse_create_expression_9();
        while let Token::Operator(Operator::Assign) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_create_expression_9());
        }
        left
    }
    pub fn parse_create_expression_9(&mut self) -> Expression {
        let mut left = self.parse_create_expression_8();
        while let Token::Operator(Operator::BooleanAnd) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_create_expression_8());
        }
        left
    }
    pub fn parse_create_expression_8(&mut self) -> Expression {
        let mut left = self.parse_create_expression_7();
        while let Token::Operator(Operator::BooleanOr) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_create_expression_7());
        }
        left
    }
    pub fn parse_create_expression_7(&mut self) -> Expression {
        let mut left = self.parse_create_expression_6();
        while let Token::Operator(Operator::NE)|Token::Operator(Operator::EQ) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_create_expression_6());
        }
        left
    }
    pub fn parse_create_expression_6(&mut self) -> Expression {
        let mut left = self.parse_create_expression_5();
        while let Token::Operator(Operator::GT)|Token::Operator(Operator::LT)|Token::Operator(Operator::GE)|Token::Operator(Operator::LE) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_create_expression_5());
        }
        left
    }
    pub fn parse_create_expression_5(&mut self) -> Expression {
        let mut left = self.parse_create_expression_4();
        while let Token::Operator(Operator::Add)|Token::Operator(Operator::Sub) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_create_expression_4());
        }
        left
    }
    pub fn parse_create_expression_4(&mut self) -> Expression {
        let mut left = self.parse_create_expression_3();
        while let Token::Operator(Operator::Mul)|Token::Operator(Operator::Div)|Token::Operator(Operator::Rem) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_create_expression_3());
        }
        left
    }
    pub fn parse_create_expression_3(&mut self) -> Expression {
        if let Token::Operator(Operator::Sub)|Token::Operator(Operator::Not) = self.current() {
            let operator = self.read();
            return Expression::create_unary(operator, self.parse_create_expression_2());
        }
        self.parse_create_expression_2()
    }
    pub fn parse_create_expression_2(&mut self) -> Expression {
        let mut left = self.parse_create_expression_1();
        while let Token::Punctuation(Punctuation::LeftRoundBracket) = self.current() {
            self.read();
            let arguments = self.parse_create_arguments();
            self.read();
            left = Expression::create_call(left, arguments);
        }
        left
    }
    pub fn parse_create_arguments(&mut self) -> Vec<Expression> {
        if let Token::Punctuation(Punctuation::RightRoundBracket) = self.current() {
            return vec![];
        }
        let mut arguments = vec![self.parse_create_expression()];
        while let Token::Punctuation(Punctuation::Comma) = self.current() {
            self.read();
            arguments.push(self.parse_create_expression());
        }
        arguments
    }
    pub fn parse_create_expression_1(&mut self) -> Expression {
        let mut left = self.parse_create_expression_0().unwrap();
        while let Token::Punctuation(Punctuation::Dot) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_create_expression_0().unwrap());
        }
        left
    }
    pub fn parse_create_expression_0(&mut self) -> Result<Expression, String> {
        match self.current() {
            Token::Punctuation(Punctuation::LeftRoundBracket) => {
                // 读取 '('
                self.read();
                // 读取表达式
                let sub = self.parse_create_expression();
                // 读取')'
                self.read();
                // 构造返回值
                Ok(Expression::create_pack(sub))
            }
            Token::Identifier {..} => {
                // 读取当前token
                let name = self.read();
                // 类构造的情况
                if let Token::Punctuation(Punctuation::LeftCurlyBracket) = self.current() {
                    // 读取'{'
                    self.read();
                    // 解析initialize
                    let initializes = self.parse_initializes();
                    // 读取'}'
                    self.read();
                    // 构造返回值
                    return Ok(Expression::create_create(name, initializes));
                }
                Ok(Expression::create_use(name))
            }
            Token::Literal(literal) => {
                self.read();
                Ok(Expression::create_literal(literal))
            }
            _ => Err("unexpected".to_string()),
        }
    }
    pub fn parse_initializes(&mut self) -> Vec<(Token, Expression)> {
        let mut initializes: Vec<(Token, Expression)> = vec![];
        self.jump_newlines_and_eof();
        loop {
            if let Token::Punctuation(Punctuation::RightCurlyBracket) = self.current() {
                break;
            }
            initializes.push(self.parse_initialize_item());
            self.jump_newlines_and_eof();
        }
        initializes
    }
    pub fn parse_initialize_item(&mut self) -> (Token, Expression) {
        // 读取变量名
        let name = self.read();
        // 读取:
        self.read();
        // 读取表达式
        let expression = self.parse_create_expression();
        // 构造返回值
        (name, expression)
    }
}

impl Parser{
    pub fn parse_break(&mut self) -> Break {
        self.read();
        Break{}
    }
}

impl Parser{
    pub fn parse_continue(&mut self) -> Continue {
        self.read();
        Continue{}
    }
}

impl Parser{
    pub fn parse_return(&mut self) -> Return {
        self.read();
        if let Token::NewLine|Token::EndOfFile = self.current() {
            self.read();
            return Return{
                optional_expression: None,
            }
        }
        let expression = self.parse_expression();
        // 构造Return并返回
        Return{
            optional_expression: Some(expression)
        }
    }
}

impl Parser{
    pub fn jump_newlines_and_eof(&mut self) {
        while let Token::NewLine|Token::EndOfFile = self.current() {
            self.read();
        }
    }
}