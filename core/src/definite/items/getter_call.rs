use crate::{definite::types::Types, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct GetterCall {
    pub data: Types,
    pub pos: defs::Cursor,
}
