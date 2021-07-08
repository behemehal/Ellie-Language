use crate::syntax::{definers, types};
use ellie_core::defs;
use serde::Serialize;

use alloc::string::String;
use core::hash::Hash;

#[derive(PartialEq, Debug, Clone, Serialize, Default, Hash)]
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

#[derive(PartialEq, Debug, Clone, Default, Serialize, Hash)]
pub struct VariableCollector {
    pub initialized: bool,
    pub named: bool,
    pub typed: bool,
    pub value_complete: bool,
    pub raw_value: String,
    pub data: Variable,
}
