use crate::{definite::types, defs};
use alloc::boxed::Box;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct NullResolver {
    pub target: Box<types::Types>,
    pub target_pos: defs::Cursor,
    pub pos: defs::Cursor,
}
