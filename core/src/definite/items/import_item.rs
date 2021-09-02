use alloc::boxed::Box;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ImportItem {
    pub from_path: String,
    pub item: Box<crate::definite::items::Collecting>,
    pub public: bool,
}
