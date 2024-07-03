use crate::{definite::definers, defs};
use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

use super::file_key::FileKey;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]

pub struct Setter {
    pub name: String,
    pub pos: defs::Cursor,
    pub public: bool,
    pub name_pos: defs::Cursor,
    pub rtype_pos: defs::Cursor,
    pub param_name_pos: defs::Cursor,
    pub body_pos: defs::Cursor,
    pub inner_page_id: usize,
    pub hash: usize,
    pub param_name: String,
    pub parameters_pos: defs::Cursor,
    pub file_keys: Vec<FileKey>,
    pub rtype: definers::DefinerCollecting,
}
