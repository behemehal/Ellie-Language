use serde::{Deserialize, Serialize};
use alloc::string::String;

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct DoubleType {
    pub value: f32,
    pub complete: bool,
}
