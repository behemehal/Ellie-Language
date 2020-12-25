use crate::alloc::string::{String, ToString};
use crate::alloc::vec::Vec;

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    pub target_type: String,
    pub value: String,
    pub is_variable: bool,
    pub is_string: bool,
    pub is_number: bool,
    pub is_bool: bool,
    pub is_function: bool
}


#[derive(Debug, Clone , PartialEq, Eq)]
pub enum Code {
    Compiled(crate::collectors::Compiled),
    Null(bool)
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub dynamic: bool,
    pub parameters: Vec<crate::collectors::function::Parameter>,
    pub type_return: String,
    pub code: Code
}