//! token的类型分别为：
//! * Identifier：标识符
//! * Literal：字面值
//!     * `String`：字符串
//!     * `Number`：数字
//!     * `Boolean`：布尔值，取值true/false
//! * `Operato`r：运算符
//!     * `+` `Add`
//!     * `-` `Sub`
//!     * `*` `Mul`
//!     * `/` `Div`
//!     * `!` `Not`
//!     * `=` `Assign`
//!     * `>` `GT`
//!     * `<` `LT`
//!     * `>=` `GE`
//!     * `<=` `LE`
//!     * `==` `EQ`
//!     * `!=` `NE`
//! * Keyword：关键字
//!     * `if`
//!     * `else`
//!     * `for`
//!     * `while`
//!     * `loop`
//!     * `fn`
//!     * `class`
//!     * `let`
//!     * `return`
//!     * `break`
//!     * `continue`
//!     * `self`
//! * Punctuation关键字：
//!     * `{`
//!     * `}`
//!     * `(`
//!     * `)`
//!     * `[`
//!     * `]`
//!     * `&&`
//!     * `||`
//!     * `.` : `Dot`
//!     * `,` : `Semicolon`
//!     * `:` : `Colon`
//! * `\n` : `NewLine`
//! * `文件结尾` : `EndOfFile`

#[derive(Clone)]
pub enum Token {
    Identifier{literal: String},
    Literal(Literal),
    Operator(Operator),
    Keyword(Keyword),
    Punctuation(Punctuation),
    NewLine,
    EndOfFile,
}

/// 字面值
/// 字符串 数字 布尔值
#[derive(Clone)]
pub enum Literal{
    String{
        literal: String,
    },
    Number{
        literal: f64,
    },
    Boolean {
        literal: bool,
    },
}

/// 运算符
/// + - * / % !
/// =
/// > < <= >= == !=
#[derive(Clone)]
pub enum Operator{
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Not,
    Assign,
    GT,
    LT,
    GE,
    LE,
    EQ,
    NE,
    BooleanAnd,
    BooleanOr,
}

/// 关键字
/// if else for while loop fn class let return break
#[derive(Clone)]
pub enum Keyword{
    If,
    Else,
    For,
    While,
    Loop,
    Fn,
    Class,
    Let,
    Return,
    Break,
    Continue,
    TheSelf,
}

#[derive(Clone)]
pub enum Punctuation{
    LeftCurlyBracket,
    RightCurlyBracket,
    LeftRoundBracket,
    RightRoundBracket,
    LeftSquareBracket,
    RightSquareBracket,
    Dot,
    Comma,
    Semicolon,
    Colon,
}

impl Token{
    pub fn dump(&self) -> String {
        match self {
            Token::Identifier {
                literal
            } => format!("Identifier_{{literal:[{}]}}", literal),
            Token::Literal(literal) => format!("Literal_{}", literal.dump()),
            Token::Operator(operator) => format!("Operator_{}", operator.dump()),
            Token::Keyword(keyword) => format!("Keyword_{}", keyword.dump()),
            Token::Punctuation(punctuation) => format!("Punctuation_{}", punctuation.dump()),
            Token::NewLine => "NewLine".to_string(),
            Token::EndOfFile => "EndOfFile".to_string(),
        }
    }
    pub fn create_end_of_file() -> Self {
        Token::EndOfFile
    }
}

impl Literal{
    pub fn dump(&self) -> String {
        match self {
            Literal::String {
                literal
            } => format!("String{{literal:\"{}\"}}", literal),
            Literal::Number {
                literal
            } => format!("Number{{literal:{}}}", literal),
            Literal::Boolean {
                literal
            } => format!("Boolean{{literal:{}}}", literal),
        }
    }
}

impl Operator{
    pub fn dump(&self) -> String {
        match self {
            Operator::Add => "Add".to_string(),
            Operator::Sub => "Sub".to_string(),
            Operator::Mul => "Mul".to_string(),
            Operator::Div => "Div".to_string(),
            Operator::Rem => "Rem".to_string(),
            Operator::Not => "Not".to_string(),
            Operator::Assign => "Assign".to_string(),
            Operator::GT => "GT".to_string(),
            Operator::LT => "LT".to_string(),
            Operator::GE => "GE".to_string(),
            Operator::LE => "LE".to_string(),
            Operator::EQ => "EQ".to_string(),
            Operator::NE => "NE".to_string(),
            Operator::BooleanAnd => "BooleanAnd".to_string(),
            Operator::BooleanOr => "BooleanOr".to_string(),
        }
    }
}

impl Keyword{
    pub fn dump(&self) -> String {
        match self {
            Keyword::If => "if".to_string(),
            Keyword::Else => "else".to_string(),
            Keyword::For => "for".to_string(),
            Keyword::While => "while".to_string(),
            Keyword::Loop => "loop".to_string(),
            Keyword::Fn => "fn".to_string(),
            Keyword::Class => "class".to_string(),
            Keyword::Let => "let".to_string(),
            Keyword::Return => "return".to_string(),
            Keyword::Break => "break".to_string(),
            Keyword::Continue => "continue".to_string(),
            Keyword::TheSelf => "self".to_string(),
        }
    }
}

impl Punctuation{
    pub fn dump(&self) -> String {
        match self {
            Punctuation::LeftCurlyBracket => "LeftCurlyBracket".to_string(),
            Punctuation::RightCurlyBracket => "RightCurlyBracket".to_string(),
            Punctuation::LeftRoundBracket => "LeftRoundBracket".to_string(),
            Punctuation::RightRoundBracket => "RightRoundBracket".to_string(),
            Punctuation::LeftSquareBracket => "LeftSquareBracket".to_string(),
            Punctuation::RightSquareBracket => "RightSquareBracket".to_string(),
            Punctuation::Dot => "Dot".to_string(),
            Punctuation::Comma => "Comma".to_string(),
            Punctuation::Semicolon => "Semicolon".to_string(),
            Punctuation::Colon => "Colon".to_string(),
        }
    }
}