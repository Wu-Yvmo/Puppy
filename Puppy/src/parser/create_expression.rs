use crate::ast::Expression;
use crate::parser::Parser;
use crate::token::{Operator, Punctuation, Token};

impl Parser {
    pub fn parse_create_expression(&mut self) -> Expression {
        self.parse_create_expression_10()
    }
    pub(self) fn parse_create_expression_10(&mut self) -> Expression {
        let mut left = self.parse_create_expression_9();
        while let Token::Operator(Operator::Assign) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_create_expression_9());
        }
        left
    }
    pub(self) fn parse_create_expression_9(&mut self) -> Expression {
        let mut left = self.parse_create_expression_8();
        while let Token::Operator(Operator::BooleanAnd) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_create_expression_8());
        }
        left
    }
    pub(self) fn parse_create_expression_8(&mut self) -> Expression {
        let mut left = self.parse_create_expression_7();
        while let Token::Operator(Operator::BooleanOr) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_create_expression_7());
        }
        left
    }
    pub(self) fn parse_create_expression_7(&mut self) -> Expression {
        let mut left = self.parse_create_expression_6();
        while let Token::Operator(Operator::NE)|Token::Operator(Operator::EQ) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_create_expression_6());
        }
        left
    }
    pub(self) fn parse_create_expression_6(&mut self) -> Expression {
        let mut left = self.parse_create_expression_5();
        while let Token::Operator(Operator::GT)|Token::Operator(Operator::LT)|Token::Operator(Operator::GE)|Token::Operator(Operator::LE) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_create_expression_5());
        }
        left
    }
    pub(self) fn parse_create_expression_5(&mut self) -> Expression {
        let mut left = self.parse_create_expression_4();
        while let Token::Operator(Operator::Add)|Token::Operator(Operator::Sub) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_create_expression_4());
        }
        left
    }
    pub(self) fn parse_create_expression_4(&mut self) -> Expression {
        let mut left = self.parse_create_expression_3();
        while let Token::Operator(Operator::Mul)|Token::Operator(Operator::Div)|Token::Operator(Operator::Rem) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_create_expression_3());
        }
        left
    }
    pub(self) fn parse_create_expression_3(&mut self) -> Expression {
        if let Token::Operator(Operator::Sub)|Token::Operator(Operator::Not) = self.current() {
            let operator = self.read();
            return Expression::create_unary(operator, self.parse_create_expression_2());
        }
        self.parse_create_expression_2()
    }
    pub(self) fn parse_create_expression_2(&mut self) -> Expression {
        let mut left = self.parse_create_expression_1();
        while let Token::Punctuation(Punctuation::LeftRoundBracket) = self.current() {
            self.read();
            let arguments = self.parse_create_arguments();
            self.read();
            left = Expression::create_call(left, arguments);
        }
        left
    }
    pub(self) fn parse_create_arguments(&mut self) -> Vec<Expression> {
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
    pub(self) fn parse_create_expression_1(&mut self) -> Expression {
        let mut left = self.parse_create_expression_0().unwrap();
        while let Token::Punctuation(Punctuation::Dot) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_create_expression_0().unwrap());
        }
        left
    }
    pub(self) fn parse_create_expression_0(&mut self) -> Result<Expression, String> {
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
    pub(self) fn parse_initializes(&mut self) -> Vec<(Token, Expression)> {
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
    pub(self) fn parse_initialize_item(&mut self) -> (Token, Expression) {
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