use crate::{definite::definers::DefinerCollecting, defs};
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ConstructorParameter {
    pub name: String,
    pub rtype: DefinerCollecting,
    pub pos: defs::Cursor,
    pub hash: usize,
}
