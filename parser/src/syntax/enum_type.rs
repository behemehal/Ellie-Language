use crate::syntax::definers;
use alloc::string::String;
use alloc::vec::Vec;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnumItem {
    pub has_type: bool,
    pub identifier: String,
    pub enum_type: definers::DefinerCollecting,
    pub identifier_pos: defs::Cursor,
    pub type_pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnumType {
    pub public: bool,
    pub name: String,
    pub items: Vec<EnumItem>,
    pub name_pos: defs::Cursor,
    pub brace_start_pos: defs::Cursor,
    pub brace_end_pos: defs::Cursor,
}

impl EnumType {
    pub fn to_definite(self) -> definite::items::enum_type::EnumType {
        definite::items::enum_type::EnumType {
            public: self.public,
            name: self.name,
            items: self
                .items
                .into_iter()
                .map(|x| definite::items::enum_type::EnumItem {
                    has_type: x.has_type,
                    identifier: x.identifier,
                    enum_type: x.enum_type.to_definite(),
                    identifier_pos: x.identifier_pos,
                    type_pos: x.type_pos,
                })
                .collect(),
            name_pos: self.name_pos,
            brace_start_pos: self.brace_start_pos,
            brace_end_pos: self.brace_end_pos,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnumTypeCollector {
    pub data: EnumType,
    pub name_collected: bool,
    pub identifier_collected: bool,
    pub type_collected: bool,
    pub at_comma: bool,
    pub at_identifier: bool,
}

impl EnumTypeCollector {
    pub fn to_definite(self) -> definite::items::enum_type::EnumType {
        self.data.to_definite()
    }

    pub fn from_definite(self, from: definite::items::enum_type::EnumType) -> Self {
        EnumTypeCollector {
            data: EnumType {
                public: from.public,
                name: from.name,
                items: from
                    .items
                    .into_iter()
                    .map(|x| EnumItem {
                        has_type: x.has_type,
                        identifier: x.identifier,
                        enum_type: definers::DefinerCollecting::default()
                            .from_definite(x.enum_type),
                        identifier_pos: x.identifier_pos,
                        type_pos: x.type_pos,
                    })
                    .collect(),
                name_pos: from.name_pos,
                brace_start_pos: from.brace_start_pos,
                brace_end_pos: from.brace_end_pos,
            },
            ..Default::default()
        }
    }

    pub fn has_dedup(&self) -> bool {
        let mut existent_names: Vec<String> = Vec::with_capacity(self.data.items.len());
        let mut duplicate = false;
        for i in &self.data.items {
            if existent_names.contains(&i.identifier) {
                duplicate = true;
                break;
            } else {
                existent_names.push(i.identifier.clone())
            }
        }
        duplicate
    }
}
