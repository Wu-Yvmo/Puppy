use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use crate::ast;
use crate::ast::{ClassItem, Program};
use crate::interpreter::value::NeoObject;

pub struct ExecuteEnvironment {
    value_manager: ValueManager,
    // method_manager: MethodManager,
}

impl ExecuteEnvironment{
    pub fn create(program: &Program) -> Self {
        Self{
            value_manager: ValueManager::create(),
        }
    }

    /// # 转发调用ValueManager中的push_frame方法
    pub fn push_frame(&mut self) {
        self.value_manager.push_frame();
    }

    /// 转发调用ValueManager中的pop_frame方法
    /// # 功能
    /// 弹出栈帧
    pub fn pop_frame(&mut self) {
        self.value_manager.pop_frame();
    }

    /// 转发调用ValueManager中的register方法
    /// # 功能
    /// 注册变量
    pub fn register(&mut self, name: String, object: NeoObject) {
        self.value_manager.register(name, object)
    }

    /// 转发调用ValueManager中的query方法
    /// # 功能
    /// 查询变量
    pub fn query(&mut self, name: String) -> NeoObject {
        self.value_manager.query(name)
    }
}

pub struct ValueManager {
    frames: Vec<HashMap<String, NeoObject>>,
}

impl ValueManager {
    pub fn create() -> Self {
        Self{
            frames: vec![]
        }
    }

    pub fn push_frame(&mut self) {
        self.frames.push(HashMap::new())
    }

    pub fn pop_frame(&mut self) {
        self.frames.pop();
    }

    pub fn register(&mut self, name: String, object: NeoObject) {
        self.frames.last_mut().unwrap().insert(name, object.clone());
    }

    pub fn query(&mut self, name: String) -> NeoObject {
        for frame in self.frames.iter().rev() {
            if let Some(value) = frame.get(&name) {
                return value.clone()
            }
        }
        panic!("no value: {}", name)
    }
}

pub struct PrototypeManager {
    prototypes: HashMap<String, Rc<ast::Class>>
}

impl PrototypeManager {
    pub fn create(program: &Program) -> Self {
        let mut prototypes: HashMap<String, Rc<ast::Class>> = HashMap::new();
        for program_item in &program.items {
            if let ast::Statement::Class(class) = program_item {
                prototypes.insert(class.name.text(), Rc::new(class.clone()));
            }
        }
        Self{
            prototypes
        }
    }
}

pub struct MethodManager{
    methods: HashMap<String, HashMap<String, Rc<ast::ClassMethod>>>
}

impl MethodManager {
    pub fn create(program: &Program) -> Self {
        let mut methods: HashMap<String, HashMap<String, Rc<ast::ClassMethod>>> = HashMap::new();
        for program_item in &program.items {
            if let ast::Statement::Class(class) = program_item {
                methods.insert(class.name.text(), HashMap::new());
                for class_item in &class.items {
                    if let ClassItem::Method(class_item_method) = class_item {
                        methods.get_mut(&class.name.text()).unwrap().insert(class_item_method.name.text(), Rc::new(class_item_method.clone()));
                    }
                }
            }
        }
        Self{
            methods
        }
    }
}

// 一个方法的执行过程：
// let a = hello{}
// 按字段构造hello，注册a
// 类型系统有用吗？
// 一个Object 应该保存的信息有：
// 原型
// 数据字段

// a.name -> Method
// 保存： a "name"方法的引用
// a.name(...) 的执行流程：
// 建立新的栈帧 self 参数压栈 解释方法 收集返回值