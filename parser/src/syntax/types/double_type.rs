use serde::{Deserialize, Serialize};

use alloc::string::String;

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct DoubleType {
    pub value: f32,
    pub raw_value: String,
    pub complete: bool,
}
