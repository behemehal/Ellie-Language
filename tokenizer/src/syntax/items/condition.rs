use ellie_core::definite::Converter;
use ellie_core::defs;
use serde::{Deserialize, Serialize};

use crate::processors::items;
use crate::processors::types;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum ConditionType {
    If,
    ElseIf,
    Else,
}

impl Default for ConditionType {
    fn default() -> ConditionType {
        ConditionType::If
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ConditionChain {
    pub rtype: ConditionType,
    pub keyword_captured: bool,
    pub condition: types::TypeProcessor,
    pub condition_filled: bool,
    pub code: Vec<items::Processors>,
    pub brace_count: usize,
    pub iterator: Box<crate::iterator::Iterator>,
    pub complete: bool,
    pub keyword_pos: defs::Cursor,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub chains: Vec<ConditionChain>,
    pub pos: defs::Cursor,
}

impl Converter<Condition, ellie_core::definite::items::condition::Condition> for Condition {
    fn to_definite(self) -> ellie_core::definite::items::condition::Condition {
        ellie_core::definite::items::condition::Condition {
            chains: self
                .chains
                .into_iter()
                .map(|x| ellie_core::definite::items::condition::ConditionChain {
                    rtype: match x.rtype {
                        ConditionType::If => {
                            ellie_core::definite::items::condition::ConditionType::If
                        }
                        ConditionType::ElseIf => {
                            ellie_core::definite::items::condition::ConditionType::ElseIf
                        }
                        ConditionType::Else => {
                            ellie_core::definite::items::condition::ConditionType::Else
                        }
                    },
                    condition: Box::new(x.condition.current.to_definite()),
                    code: x.code.into_iter().map(|x| x.to_definite()).collect(),
                    keyword_pos: x.keyword_pos,
                })
                .collect(),
            pos: self.pos,
        }
    }

    fn from_definite(self, from: ellie_core::definite::items::condition::Condition) -> Condition {
        todo!()
    }
}
