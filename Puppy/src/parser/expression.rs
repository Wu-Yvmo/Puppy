use crate::ast::Expression;
use crate::parser::Parser;
use crate::token::{Operator, Punctuation, Token};

impl Parser{
    pub fn parse_expression(&mut self) -> Expression {
        let expression = self.parse_expression_10();
        self.read();
        expression
    }
    pub(self) fn parse_expression_10(&mut self) -> Expression {
        let mut left = self.parse_expression_9();
        while let Token::Operator(Operator::Assign) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_expression_9());
        }
        left
    }
    pub(self) fn parse_expression_9(&mut self) -> Expression {
        let mut left = self.parse_expression_8();
        while let Token::Operator(Operator::BooleanAnd) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_expression_8());
        }
        left
    }
    pub(self) fn parse_expression_8(&mut self) -> Expression {
        let mut left = self.parse_expression_7();
        while let Token::Operator(Operator::BooleanOr) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_expression_7());
        }
        left
    }
    pub(self) fn parse_expression_7(&mut self) -> Expression {
        let mut left = self.parse_expression_6();
        while let Token::Operator(Operator::EQ)|Token::Operator(Operator::NE) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_expression_6());
        }
        left
    }
    pub(self) fn parse_expression_6(&mut self) -> Expression {
        let mut left = self.parse_expression_5();
        while let Token::Operator(Operator::GT)|Token::Operator(Operator::LT)|Token::Operator(Operator::GE)|Token::Operator(Operator::LE) = self.current() {
            let operator = self.current();
            left = Expression::create_binary(left, operator, self.parse_expression_5());
        }
        left
    }
    pub(self) fn parse_expression_5(&mut self) -> Expression {
        let mut left = self.parse_expression_4();
        while let Token::Operator(Operator::Add)|Token::Operator(Operator::Sub) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_expression_4());
        }
        left
    }
    pub(self) fn parse_expression_4(&mut self) -> Expression {
        let mut left = self.parse_expression_3();
        while let Token::Operator(Operator::Mul)|Token::Operator(Operator::Div)|Token::Operator(Operator::Rem) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_expression_3());
        }
        left
    }
    pub(self) fn parse_expression_3(&mut self) -> Expression {
        if let Token::Operator(Operator::Sub)|Token::Operator(Operator::Not) = self.current() {
            let operator = self.read();
            return Expression::create_unary(operator, self.parse_expression_3());
        }
        self.parse_expression_2()
    }
    pub(self) fn parse_expression_2(&mut self) -> Expression {
        let mut function = self.parse_expression_1();
        while let Token::Punctuation(Punctuation::LeftRoundBracket) = self.current() {
            self.read();
            let arguments = self.parse_arguments();
            self.read();
            function = Expression::create_call(function, arguments);
        }
        function
    }
    pub(self) fn parse_arguments(&mut self) -> Vec<Expression> {
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
    pub(self) fn parse_expression_1(&mut self) -> Expression {
        let mut left = self.parse_expression_0().unwrap();
        while let Token::Punctuation(Punctuation::Dot) = self.current() {
            let operator = self.read();
            left = Expression::create_binary(left, operator, self.parse_expression_0().unwrap());
        }
        left
    }
    pub(self) fn parse_expression_0(&mut self) -> Result<Expression, String> {
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