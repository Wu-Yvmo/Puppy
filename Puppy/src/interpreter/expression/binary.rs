use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use crate::ast::Expression;
use crate::interpreter::Interpreter;
use crate::interpreter::value::{NeoDataModel, NeoObject};
use crate::token::{Operator, Punctuation, Token};

impl Interpreter {
    // 名称有冲突，解决一下
    pub fn interpret_expression_binary(&mut self, l: &Expression, op: &Token, r: &Expression) -> NeoObject {
        match op {
            Token::Operator(op) => match op {
                Operator::Add => self.interpret_expression_binary_add(l, r),
                Operator::Sub => self.interpret_expression_binary_sub(l, r),
                Operator::Mul => self.interpret_expression_binary_mul(l, r),
                Operator::Div => self.interpret_expression_binary_div(l, r),
                Operator::Rem => self.interpret_expression_binary_rem(l, r),
                Operator::Not => panic!("unexpected op"),
                Operator::Assign => self.interpret_expression_binary_assign(l, r),
                Operator::GT => self.interpret_expression_binary_gt(l, r),
                Operator::LT => self.interpret_expression_binary_lt(l, r),
                Operator::GE => self.interpret_expression_binary_ge(l, r),
                Operator::LE => self.interpret_expression_binary_le(l, r),
                Operator::EQ => self.interpret_expression_binary_eq(l, r),
                Operator::NE => self.interpret_expression_binary_ne(l, r),
                Operator::BooleanAnd => self.interpret_expression_binary_boolean_and(l, r),
                Operator::BooleanOr => self.interpret_expression_binary_boolean_or(l, r),
            }
            Token::Punctuation(punctuation) => match punctuation {
                Punctuation::Dot => {
                    self.interpret_expression(l);
                    let name = match r {
                        Expression::Use(expression_use) => {
                            expression_use.name.text()
                        }
                        _ => panic!("unexpected")
                    };
                    // 查找类型的方法
                    // 目前没有提供支持
                    todo!()
                }
                _ => panic!("unexpected punctuation"),
            }
            _ => panic!("unexpected token"),
        }
    }

    // 下面开始重构二元表达式的运算逻辑
    fn interpret_expression_binary_add(&mut self, l: &Expression, r: &Expression) -> NeoObject {
        let (l_object, r_object) = self.interpret_expression_binary_object_prepare(l, r);
        let l_neo_data_model = l_object.neo_data_model();
        let r_neo_data_model = r_object.neo_data_model();
        if l_neo_data_model.is_number() && r_neo_data_model.is_number() {
            NeoObject::create_number(l_neo_data_model.as_number() + r_neo_data_model.as_number())
        } else {
            panic!("unexpected")
        }
    }

    fn interpret_expression_binary_sub(&mut self, l: &Expression, r: &Expression) -> NeoObject {
        let (l_object, r_object) = self.interpret_expression_binary_object_prepare(l, r);
        let l_neo_data_model = l_object.neo_data_model();
        let r_neo_data_model = r_object.neo_data_model();
        if l_neo_data_model.is_number() && r_neo_data_model.is_number() {
            NeoObject::create_number(l_neo_data_model.as_number() - r_neo_data_model.as_number())
        } else {
            panic!("unexpected")
        }
    }
    
    fn interpret_expression_binary_mul(&mut self, l: &Expression, r: &Expression) -> NeoObject {
        let (l_object, r_object) = self.interpret_expression_binary_object_prepare(l, r);
        let l_neo_data_model = l_object.neo_data_model();
        let r_neo_data_model = r_object.neo_data_model();
        if l_neo_data_model.is_number() && r_neo_data_model.is_number() {
            NeoObject::create_number(l_neo_data_model.as_number() * r_neo_data_model.as_number())
        } else {
            panic!("unexpected")
        }
    }
    
    fn interpret_expression_binary_div(&mut self, l: &Expression, r: &Expression) -> NeoObject {
        let (l_object, r_object) = self.interpret_expression_binary_object_prepare(l, r);
        let l_neo_data_model = l_object.neo_data_model();
        let r_neo_data_model = r_object.neo_data_model();
        if l_neo_data_model.is_number() && r_neo_data_model.is_number() {
            NeoObject::create_number(l_neo_data_model.as_number() / r_neo_data_model.as_number())
        } else {
            panic!("unexpected")
        }
    }
    
    fn interpret_expression_binary_rem(&mut self, l: &Expression, r: &Expression) -> NeoObject {
        let (l_object, r_object) = self.interpret_expression_binary_object_prepare(l, r);
        let l_neo_data_model = l_object.neo_data_model();
        let r_neo_data_model = r_object.neo_data_model();
        if l_neo_data_model.is_number() && r_neo_data_model.is_number() {
            NeoObject::create_number(((*l_neo_data_model.as_number() as i64) % (*r_neo_data_model.as_number() as i64)) as f64)
        } else {
            panic!("unexpected")
        }
    }

    fn interpret_expression_binary_assign(&mut self, l: &Expression, r: &Expression) -> NeoObject {
        let (mut l_object, r_object) = self.interpret_expression_binary_object_prepare(l, r);
        l_object.set_handle(r_object.get_handle());
        r_object
    }
    
