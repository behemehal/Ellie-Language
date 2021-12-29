use ellie_core::defs;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionParameter {
    pub name: String,
    pub reference: bool,
    pub rtype: ellie_core::definite::definers::DefinerCollecting,
    pub pos: defs::Cursor,
}
