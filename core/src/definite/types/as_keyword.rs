use crate::{
    definite::{definers, types},
    defs,
};
use alloc::boxed::Box;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct AsKeyword {
    pub target: Box<types::Types>,
    pub pos: defs::Cursor,
    pub target_pos: defs::Cursor,
    pub type_pos: defs::Cursor,
    pub rtype: definers::DefinerCollecting,
}
