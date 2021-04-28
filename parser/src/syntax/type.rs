use serde::Serialize;
use alloc::string::String;
use alloc::boxed::Box;

#[derive(PartialEq, Debug, Clone, Serialize)]
pub enum Child {
    Some(DirectType),
    None
}

impl Default for Child {
    fn default() -> Self {
        Child::None
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct DirectType {
    pub name: String,
    pub child: Box<Option<Child>>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct TypeConstructorCollector {
    pub data: DirectType,
    pub collecting: String,
    pub child_started: bool,
}