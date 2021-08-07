use crate::syntax::types;
use ellie_core::defs;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Callers {
    FunctionCall(types::function_call::FunctionCallCollector),
    NewCall(types::new_call::NewCall),
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Caller {
    pub value: types::Types,
    pub pos: defs::Cursor,
}
