use crate::definite::items::Collecting;
use crate::defs;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ConstructorParameter {
    pub name: String,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Constructor {
    pub parameters: Vec<ConstructorParameter>,
    pub inner_page_id: u64,
    pub inside_code: Vec<Collecting>,
    pub name_pos: defs::Cursor,
    pub parameters_pos: defs::Cursor,
    pub pos: defs::Cursor,
}
