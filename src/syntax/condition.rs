use crate::mapper::{defs, Collecting};
use crate::syntax::{types, variable};

#[derive(PartialEq, Debug, Clone)]
pub enum ConditionType {
    If,
    ElseIf,
    Else
}

impl Default for ConditionType {
    fn default() -> Self {
        ConditionType::If
    }
}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct ConditionChain {
    pub r#type: ConditionType,
    pub condition: types::cloak_type::CloakType,
    pub inside_code: Vec<Collecting>
}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct ConditionChainCollector {
    pub data: ConditionChain,
    pub keyword_pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct ConditionCollector {
    pub might_be_else_if: bool,
    pub else_if_keyword_collector: String,

    pub chains : Vec<ConditionChain>,
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