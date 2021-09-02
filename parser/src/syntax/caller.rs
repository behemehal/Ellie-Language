use crate::syntax::types;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Callers {
    FunctionCall(types::function_call::FunctionCallCollector),
    ConstructedClass(types::constructed_class::ConstructedClass),
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Caller {
    pub value: types::Types,
    pub pos: defs::Cursor,
}

impl Caller {
    pub fn to_definite(self) -> definite::items::caller::Caller {
        definite::items::caller::Caller {
            value: self.value.to_definite(),
            pos: self.pos,
        }
    }
}
