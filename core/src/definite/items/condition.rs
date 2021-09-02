use crate::alloc::boxed::Box;
use crate::alloc::string::String;
use crate::alloc::vec::Vec;
use crate::definite::items::Collecting;
use crate::definite::{items::variable, types};
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
    pub inside_code: Vec<Collecting>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub chains: Vec<ConditionChain>,
    pub keyword_pos: defs::Cursor,
    pub cloak_pos: defs::Cursor,
}
