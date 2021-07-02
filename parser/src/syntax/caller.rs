use crate::syntax::types;
use ellie_core::defs;
use serde::Serialize;

use alloc::string::{String};

#[derive(PartialEq, Debug, Clone, Serialize)]
pub enum Callers {
    FunctionCall(types::function_call::FunctionCallCollector)
}

#[derive(PartialEq, Debug, Clone, Serialize)]
pub struct Caller {
    pub value: types::Types,
    pub pos: defs::Cursor,
}
