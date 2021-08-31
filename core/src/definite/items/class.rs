use crate::definite::items::{constructor, function, variable};
use alloc::string::String;
use alloc::vec::Vec;
use crate::defs;
use serde::{Deserialize, Serialize};
use std::boxed::Box;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct GenericDefining {
    pub name: String,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub name: String,
    pub public: bool,
    pub constructor: constructor::Constructor,
    pub generic_definings: Vec<GenericDefining>,
    pub properties: Vec<variable::Variable>,
    pub methods: Vec<function::Function>,
    pub name_pos: defs::Cursor,
    pub pos: defs::Cursor,
}