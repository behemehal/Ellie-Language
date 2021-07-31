use crate::syntax::{definers, types};
use alloc::string::String;
use ellie_core::defs;
use serde::Deserialize;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Variable {
    pub name: String,
    pub dynamic: bool,
    pub constant: bool,
    pub public: bool,
    pub value: types::Types,
    pub pos: defs::Cursor,
    pub name_pos: defs::Cursor,
    pub value_pos: defs::Cursor,
    pub type_pos: defs::Cursor,
    pub rtype: definers::DefinerCollecting,
    pub hash: String,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct VariableCollector {
    pub initialized: bool,
    pub named: bool,
    pub typed: bool,
    pub value_complete: bool,
    pub raw_value: String,
    pub data: Variable,
}
