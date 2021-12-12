use crate::alloc::boxed::Box;
use crate::alloc::vec::Vec;
use crate::definite::items::Collecting;
use crate::definite::types;
use crate::defs;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum ConditionType {
    If,
    ElseIf,
    Else,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ConditionChain {
    pub rtype: ConditionType,
    pub condition: Box<types::Types>,
    pub code: Vec<Collecting>,
    pub keyword_pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub chains: Vec<ConditionChain>,
    pub pos: defs::Cursor,
}
