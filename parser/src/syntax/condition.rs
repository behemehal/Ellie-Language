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
    pub code: Vec<Collecting>,
    pub pos: defs::Cursor,
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
    pub brace_count: usize,
    pub might_be_else_if: bool,
    pub else_if_keyword_collector: String,
    pub initialized: bool,
    pub inside_object_start: bool,
    pub inside_object_count: i64,
    pub cloak_collected: bool,
    pub complete: bool, //Fill this when end bracket placed
    pub code: Box<crate::parser::RawParser>,
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
                    code: x.code.into_iter().map(|x| x.to_definite()).collect(),
                    pos: x.pos,
                })
                .collect(),
            keyword_pos: self.data.keyword_pos,
            cloak_pos: self.data.cloak_pos,
        }
    }

    pub fn from_definite(self, from: definite::items::condition::Condition) -> Self {
        ConditionCollector {
            data: Condition {
                chains: from
                    .chains
                    .into_iter()
                    .map(|x| ConditionChain {
                        rtype: match x.rtype {
                            definite::items::condition::ConditionType::If => ConditionType::If,
                            definite::items::condition::ConditionType::ElseIf => {
                                ConditionType::ElseIf
                            }
                            definite::items::condition::ConditionType::Else => ConditionType::Else,
                        },
                        condition: Box::new(types::Types::default().from_definite(*x.condition)),
                        code: x
                            .code
                            .into_iter()
                            .map(|e| Collecting::default().from_definite(e))
                            .collect(),
                        pos: x.pos,
                    })
                    .collect(),
                keyword_pos: from.keyword_pos,
                cloak_pos: from.cloak_pos,
            },
            complete: true,
            ..Default::default()
        }
    }
}
