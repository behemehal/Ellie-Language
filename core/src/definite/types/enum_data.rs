use crate::{definite::types, defs};
use alloc::{boxed::Box, string::String};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Pointer {
    NoData,
    Data(Box<types::Types>),
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct EnumData {
    pub reference: Box<types::Types>,
    pub reference_pos: defs::Cursor,
    pub brace_pos: defs::Cursor,
    pub value: Pointer,
    pub field_name: String,
    pub pos: defs::Cursor,
}
