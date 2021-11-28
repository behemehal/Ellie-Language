use alloc::string::String;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct StringType {
    pub value: String,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct StringTypeCollector {
    pub data: StringType,
    pub complete: bool,
    pub comma_started: bool,
}

impl definite::Converter<StringTypeCollector, definite::types::string::StringType>
    for StringTypeCollector
{
    fn to_definite(self) -> definite::types::string::StringType {
        definite::types::string::StringType {
            value: self.data.value,
            pos: self.data.pos,
        }
    }

    fn from_definite(self, from: definite::types::string::StringType) -> Self {
        StringTypeCollector {
            data: StringType {
                value: from.value,
                pos: from.pos,
            },
            complete: true,
            ..Default::default()
        }
    }
}
