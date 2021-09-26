pub use crate::parser;
use alloc::boxed::Box;
use alloc::string::String;
use ellie_core::definite;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImportItem {
    pub from_path: String,
    pub resolution_id: u64,
    pub item: Box<parser::Collecting>,
    pub public: bool,
}

impl ImportItem {
    pub fn to_definite(self) -> definite::items::import_item::ImportItem {
        definite::items::import_item::ImportItem {
            resolution_id: self.resolution_id,
            from_path: self.from_path,
            item: Box::new(self.item.to_definite()),
            public: self.public,
        }
    }

    pub fn from_definite(self, from: definite::items::import_item::ImportItem) -> Self {
        ImportItem {
            resolution_id: from.resolution_id,
            from_path: from.from_path,
            item: Box::new(parser::Collecting::default().from_definite(*from.item)),
            public: from.public,
        }
    }
}
