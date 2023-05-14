use crate::syntax::items::definers;
use ellie_core::{definite::Converter, defs};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct GenericDefining {
    pub name: String,
    pub hash: usize,
    pub pos: defs::Cursor,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EnumItem {
    pub has_type: bool,
    pub identifier: String,
    pub enum_type: definers::DefinerCollector,
    pub identifier_pos: defs::Cursor,
    pub type_complete: bool,
    pub type_pos: defs::Cursor,
    pub identifier_collected: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EnumType {
    pub public: bool,
    pub name_pos: defs::Cursor,
    pub name_collected: bool,

    pub continuum_collected: bool,
    pub generics_collected: bool,
    pub generic_definings: Vec<GenericDefining>,

    pub items: Vec<EnumItem>,
    pub hash: usize,

    pub name: String,
    pub pos: defs::Cursor,
    pub body_pos: defs::Cursor,
    pub complete: bool,
}

impl Converter<EnumType, ellie_core::definite::items::enum_type::EnumType> for EnumType {
    fn to_definite(self) -> ellie_core::definite::items::enum_type::EnumType {
        todo!()
    }

    fn from_definite(self, _: ellie_core::definite::items::enum_type::EnumType) -> EnumType {
        todo!()
    }
}
