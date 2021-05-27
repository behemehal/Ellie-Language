use crate::syntax::{definers, types};
use crate::defs;
use libc::c_char;

#[repr(C)]
pub struct Variable {
    pub name: *const c_char,
    pub dynamic: bool,
    pub public: bool,
    pub value: types::Types,
    pub pos: defs::Cursor,
}

#[repr(C)]
pub struct VariableCollector {
    pub initialized: bool,
    pub named: bool,
    pub typed: bool,
    pub value_complete: bool,
    pub r#type: definers::DefinerCollecting,
    pub raw_value: *const c_char,
    pub data: Variable,
}
