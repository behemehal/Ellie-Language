use crate::alloc::string::String;
use crate::alloc::vec::Vec;
pub mod definers;
pub mod items;
pub mod types;
use serde::{Deserialize, Serialize};

/// Definite language items
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DefiniteTokenized {
    pub name: String,
    pub items: Vec<items::Collecting>,
}

/// Definite <-> tokenized data interface
pub trait Converter<F, T> {
    fn to_definite(self) -> T;
    fn from_definite(self, from: T) -> F;
}
