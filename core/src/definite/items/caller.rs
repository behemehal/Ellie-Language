use crate::definite::types;
use crate::defs;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Callers {
    FunctionCall(types::function_call::FunctionCall),
    ConstructedClass(types::constructed_class::ConstructedClass),
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Caller {
    pub value: types::Types,
    pub pos: defs::Cursor,
}