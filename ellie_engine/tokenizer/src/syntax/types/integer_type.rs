use alloc::fmt::Debug;
use alloc::string::String;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct IntegerType {
    pub value: isize,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct IntegerTypeCollector {
    pub data: IntegerType,
    pub raw: String,
    pub complete: bool,
}

impl definite::Converter<IntegerTypeCollector, definite::types::integer::IntegerType>
    for IntegerTypeCollector
{
    fn to_definite(self) -> definite::types::integer::IntegerType {
        definite::types::integer::IntegerType {
            value: self.data.value,
            pos: self.data.pos,
        }
    }

    fn from_definite(self, from: definite::types::integer::IntegerType) -> Self {
        let value = from.value;
        IntegerTypeCollector {
            data: IntegerType {
                value,
                pos: from.pos,
            },
            raw: value.to_string(),
            complete: true,
        }
    }
}
