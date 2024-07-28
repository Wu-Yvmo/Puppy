use crate::token::Token;

pub enum Type {
    TheSelf,
    Normal(Normal),
}
impl Type {
    pub fn create_self() -> Self {
        Self::TheSelf
    }
    pub fn create_normal(name: Token, optional_template_describe: Option<TemplateDescribe>) -> Self {
        Self::Normal(Normal{
            name,
            optional_template_describe
        })
    }
}

pub struct Normal{
    pub name: Token,
    pub optional_template_describe:Option<TemplateDescribe>,
}

impl Normal{
    pub fn create(name: Token, optional_template_describe: Option<TemplateDescribe>) -> Self {
        Self{
            name,
            optional_template_describe
        }
    }
}



pub struct TemplateDescribe {
    pub items: Vec<Type>,
}

impl TemplateDescribe {
    pub fn create_template_describe(items: Vec<Type>) -> Self {
        Self {
            items
        }
    }
}