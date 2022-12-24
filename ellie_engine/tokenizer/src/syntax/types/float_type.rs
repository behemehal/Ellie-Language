use super::integer_type::IntegerTypeCollector;
use alloc::string::String;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct FloatType {
    pub value: f32,
    pub raw: String,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct FloatTypeCollector {
    pub data: FloatType,
    pub base: String,
    pub point: String,
    pub base_p: IntegerTypeCollector,
    pub at_point: bool,
    pub complete: bool,
    pub no_base: bool,
}

impl definite::Converter<FloatTypeCollector, definite::types::float::FloatType>
    for FloatTypeCollector
{
    fn to_definite(self) -> definite::types::float::FloatType {
        definite::types::float::FloatType {
            value: self.data.value,
            pos: self.data.pos,
        }
    }

    fn from_definite(self, from: definite::types::float::FloatType) -> Self {
        let raw = from.value.to_string();
        let partitions = raw.split(".").collect::<Vec<_>>();

        FloatTypeCollector {
            data: FloatType {
                value: from.value,
                raw: raw.clone(),
                pos: from.pos,
            },
            base: partitions[0].to_owned(),
            point: partitions[1].to_owned(),
            at_point: true,
            complete: true,
            ..Default::default()
        }
    }
}
