use crate::parser::Collecting;
use crate::syntax::{types, variable};
use ellie_core::{definite, defs};
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

impl ConditionCollector {
    pub fn to_definite(self) -> definite::items::condition::Condition {
        definite::items::condition::Condition {
            chains: self
                .data
                .chains
                .into_iter()
                .map(|x| definite::items::condition::ConditionChain {
                    rtype: match x.rtype {
                        ConditionType::If => definite::items::condition::ConditionType::If,
                        ConditionType::ElseIf => definite::items::condition::ConditionType::ElseIf,
                        ConditionType::Else => definite::items::condition::ConditionType::Else,
                    },
                    condition: Box::new(x.condition.to_definite()),
                    inside_code: x.inside_code.into_iter().map(|x| x.to_definite()).collect(),
                })
                .collect(),
            keyword_pos: self.data.keyword_pos,
            cloak_pos: self.data.cloak_pos,
        }
    }
}
