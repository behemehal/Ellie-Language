use super::integer_type::IntegerTypeCollector;
use alloc::string::String;
use ellie_core::{
    definite::{self, types::decimal::DecimalTypeEnum},
    defs,
};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct DecimalType {
    pub value: DecimalTypeEnum,
    pub raw: String,
    pub pos: defs::Cursor,
    pub is_double: bool,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct DecimalTypeCollector {
    pub data: DecimalType,
    pub base: String,
    pub point: String,
    pub base_p: IntegerTypeCollector,
    pub at_point: bool,
    pub complete: bool,
    pub no_base: bool,
}

impl definite::Converter<DecimalTypeCollector, definite::types::decimal::DecimalType>
    for DecimalTypeCollector
{
    fn to_definite(self) -> definite::types::decimal::DecimalType {
        definite::types::decimal::DecimalType {
            value: self.data.value,
            is_double: self.data.is_double,
            pos: self.data.pos,
        }
    }

    fn from_definite(self, from: definite::types::decimal::DecimalType) -> Self {
        let raw = match from.value {
            DecimalTypeEnum::Float(e) => e.to_string(),
            DecimalTypeEnum::Double(e) => e.to_string(),
        };
        let partitions = raw.split(".").collect::<Vec<_>>();

        DecimalTypeCollector {
            data: DecimalType {
                value: from.value,
                raw: raw.clone(),
                pos: from.pos,
                is_double: from.is_double,
            },
            base: partitions[0].to_owned(),
            point: partitions[1].to_owned(),
            at_point: true,
            complete: true,
            ..Default::default()
        }
    }
}
