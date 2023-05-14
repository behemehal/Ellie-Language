use alloc::string::String;
use ellie_core::{definite::Converter, defs};
use serde::{Deserialize, Serialize};

use crate::processors::types::{Processors, TypeProcessor};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct FileKey {
    pub key_name: String,
    pub value: Processors,
    pub keyword_collected: bool,
    pub name_collected: bool,
    pub key_name_location: defs::Cursor,
    pub value_location: defs::Cursor,
    pub pos: defs::Cursor,
    pub value_cache: TypeProcessor,
    pub is_global: bool,
    pub complete: bool,
}

impl Converter<FileKey, ellie_core::definite::items::file_key::FileKey> for FileKey {
    fn to_definite(self) -> ellie_core::definite::items::file_key::FileKey {
        ellie_core::definite::items::file_key::FileKey {
            key_name: self.key_name,
            value: self.value.to_definite(),
            key_name_location: self.key_name_location,
            is_global: self.is_global,
            value_location: self.value_location,
            pos: self.pos,
        }
    }

    fn from_definite(self, from: ellie_core::definite::items::file_key::FileKey) -> FileKey {
        FileKey {
            key_name: from.key_name,
            value: Processors::default().from_definite(from.value),
            key_name_location: from.key_name_location,
            value_location: from.value_location,
            pos: from.pos,
            is_global: from.is_global,
            ..Default::default()
        }
    }
}
