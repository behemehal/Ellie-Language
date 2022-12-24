use super::integer_type::IntegerTypeCollector;
use alloc::string::String;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct DoubleType {
    pub value: f64,
    pub raw: String,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct DoubleTypeCollector {
    pub data: DoubleType,
    pub base: String,
    pub point: String,
    pub base_p: IntegerTypeCollector,
    pub at_point: bool,
    pub complete: bool,
    pub no_base: bool,
}

impl definite::Converter<DoubleTypeCollector, definite::types::double::DoubleType>
    for DoubleTypeCollector
{
    fn to_definite(self) -> definite::types::double::DoubleType {
        definite::types::double::DoubleType {
            value: self.data.value,
            pos: self.data.pos,
        }
    }

    fn from_definite(self, from: definite::types::double::DoubleType) -> Self {
        let raw = from.value.to_string();
        let partitions = raw.split(".").collect::<Vec<_>>();

        DoubleTypeCollector {
            data: DoubleType {
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
