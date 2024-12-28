use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use crate::ast;
use crate::ast::{Class, ClassMethod};


// 当赋值发生在基础类型中时？
// 什么会发生？
// let a = 1.0
// a = 2.0

// 发生：在环境变量中对a进行查询
// 得到a
// 把a的值设置成为2.0，还是把a重新注册为2.0这个新变量？
// 我更倾向于把a注册成2.0这个新变量

// a.name = "string"
// 这个过程怎么发生？我倾向于把a.name设置成新的"string"对象
// member_access

// 对于NeoObject的设计，是为了实现这样一个功能：
// 所有的对象都是以引用的形式持有的
#[derive(Clone)]
pub struct NeoObject {
    pub handle: Rc<RefCell<NeoDataModel>>
}

/// 实现基本的创建
impl NeoObject {
    pub fn create_break() -> Self {
        Self{
            handle: Rc::new(RefCell::new(NeoDataModel::create_break()))
        }
    }

    pub fn create_continue() -> Self {
        Self{
            handle: Rc::new(RefCell::new(NeoDataModel::create_continue()))
        }
    }

    pub fn create_nil() -> Self {
        Self{
            handle: Rc::new(RefCell::new(NeoDataModel::create_nil()))
        }
    }

    pub fn create_number(value: f64) -> Self {
        Self{
            handle: Rc::new(RefCell::new(NeoDataModel::create_number(value)))
        }
    }

    pub fn create_string(value: String) -> Self {
        Self{
            handle: Rc::new(RefCell::new(NeoDataModel::create_string(value)))
        }
    }

    pub fn create_boolean(value: bool) -> Self {
        Self{
            handle: Rc::new(RefCell::new(NeoDataModel::create_boolean(value)))
        }
    }

    pub fn create_user(prototype: String, data: HashMap<String, NeoObject>) -> Self {
        Self{
            handle: Rc::new(RefCell::new(NeoDataModel::create_user(prototype, data)))
        }
    }

    pub fn create_method(user: NeoObject, name: String, parameters: Vec<NeoObject>) -> Self {
        Self{
            handle: Rc::new(RefCell::new(NeoDataModel::create_method(user, name, parameters)))
        }
    }
}

/// 把NeoObject展开为f64，即number
/// 是不是现实的，把NeoObject展开为f64？这里涉及一个问题，那就是NeoDataModel的类型并不确定
/// 真的是这样吗？
/// borrow返回的是一个新建出来的玩意，所以self的生命周期没有来到Ref里，这就是我们获得不了&f64的原因
impl NeoObject {
    pub fn get_handle(&self) -> Rc<RefCell<NeoDataModel>> {
        self.handle.clone()
    }

    pub fn set_handle(&mut self, new_handle: Rc<RefCell<NeoDataModel>>) {
        self.handle = new_handle
    }

    pub fn neo_data_model(&self) -> Ref<'_, NeoDataModel> {
        self.handle.borrow()
    }

    pub fn neo_data_model_mut(&self) -> RefMut<'_, NeoDataModel>{
        self.handle.borrow_mut()
    }
}

#[derive(Clone)]
pub enum NeoDataModel{
    Break,
    Continue,
    Return(Option<NeoObject>),
    Nil,
    Number(NeoNumber),
    String(NeoString),
    Boolean(NeoBoolean),
    User(NeoUser),
    Method(NeoMethod),
}

impl NeoDataModel{
    pub fn create_break() -> Self {
        Self::Break
    }

    pub fn create_continue() -> Self {
        Self::Continue
    }

    pub fn create_return(return_value: Option<NeoObject>) -> Self {
        Self::Return(return_value)
    }

    pub fn create_nil() -> Self {
        Self::Nil
    }

    pub fn create_number(value: f64) -> Self {
        Self::Number(NeoNumber{
            value
        })
    }

    pub fn create_string(value: String) -> Self {
        Self::String(NeoString{
            value
        })
    }

    pub fn create_boolean(value: bool) -> Self {
        Self::Boolean(NeoBoolean{
            value
        })
    }

    pub fn create_user(prototype: String, data: HashMap<String, NeoObject>) -> Self {
        Self::User(NeoUser{
            prototype,
            data
        })
    }

    pub fn create_method(user: NeoObject, name: String, parameters: Vec<NeoObject>) -> Self {
        Self::Method(NeoMethod{
            user,
            name,
            parameters,
        })
    }

    pub fn as_return(&self) -> &Option<NeoObject> {
        match self {
            NeoDataModel::Return(return_value) => return_value,
            _ => panic!("NeoObject is not a return")
        }
    }

    pub fn as_number(&self) -> &f64 {
        match self {
            NeoDataModel::Number(number) => &number.value,
            _ => panic!("NeoObject is not a number")
        }
    }

    pub fn as_boolean(&self) -> &bool {
        match self {
            NeoDataModel::Boolean(boolean) => &boolean.value,
            _ => panic!("NeoObject is not a boolean")
        }
    }

    pub fn as_string(&self) -> &String {
        match self {
            NeoDataModel::String(string) => &string.value,
            _ => panic!("NeoObject is not a string")
        }
    }

    pub fn as_number_mut(&mut self) -> &mut f64 {
        match self {
            NeoDataModel::Number(number) => &mut number.value,
            _ => panic!("NeoObject is not a number")
        }
    }

    pub fn as_boolean_mut(&mut self) -> &mut bool {
        match self {
            NeoDataModel::Boolean(boolean) => &mut boolean.value,
            _ => panic!("NeoObject is not a boolean")
        }
    }

    pub fn as_string_mut(&mut self) -> &mut String {
        match self {
            NeoDataModel::String(string) => &mut string.value,
            _ => panic!("NeoObject is not a string")
        }
    }

    pub fn is_break(&self) -> bool {
        if let NeoDataModel::Break = self {
            return true
        }
        false
    }

    pub fn is_continue(&self) -> bool {
        if let NeoDataModel::Continue = self {
            return true
        }
        false
    }

    pub fn is_return(&self) -> bool {
        if let NeoDataModel::Return(_) = self {
            return true
        }
        false
    }

    pub fn is_nil(&self) -> bool {
        if let NeoDataModel::Nil = self {
            return true
        }
        false
    }

    pub fn is_number(&self) -> bool {
        if let NeoDataModel::Number(_) = self {
            return true
        }
        false
    }

    pub fn is_boolean(&self) -> bool {
        if let NeoDataModel::Boolean(_) = self {
            return true
        }
        false
    }

    pub fn is_string(&self) -> bool {
        if let NeoDataModel::String(_) = self {
            return true
        }
        false
    }
}

#[derive(Clone)]
pub struct NeoNumber{
    pub value: f64
}

#[derive(Clone)]
pub struct NeoString{
    pub value: String
}

#[derive(Clone)]
pub struct NeoBoolean{
    pub value: bool
}

#[derive(Clone)]
pub struct NeoUser{
    // 原型名称
    pub prototype: String,
    // 数据成员
    pub data: HashMap<String, NeoObject>
}

#[derive(Clone)]
pub struct NeoMethod{
    /// 方法使用者
    pub user: NeoObject,
    /// 方法名
    pub name: String,
    /// 参数列表
    pub parameters: Vec<NeoObject>,
}

// 理论上讲，Method的执行需要这些内容：
// 使用者
// 参数
// 方法的内容

// 应该如何执行一个方法？
// 1.找到方法的定义
// 2.计算方法的parameters
// 3.执行方法

// 暂时不准备提供对[] () {}的支持