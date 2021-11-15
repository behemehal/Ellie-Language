use alloc::string::String;
use enum_as_inner::EnumAsInner;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FloatTypes {
    F32,
    F64,
}

impl Default for FloatTypes {
    fn default() -> Self {
        FloatTypes::F32
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, EnumAsInner, Deserialize)]
pub enum FloatSize {
    F32(f32),
    F64(f64),
}

impl Default for FloatSize {
    fn default() -> Self {
        FloatSize::F32(0.0)
    }
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct FloatType {
    pub value: FloatSize,
    pub rtype: FloatTypes,
    pub raw: String,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct FloatTypeCollector {
    pub data: FloatType,
    pub base: String,
    pub point: String,
    pub at_point: bool,
    pub complete: bool,
}
