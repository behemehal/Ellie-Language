use crate::{definite, defs};
use serde::{Deserialize, Serialize};
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct BracketReference {
    pub pos: defs::Cursor,
}