    fn interpret_expression_binary_gt(&mut self, l: &Expression, r: &Expression) -> NeoObject {
        let (l_object, r_object) = self.interpret_expression_binary_object_prepare(l, r);
        let l_neo_data_model = l_object.neo_data_model();
        let r_neo_data_model = r_object.neo_data_model();
        if l_neo_data_model.is_number() && r_neo_data_model.is_number() {
            NeoObject::create_boolean(l_neo_data_model.as_number() > r_neo_data_model.as_number())
        } else {
            panic!("unexpected")
        }
    }
    
    fn interpret_expression_binary_lt(&mut self, l: &Expression, r: &Expression) -> NeoObject {
        let (l_object, r_object) = self.interpret_expression_binary_object_prepare(l, r);
        let l_neo_data_model = l_object.neo_data_model();
        let r_neo_data_model = r_object.neo_data_model();
        if l_neo_data_model.is_number() && r_neo_data_model.is_number() {
            NeoObject::create_boolean(l_neo_data_model.as_number() < r_neo_data_model.as_number())
        } else {
            panic!("unexpected")
        }
    }

    fn interpret_expression_binary_ge(&mut self, l: &Expression, r: &Expression) -> NeoObject {
        let (l_object, r_object) = self.interpret_expression_binary_object_prepare(l, r);
        let l_neo_data_model = l_object.neo_data_model();
        let r_neo_data_model = r_object.neo_data_model();
        if l_neo_data_model.is_number() && r_neo_data_model.is_number() {
            NeoObject::create_boolean(l_neo_data_model.as_number() >= r_neo_data_model.as_number())
        } else {
            panic!("unexpected")
        }
    }
    
    fn interpret_expression_binary_le(&mut self, l: &Expression, r: &Expression) -> NeoObject {
        let (l_object, r_object) = self.interpret_expression_binary_object_prepare(l, r);
        let l_neo_data_model = l_object.neo_data_model();
        let r_neo_data_model = r_object.neo_data_model();
        if l_neo_data_model.is_number() && r_neo_data_model.is_number() {
            NeoObject::create_boolean(l_neo_data_model.as_number() <= r_neo_data_model.as_number())
        } else {
            panic!("unexpected")
        }
    }
    
    fn interpret_expression_binary_eq(&mut self, l: &Expression, r: &Expression) -> NeoObject {
        let (l_object, r_object) = self.interpret_expression_binary_object_prepare(l, r);
        let l_neo_data_model = l_object.neo_data_model();
        let r_neo_data_model = r_object.neo_data_model();
        if l_neo_data_model.is_number() && r_neo_data_model.is_number() {
            NeoObject::create_boolean(l_neo_data_model.as_number() == r_neo_data_model.as_number())
        } else if l_neo_data_model.is_boolean() && r_neo_data_model.is_boolean() {
            NeoObject::create_boolean(l_neo_data_model.as_boolean() == r_neo_data_model.as_boolean())
        } else if l_neo_data_model.is_string() && r_neo_data_model.is_string() {
            NeoObject::create_boolean(l_neo_data_model.as_string() == r_neo_data_model.as_string())
        } else {
            panic!("unexpected")
        }
    }
    
    fn interpret_expression_binary_ne(&mut self, l: &Expression, r: &Expression) -> NeoObject {
        let (l_object, r_object) = self.interpret_expression_binary_object_prepare(l, r);
        let l_neo_data_model = l_object.neo_data_model();
        let r_neo_data_model = r_object.neo_data_model();
        if l_neo_data_model.is_number() && r_neo_data_model.is_number() {
            NeoObject::create_boolean(l_neo_data_model.as_number() != r_neo_data_model.as_number())
        } else if l_neo_data_model.is_boolean() && r_neo_data_model.is_boolean() {
            NeoObject::create_boolean(l_neo_data_model.as_boolean()!= r_neo_data_model.as_boolean())
        } else if l_neo_data_model.is_string() && r_neo_data_model.is_string() {
            NeoObject::create_boolean(l_neo_data_model.as_string()!= r_neo_data_model.as_string())
        } else {
            panic!("unexpected")
        }
    }
    
    fn interpret_expression_binary_boolean_and(&mut self, l: &Expression, r: &Expression) -> NeoObject {
        let (l_object, r_object) = self.interpret_expression_binary_object_prepare(l, r);
        let l_neo_data_model = l_object.neo_data_model();
        let r_neo_data_model = r_object.neo_data_model();
        if l_neo_data_model.is_boolean() && r_neo_data_model.is_boolean() {
            NeoObject::create_boolean(*l_neo_data_model.as_boolean() && *r_neo_data_model.as_boolean())
        } else {
            panic!("unexpected")
        }
    }
    
    fn interpret_expression_binary_boolean_or(&mut self, l: &Expression, r: &Expression) -> NeoObject {
        let (l_object, r_object) = self.interpret_expression_binary_object_prepare(l, r);
        let l_neo_data_model = l_object.neo_data_model();
        let r_neo_data_model = r_object.neo_data_model();
        if l_neo_data_model.is_boolean() && r_neo_data_model.is_boolean() {
            NeoObject::create_boolean(*l_neo_data_model.as_boolean() || *r_neo_data_model.as_boolean())
        } else {
            panic!("unexpected")
        }
    }
    
    fn interpret_expression_binary_object_prepare(&mut self, l: &Expression, r: &Expression) -> (NeoObject, NeoObject) {
        (self.interpret_expression(l), self.interpret_expression(r))
    }
}