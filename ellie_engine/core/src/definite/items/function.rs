use crate::definite::definers;
use crate::defs;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use super::file_key::FileKey;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct FunctionParameter {
    pub name: String,
    pub rtype: definers::DefinerCollecting,
    pub name_pos: defs::Cursor,
    pub rtype_pos: defs::Cursor,
    pub multi_capture: bool,
    pub is_mut: bool,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: definers::DefinerCollecting,
    pub inner_page_id: usize,
    pub public: bool,
    pub name_pos: defs::Cursor,
    pub body_pos: defs::Cursor,
    pub file_keys: Vec<FileKey>,
    pub no_return: bool,
    pub parameters_pos: defs::Cursor,
    pub return_pos: defs::Cursor,
    pub pos: defs::Cursor,
    pub hash: usize,
}
