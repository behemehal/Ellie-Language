use std::boxed::Box;
use crate::syntax::{constructor, function, variable};
use alloc::string::String;
use alloc::vec::Vec;
use ellie_core::defs;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct GenericDefining {
    pub name: String,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct Class {
    pub name: String,
    pub constructor: constructor::Constructor,
    pub generic_definings: Vec<GenericDefining>,
    pub properties: Vec<variable::Variable>,
    //pub getters: Vec<types::arrow_function::ArrowFunctionCollector>,
    //pub setters: Vec<types::arrow_function::ArrowFunctionCollector>,
    pub methods: Vec<function::Function>,
    pub name_pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct ClassCollector {
    pub generic_definings_collected: bool,
    pub brace_count: usize,
    pub name_collected: bool,
    pub inside_code_string: String,
    pub generic_brace_open: bool,
    pub has_code: bool,
    pub at_comma: bool,
    pub data: Class,
    pub code: Box<crate::parser::Parser>,
}
