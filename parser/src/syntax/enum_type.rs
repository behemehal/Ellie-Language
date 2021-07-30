use crate::syntax::definers;
use alloc::string::String;
use alloc::vec::Vec;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct EnumItem {
    pub has_type: bool,
    pub identifier: String,
    pub enum_type: definers::DefinerCollecting,
}

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct EnumType {
    pub public: bool,
    pub name: String,
    pub items: Vec<EnumItem>,
}

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct EnumTypeCollector {
    pub data: EnumType,
    pub name_collected: bool,
    pub identifier_collected: bool,
    pub type_collected: bool,
    pub at_comma: bool,
}
