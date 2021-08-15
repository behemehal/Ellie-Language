pub use crate::parser;
use alloc::boxed::Box;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ImportItem {
    pub from_path: String,
    pub item: Box<parser::Collecting>,
    pub public: bool,
}
