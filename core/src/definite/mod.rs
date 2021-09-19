use crate::alloc::string::String;
use crate::alloc::vec::Vec;
pub mod definers;
pub mod items;
pub mod types;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DefiniteParsed {
    pub name: String,
    pub items: Vec<items::Collecting>,
}
