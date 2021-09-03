use crate::definite::items::Collecting;
use crate::definite::{definers, items::function};
use crate::defs;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ArrowFunction {
    pub parameters: Vec<function::FunctionParameter>,
    pub return_type: definers::DefinerCollecting,
    pub inside_code: Vec<Collecting>,
    pub return_pos: defs::Cursor,
}
