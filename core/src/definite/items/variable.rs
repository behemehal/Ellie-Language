use crate::definite::{definers, types};
use crate::defs;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub constant: bool,
    pub public: bool,
    pub has_type: bool,
    pub has_value: bool,
    pub value: types::Types,
    pub pos: defs::Cursor,
    pub name_pos: defs::Cursor,
    pub value_pos: defs::Cursor,
    pub type_pos: defs::Cursor,
    pub rtype: definers::DefinerCollecting,
    pub hash: String,
}
