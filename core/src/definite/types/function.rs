use crate::definite::{definers, items::Collecting};
use crate::defs;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct FunctionParameter {
    pub name: String,
    pub rtype: Option<definers::DefinerCollecting>,
    pub name_pos: defs::Cursor,
    pub rtype_pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub parameters: Vec<FunctionParameter>,
    pub has_parameter_definings: bool,
    pub return_type: definers::DefinerCollecting,
    pub inside_code: Vec<Collecting>,
    pub return_pos: defs::Cursor,
    pub arrow_function: bool,
}
