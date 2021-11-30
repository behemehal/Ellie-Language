use crate::definite::items::Collecting;
use crate::definite::{definers, items::function};
use crate::defs;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ArrowFunctionParameter {
    pub name: String,
    pub rtype: Option<definers::DefinerCollecting>,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ArrowFunction {
    pub parameters: Vec<ArrowFunctionParameter>,
    pub has_parameter_definings: bool,
    pub return_type: definers::DefinerCollecting,
    pub inside_code: Vec<Collecting>,
    pub return_pos: defs::Cursor,
}
