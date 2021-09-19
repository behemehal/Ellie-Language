use alloc::string::{String, ToString};
use ellie_core::definite;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct BoolType {
    pub value: bool,
    pub raw: String,
}

impl BoolType {
    pub fn to_definite(self) -> definite::types::bool::BoolType {
        definite::types::bool::BoolType { value: self.value }
    }

    pub fn from_definite(self, from: definite::types::bool::BoolType) -> Self {
        BoolType {
            value: from.value,
            raw: from.value.to_string(),
        }
    }
}
