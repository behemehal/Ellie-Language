use crate::syntax::types;
use ellie_core::defs;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Serialize, Hash)]
pub enum Callers {
    FunctionCall(types::function_call::FunctionCallCollector),
}

#[derive(PartialEq, Debug, Clone, Serialize, Hash)]
pub struct Caller {
    pub value: types::Types,
    pub pos: defs::Cursor,
}
