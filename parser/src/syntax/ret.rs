use serde::Serialize;
use crate::syntax::types;
use ellie_core::defs;

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct Ret {
    pub value: types::Types,
    pub keyword_pos: defs::Cursor,
    pub value_position: defs::Cursor
}