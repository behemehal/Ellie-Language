use crate::alloc::vec::Vec;
use crate::alloc::string::String;
pub mod definers;
pub mod items;
pub mod types;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DefiniteParsed {
    pub name: String,
    pub items: Vec<items::Collecting>,
}