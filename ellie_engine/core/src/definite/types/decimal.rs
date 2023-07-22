use crate::defs;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum DecimalTypeEnum {
    Float(f64),
    Double(f32),
}

impl Default for DecimalTypeEnum {
    fn default() -> Self {
        DecimalTypeEnum::Float(0.0)
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct DecimalType {
    pub value: DecimalTypeEnum,
    pub pos: defs::Cursor,
    pub is_double: bool,
}
