use crate::syntax::{definers, types};
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct BracketReferenceCollector {
    pub complete: bool,
    pub data: BracketReference,
    pub root_available: bool,
    pub resolved: definers::DefinerCollecting,
}

impl BracketReferenceCollector {
    pub fn to_definite(self) -> definite::types::bracket_reference::BracketReference {
        todo!()
    }

    pub fn from_definite(self, from: definite::types::bracket_reference::BracketReference) -> Self {
        todo!()
    }
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct BracketReference {
    pub pos: defs::Cursor,
    pub target: definers::DefinerCollecting,
}
