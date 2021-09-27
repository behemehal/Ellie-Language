use alloc::boxed::Box;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ImportItem {
    pub from_path: String,
    pub resolution_id: u64,
    pub from_import: u64,
    pub item: Box<crate::definite::items::Collecting>,
    pub public: bool,
}
