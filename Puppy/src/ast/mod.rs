//! 提供抽象语法树

// 模块声明
mod program;
mod statement_item;
mod r#if;
mod r#while;
mod r#for;
mod r#loop;
mod expression;
mod r#type;
mod r#break;
mod r#continue;
mod r#return;
mod class;
mod block;
mod r#let;

// 定义导出
pub use program::*;
pub use statement_item::*;
pub use r#if::*;
pub use r#while::*;
pub use r#for::*;
pub use r#loop::*;
pub use expression::*;
pub use r#type::*;
pub use r#break::*;
pub use r#continue::*;
pub use r#return::*;
pub use class::*;
pub use block::*;
pub use r#let::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct T{
    name: String,
    age: i32,
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn json() {
        let t = T{
            name: "Hell".to_string(),
            age: 34,
        };
        let json = serde_json::to_string_pretty(&t).unwrap();
        println!("{}", json)
    }
}