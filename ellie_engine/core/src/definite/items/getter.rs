use crate::{definite::definers, defs};
use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

use super::file_key::FileKey;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]

pub struct Getter {
    pub name: String,
    pub name_pos: defs::Cursor,
    pub pos: defs::Cursor,
    pub public: bool,
    pub return_type: definers::DefinerCollecting,
    pub return_pos: defs::Cursor,
    pub body_pos: defs::Cursor,
    pub file_keys: Vec<FileKey>,
    pub inner_page_id: usize,
    pub hash: usize,
}
