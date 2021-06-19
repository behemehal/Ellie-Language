use alloc::vec::Vec;
use alloc::string::String;
use crate::syntax::{types, variable, function};
use ellie_core::defs;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct ExtendClass {
    position: defs::Cursor,
    name: String,
}

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct Class {
    pub name: String,
    pub constructor: function::Function,
    pub extends: Vec<ClassCollector>,
    pub properties: Vec<variable::VariableCollector>,
    pub getters: Vec<types::arrow_function::ArrowFunctionCollector>,
    pub setters: Vec<types::arrow_function::ArrowFunctionCollector>,
    pub methods: Vec<function::FunctionCollector>
}

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct ClassCollector {
    pub name_pos: defs::Cursor,
    pub name_collected: bool,
    pub extends_collected: bool,
    pub extend_collect: String,
    pub at_comma: bool,
    pub data: Class
}
