use crate::definite::items::{constructor, function, getter, setter, variable};
use crate::defs;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

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
    pub getters: Vec<getter::Getter>,
    pub setters: Vec<setter::Setter>,
    pub methods: Vec<function::Function>,
    pub name_pos: defs::Cursor,
    pub pos: defs::Cursor,
}
