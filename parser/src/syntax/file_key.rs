use crate::syntax::types;
use alloc::string::String;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileKey {
    pub key_name: String,
    pub value: types::Types,
    pub key_name_location: defs::Cursor,
    pub value_location: defs::Cursor,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileKeyCollector {
    pub data: FileKey,
    pub key_name_collected: bool,
    pub value_collected: bool,
}

impl FileKeyCollector {
    pub fn to_definite(self) -> definite::items::file_key::FileKey {
        definite::items::file_key::FileKey {
            key_name: self.data.key_name,
            value: self.data.value.to_definite(),
            key_name_location: self.data.key_name_location,
            value_location: self.data.value_location,
            pos: self.data.pos,
        }
    }

    pub fn from_definite(self, from: definite::items::file_key::FileKey) -> Self {
        FileKeyCollector {
            data: FileKey {
                key_name: from.key_name,
                value: types::Types::default().from_definite(from.value),
                key_name_location: from.key_name_location,
                value_location: from.value_location,
                pos: from.pos,
            },
            ..Default::default()
        }
    }
}
