use std::cmp::Ordering;
use crate::token::Token;
use regex::{Match, Regex};
use crate::token::Keyword::{Break, Class, Else, Fn, For, If, Let, Loop, Return, While, Continue, TheSelf};
use crate::token::Literal::{Boolean, Number, String};
use crate::token::Operator::{Add, Assign, BooleanAnd, BooleanOr, Div, EQ, GE, GT, LE, LT, Mul, NE, Not, Rem, Sub};
use crate::token::Punctuation::{Colon, Comma, Dot, LeftCurlyBracket, LeftRoundBracket, LeftSquareBracket, RightCurlyBracket, RightRoundBracket, Semicolon};

pub fn lex(code: &str) -> Vec<Token> {
    let mut tokens = Lexer::new(code).lex();
    tokens.push(Token::create_end_of_file());
    tokens
}

/// .0字段是模式 .1字段用于生成Token
struct Lexer<'a>{
    keyword_patterns: Vec<(Regex, fn(&str) -> Token)>,
    literal_patterns: Vec<(Regex, fn(&str) -> Token)>,
    operator_patterns: Vec<(Regex, fn(&str) -> Token)>,
    punctuation_patterns: Vec<(Regex, fn(&str) -> Token)>,
    identifier_pattern: Vec<(Regex, fn(&str) -> Token)>,
    newline_pattern: Vec<(Regex, fn(&str) -> Token)>,
    code: &'a str,
}

impl<'a> Lexer<'a>{
    pub fn new(code: &'a str) -> Self {
        Self{
            identifier_pattern: vec![
                (Regex::new("^[a-zA-Z_][a-zA-Z0-9_]*").unwrap(), |identifier|-> Token{
                    Token::Identifier {
                        literal: identifier.to_string(),
                    }
                }),
            ],
            literal_patterns: vec![
                (Regex::new("^\"[a-zA-Z0-9]*\"").unwrap(), |literal|->Token{
                    Token::Literal(String {
                        literal: literal[1..literal.len()-1].to_string(),
                    })
                }),
                (Regex::new("^[0-9]+").unwrap(), |literal|->Token{
                    Token::Literal(Number {
                        literal: {
                            let mut val = 0.0;
                            for c in literal.chars() {
                                val *= 10.0;
                                val += (c as usize - '0' as usize) as f64;
                            }
                            val
                        }
                    })
                }),
                (Regex::new("^((true)|(false))").unwrap(), |literal|->Token{
                    match literal{
                        "true" => Token::Literal(Boolean {
                            literal: true,
                        }),
                        "false" => Token::Literal(Boolean {
                            literal: true,
                        }),
                        _ => panic!("unexpected")
                    }
                }),
            ],
            operator_patterns: vec![
                (Regex::new("^\\+").unwrap(), |_|{
                    Token::Operator(Add)
                }),
                (Regex::new("^-").unwrap(), |_|{
                    Token::Operator(Sub)
                }),
                (Regex::new("^\\*").unwrap(), |_|{
                    Token::Operator(Mul)
                }),
                (Regex::new("^/").unwrap(), |_|{
                    Token::Operator(Div)
                }),
                (Regex::new("^%").unwrap(), |_|{
                    Token::Operator(Rem)
                }),
                (Regex::new("^!").unwrap(), |_|{
                    Token::Operator(Not)
                }),
                (Regex::new("^=").unwrap(), |_|{
                    Token::Operator(Assign)
                }),
                (Regex::new("^>").unwrap(), |_|{
                    Token::Operator(GT)
                }),
                (Regex::new("^<").unwrap(), |_|{
                    Token::Operator(LT)
                }),
                (Regex::new("^>=").unwrap(), |_|{
                    Token::Operator(GE)
                }),
                (Regex::new("^<=").unwrap(), |_|{
                    Token::Operator(LE)
                }),
                (Regex::new("^==").unwrap(), |_|{
                    Token::Operator(EQ)
                }),
                (Regex::new("^!=").unwrap(), |_|{
                    Token::Operator(NE)
                }),
                (Regex::new("^&&").unwrap(), |_|{
                    Token::Operator(BooleanAnd)
                }),
                (Regex::new("^\\|\\|").unwrap(), |_|{
                    Token::Operator(BooleanOr)
                }),
            ],
            keyword_patterns: vec![
                (Regex::new("^if").unwrap(), |_|{
                    Token::Keyword(If)
                }),
                (Regex::new("^else").unwrap(), |_|{
                    Token::Keyword(Else)
                }),
                (Regex::new("^for").unwrap(), |_|{
                    Token::Keyword(For)
                }),
                (Regex::new("^while").unwrap(), |_|{
                    Token::Keyword(While)
                }),
                (Regex::new("^loop").unwrap(), |_|{
                    Token::Keyword(Loop)
                }),
                (Regex::new("^fn").unwrap(), |_|{
                    Token::Keyword(Fn)
                }),
                (Regex::new("^class").unwrap(), |_|{
                    Token::Keyword(Class)
                }),
                (Regex::new("^let").unwrap(), |_|{
                    Token::Keyword(Let)
                }),
                (Regex::new("^return").unwrap(), |_|{
                    Token::Keyword(Return)
                }),
                (Regex::new("^break").unwrap(), |_|{
                    Token::Keyword(Break)
                }),
                (Regex::new("^continue").unwrap(), |_|{
                    Token::Keyword(Continue)
                }),
                (Regex::new("^self").unwrap(), |_|{
                    Token::Keyword(TheSelf)
                }),
            ],
            punctuation_patterns: vec![
                (Regex::new("^\\{").unwrap(), |_|{
                    Token::Punctuation(LeftCurlyBracket)
                }),
                (Regex::new("^}").unwrap(), |_|{
                    Token::Punctuation(RightCurlyBracket)
                }),
                (Regex::new("^\\(").unwrap(), |_|{
                    Token::Punctuation(LeftRoundBracket)
                }),
                (Regex::new("^\\)").unwrap(), |_|{
                    Token::Punctuation(RightRoundBracket)
                }),
                (Regex::new("^\\[").unwrap(), |_|{
                    Token::Punctuation(LeftSquareBracket)
                }),
                (Regex::new("^]").unwrap(), |_|{
                    Token::Punctuation(RightRoundBracket)
                }),
                (Regex::new("^\\.").unwrap(), |_|{
                    Token::Punctuation(Dot)
                }),
                (Regex::new("^,").unwrap(), |_|{
                    Token::Punctuation(Comma)
                }),
                (Regex::new("^;").unwrap(), |_|{
                    Token::Punctuation(Semicolon)
                }),
                (Regex::new("^:").unwrap(), |_|{
                    Token::Punctuation(Colon)
                }),
            ],
            newline_pattern: vec![
                (Regex::new("^\n").unwrap(), |_|{
                    Token::NewLine
                }),
            ],
            code,
        }
    }
}

