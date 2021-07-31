use crate::syntax::types;
use ellie_core::defs;
use serde::Deserialize;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Callers {
    FunctionCall(types::function_call::FunctionCallCollector),
    ClassCall(types::class_call::ClassCallCollector),
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Caller {
    pub value: types::Types,
    pub pos: defs::Cursor,
}
