use serde_json::json;

pub trait ContextHeader {
    fn get_header_value(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct NoContext;

#[derive(Debug, Clone)]
pub struct NewGroupDmContext;

impl ContextHeader for NoContext {
    fn get_header_value(&self) -> String {
        json!({}).to_string()
    }
}

impl ContextHeader for NewGroupDmContext {
    fn get_header_value(&self) -> String {
        json!({"location":"New Group DM"}).to_string()
    }
}

#[derive(Debug, Clone)]
pub enum Context {
    NoContext,
    NewGroupDmContext,
}

impl ContextHeader for Context {
    fn get_header_value(&self) -> String {
        match self {
            Context::NoContext => NoContext.get_header_value(),
            Context::NewGroupDmContext => NewGroupDmContext.get_header_value(),
        }
    }
}
