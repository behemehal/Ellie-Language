use crate::{definite::types, defs};
use alloc::boxed::Box;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Negative {
    pub value: Box<types::Types>,
    pub pos: defs::Cursor,
}
