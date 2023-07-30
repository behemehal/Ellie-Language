use crate::alloc::vec::Vec;
use crate::definite::types;
use crate::defs;
use crate::{alloc::boxed::Box, definite::definers::DefinerCollecting};
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
    pub inner_page_id: usize,
    pub keyword_pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub hash: usize,
    pub chains: Vec<ConditionChain>,
    pub returns: Option<DefinerCollecting>,
    pub pos: defs::Cursor,
}
