use crate::syntax::types;
use alloc::boxed::Box;
use alloc::string::String;
use ellie_core::defs;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct NewCall {
    pub value: Box<types::Types>,
    pub keyword_pos: defs::Cursor,
    pub value_pos: defs::Cursor,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct NewCallCollector {
    pub data: NewCall,
    pub keyword_collected: bool,
    pub keyword_index: i8,
    pub raw_value: String,
    pub value_collected: bool,
    pub complete: bool,
}
