use crate::parser::Collecting;
use crate::syntax::{types, variable};
use ellie_core::defs;
use serde::{Deserialize, Serialize};

use crate::alloc::boxed::Box;
use crate::alloc::string::String;
use crate::alloc::vec::Vec;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
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

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConditionChain {
    pub rtype: ConditionType,
    pub condition: Box<types::Types>,
    pub inside_code: Vec<Collecting>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConditionChainCollector {
    pub data: ConditionChain,
    pub keyword_pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Condition {
    pub chains: Vec<ConditionChain>,
    pub keyword_pos: defs::Cursor,
    pub cloak_pos: defs::Cursor, //Cloak position if [test] ......
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConditionCollector {
    pub data: Condition,
    pub cloak_itered_data: variable::VariableCollector,
    pub inside_code_string: String,
    pub might_be_else_if: bool,
    pub else_if_keyword_collector: String,
    pub initialized: bool,   
    pub inside_object_start: bool,
    pub inside_object_count: i64,
    pub cloak_collected: bool,
    pub complete: bool, //Fill this when end bracket placed
}
