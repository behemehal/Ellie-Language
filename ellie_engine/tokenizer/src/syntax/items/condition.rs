use crate::processors::items;
use crate::processors::types::{self, TypeProcessor};
use ellie_core::definite::Converter;
use ellie_core::defs;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
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
    #[serde(skip)]
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
        unreachable!();
    }

    fn from_definite(self, from: ellie_core::definite::items::condition::Condition) -> Condition {
        Condition {
            chains: from
                .chains
                .into_iter()
                .map(|x| ConditionChain {
                    rtype: match x.rtype {
                        ellie_core::definite::items::condition::ConditionType::If => {
                            ConditionType::If
                        }
                        ellie_core::definite::items::condition::ConditionType::ElseIf => {
                            ConditionType::ElseIf
                        }
                        ellie_core::definite::items::condition::ConditionType::Else => {
                            ConditionType::Else
                        }
                    },
                    condition: TypeProcessor {
                        current: types::Processors::default().from_definite(*x.condition),
                        ignore: false,
                    },
                    code: vec![],
                    keyword_pos: x.keyword_pos,
                    ..Default::default()
                })
                .collect(),
            pos: self.pos,
        }
    }
}
