pub use crate::parser;
use alloc::boxed::Box;
use alloc::string::String;
use serde::{Deserialize, Serialize};
use ellie_core::definite;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ImportItem {
    pub from_path: String,
    pub item: Box<parser::Collecting>,
    pub public: bool,
}

impl ImportItem {
    pub fn to_definite(self) -> definite::items::import_item::ImportItem {
        definite::items::import_item::ImportItem {
            from_path: self.from_path,
            item: Box::new(self.item.to_definite()),
            public: self.public,
        }
    }
}
