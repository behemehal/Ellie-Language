use ellie_core::defs;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionParameter {
    pub name: String,
    pub type_pos: defs::Cursor,
    pub name_pos: defs::Cursor,
    pub rtype: crate::syntax::items::definers::DefinerCollector,
    pub pos: defs::Cursor,
}