impl<'a> Lexer<'a>{
    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        self.jump_whites();
        while self.code.len() > 0 {
            tokens.push(self.tokenize_one());
            self.jump_whites();
        }
        tokens
    }
    pub fn jump_whites(&mut self) {
        while self.code.len() > 0 && (self.code.as_bytes()[0] == ' ' as u8 ||
            self.code.as_bytes()[0] == ' ' as u8 ||
            self.code.as_bytes()[0] == '\r' as u8) {
            self.code = &self.code[1..];
        }
    }
    // 新的lex_one
    fn tokenize_one(&mut self) -> Token {
        let possible_tokens = vec![
            self.tokenize_by_patterns(&self.keyword_patterns),
            self.tokenize_by_patterns(&self.literal_patterns),
            self.tokenize_by_patterns(&self.operator_patterns),
            self.tokenize_by_patterns(&self.punctuation_patterns),
            self.tokenize_by_patterns(&self.identifier_pattern),
            self.tokenize_by_patterns(&self.newline_pattern),
        ];
        for token in possible_tokens {
            if let Some((token, len)) = token {
                self.code = &self.code[len..];
                return token;
            }
        }
        panic!("no valid token: {}", self.code)
    }
    fn tokenize_by_patterns(&self, patterns: &Vec<(Regex, fn(&str) -> Token)>) -> Option<(Token, usize)> {
        // 1.收集所有的匹配结果和构建函数
        let match_results= {
            let mut match_results: Vec<(Match, &fn(&str) -> Token)> = vec![];
            for (pattern, constructor) in patterns {
                if let Some(success_match) = pattern.find(self.code) {
                    match_results.push((success_match, constructor));
                }
            }
            match_results
        };
        if match_results.len() == 0 {
            return None
        }
        // 2.选出最长的
        let max_match = match_results.iter().max_by(|l, r|{
            if l.0.len() < r.0.len() {
                return Ordering::Less
            }
            if l.0.len() < r.0.len() {
                return Ordering::Equal
            }
            Ordering::Greater
        }).unwrap().clone();
        let token = max_match.1(max_match.0.as_str());
        Some((token, max_match.0.len()))
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_lex_generatial() {
        let code = "let abc12 \"hello\" 1234 true false + - * / ! = > < >= <= == != if else for while loop fn class let return break continue {}()[] . && ||\n , :;";
        let tokens = lex(code);
        for token in &tokens {
            println!("{}", token.dump())
        }
    }
    #[test]
    fn test_lex_identifier() {
        let code = "abc def gh";
        let tokens = lex(code);
        assert_eq!(tokens.len(), 3);
    }
    #[test]
    fn test_lex_literal() {
        let code = "\"abcdef\" 1234 true false";
        let tokens = lex(code);
        todo!()
    }
    #[test]
    fn test_lex_operator() {
        //
    }
    #[test]
    fn test_lex_keyword() {
        //
    }
}