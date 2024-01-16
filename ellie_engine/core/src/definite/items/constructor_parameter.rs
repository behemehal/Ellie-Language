use crate::definite::definers::DefinerCollecting;
use crate::defs;
use alloc::string::String;
use serde::Deserialize;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ConstructorParameter {
    pub name: String,
    pub rtype: DefinerCollecting,
    pub pos: defs::Cursor,
    pub hash: usize,
}
