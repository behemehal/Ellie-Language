use crate::alloc::boxed::Box;
use crate::alloc::vec::Vec;
use crate::definite::items::Collecting;
use crate::definite::types;
use crate::defs;


#[repr(C)]
pub enum ConditionType {
    If,
    ElseIf,
    Else,
}

#[repr(C)]
pub struct ConditionChain {
    pub rtype: ConditionType,
    pub condition: Box<types::Types>,
    pub code: Vec<Collecting>,
    pub pos: defs::Cursor,
}

#[repr(C)]
pub struct Condition {
    pub chains: Vec<ConditionChain>,
    pub keyword_pos: defs::Cursor,
    pub cloak_pos: defs::Cursor,
}
