use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct TypeDef {
    pub name: String,
    pub hash: u64,
}