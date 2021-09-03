use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FloatTypes {
    F32,
    F64,
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FloatSize {
    F32(f32),
    F64(f64),
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct FloatType {
    pub value: FloatSize,
    pub rtype: FloatTypes,
}
