use crate::{definite::types, defs};
use alloc::boxed::Box;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct BraceReferenceType {
    pub reference: Box<types::Types>,
    pub reference_pos: defs::Cursor,
    pub brace_pos: defs::Cursor,
    pub value: Box<types::Types>,
}
