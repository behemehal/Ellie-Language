use crate::processors::types;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Pointer {
    NoData,
    Data(Box<types::Processors>),
}

impl Default for Pointer {
    fn default() -> Self {
        Pointer::NoData
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EnumData {
    pub reference: Box<types::Processors>,
    pub reference_pos: defs::Cursor,
    pub brace_pos: defs::Cursor,
    pub value: Pointer,
    pub field_name: String,
    pub pos: defs::Cursor,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EnumDataCollector {
    pub data: EnumData,
    pub brace_started: bool,
    pub itered_cache: Box<types::TypeProcessor>,
    pub complete: bool,
}

impl definite::Converter<EnumDataCollector, definite::types::enum_data::EnumData>
    for EnumDataCollector
{
    fn to_definite(self) -> definite::types::enum_data::EnumData {
        definite::types::enum_data::EnumData {
            reference: Box::new(self.data.reference.to_definite()),
            reference_pos: self.data.reference_pos,
            brace_pos: self.data.brace_pos,
            field_name: self.data.field_name,
            value: match self.data.value {
                Pointer::NoData => definite::types::enum_data::Pointer::NoData,
                Pointer::Data(data) => {
                    definite::types::enum_data::Pointer::Data(Box::new(data.to_definite()))
                }
            },
            pos: self.data.pos,
        }
    }

    fn from_definite(self, from: definite::types::enum_data::EnumData) -> EnumDataCollector {
        EnumDataCollector {
            data: EnumData {
                reference: Box::new(types::Processors::default().from_definite(*from.reference)),
                reference_pos: from.reference_pos,
                brace_pos: from.brace_pos,
                field_name: from.field_name,
                value: match from.value {
                    definite::types::enum_data::Pointer::NoData => Pointer::NoData,
                    definite::types::enum_data::Pointer::Data(val) => {
                        Pointer::Data(Box::new(types::Processors::default().from_definite(*val)))
                    }
                },
                pos: from.pos,
            },
            ..Default::default()
        }
    }
}
