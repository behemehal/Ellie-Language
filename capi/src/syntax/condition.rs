use crate::defs;
use crate::parser::Collecting;
use crate::syntax::{types, variable};
use libc::c_char;

#[repr(C)]
pub enum ConditionType {
    If,
    ElseIf,
    Else,
}

#[repr(C)]
pub struct ConditionChain {
    pub rtype: ConditionType,
    pub condition: types::cloak_type::CloakType,
    pub inside_code: *const Collecting,
}

#[repr(C)]
pub struct ConditionChainCollector {
    pub data: ConditionChain,
    pub keyword_pos: defs::Cursor,
}

#[repr(C)]
pub struct ConditionCollector {
    pub might_be_else_if: bool,
    pub else_if_keyword_collector: *const c_char,
    pub chains: *const ConditionChain,
    pub keyword_pos: defs::Cursor,
    pub initialized: bool,
    pub inside_code_string: *const c_char,
    pub inside_object_start: bool,
    pub inside_object_count: i64,
    pub cloak_collected: bool,
    pub cloak_pos: defs::Cursor,
    pub cloak_itered_data: variable::VariableCollector,
    pub complete: bool,
}
