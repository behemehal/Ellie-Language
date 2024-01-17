use crate::{definite::definers, defs};
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct FunctionParameter {
    pub name: String,
    pub rtype: definers::DefinerCollecting,
    pub name_pos: defs::Cursor,
    pub rtype_pos: defs::Cursor,
    pub hash: usize,
}
