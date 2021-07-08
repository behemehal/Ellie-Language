use crate::parser::Collecting;
use crate::syntax::{types, variable};
use ellie_core::defs;
use serde::Serialize;

use crate::alloc::string::String;
use crate::alloc::vec::Vec;

#[derive(PartialEq, Debug, Clone, Serialize, Hash)]
pub enum ConditionType {
    If,
    ElseIf,
    Else,
}

impl Default for ConditionType {
    fn default() -> Self {
        ConditionType::If
    }
}

#[derive(PartialEq, Debug, Clone, Default, Serialize, Hash)]
pub struct ConditionChain {
    pub rtype: ConditionType,
    pub condition: types::cloak_type::CloakType,
    pub inside_code: Vec<Collecting>,
}

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct ConditionChainCollector {
    pub data: ConditionChain,
    pub keyword_pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Default, Serialize, Hash)]
pub struct ConditionCollector {
    pub might_be_else_if: bool,
    pub else_if_keyword_collector: String,

    pub chains: Vec<ConditionChain>,
    pub keyword_pos: defs::Cursor,
    pub initialized: bool,

    pub inside_code_string: String,
    pub inside_object_start: bool,
    pub inside_object_count: i64,

    pub cloak_collected: bool,
    pub cloak_pos: defs::Cursor, //Cloak position if [test] ......
    pub cloak_itered_data: variable::VariableCollector,

    pub complete: bool, //Fill this when end bracket placed
}
