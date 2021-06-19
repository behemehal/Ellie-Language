use crate::syntax::definers;
use ellie_core::defs;
use serde::Serialize;

use alloc::string::String;

#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct Variable {
    pub name: String,
    pub dynamic: bool,
    pub public: bool,
    pub value: crate::syntax::types::Types,
    pub pos: defs::Cursor,
    pub value_pos: defs::Cursor,
    pub type_pos: defs::Cursor,
    pub rtype: definers::DefinerCollecting,
}

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct VariableCollector {
    pub initialized: bool,
    pub named: bool,
    pub typed: bool,
    pub value_complete: bool,
    pub raw_value: String,
    pub data: Variable,
}
