use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Type {
    pub name: Token,
    pub optional_template_describe: Option<TemplateDescribe>,
}
impl Type {
    pub fn create(name: Token, optional_template_describe: Option<TemplateDescribe>) -> Self {
        Self{
            name,
            optional_template_describe
        }
    }
}

#[derive(Debug, Clone)]
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