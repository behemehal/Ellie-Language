use crate::processors::types;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BraceReferenceType {
    pub reference: Box<types::Processors>,
    pub reference_pos: defs::Cursor,
    pub brace_pos: defs::Cursor,
    pub value: Box<types::Processors>,
    pub pos: defs::Cursor,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BraceReferenceTypeCollector {
    pub data: BraceReferenceType,
    pub brace_started: bool,
    pub itered_cache: Box<types::TypeProcessor>,
    pub complete: bool,
}

impl
    definite::Converter<
        BraceReferenceTypeCollector,
        definite::types::brace_reference::BraceReferenceType,
    > for BraceReferenceTypeCollector
{
    fn to_definite(self) -> definite::types::brace_reference::BraceReferenceType {
        definite::types::brace_reference::BraceReferenceType {
            reference: Box::new(self.data.reference.to_definite()),
            reference_pos: self.data.reference_pos,
            brace_pos: self.data.brace_pos,
            value: Box::new(self.data.value.to_definite()),
            pos: self.data.pos,
        }
    }

    fn from_definite(
        self,
        from: definite::types::brace_reference::BraceReferenceType,
    ) -> BraceReferenceTypeCollector {
        BraceReferenceTypeCollector {
            data: BraceReferenceType {
                reference: Box::new(types::Processors::default().from_definite(*from.reference)),
                reference_pos: from.reference_pos,
                brace_pos: from.brace_pos,
                value: Box::new(types::Processors::default().from_definite(*from.value)),
                pos: from.pos,
            },
            ..Default::default()
        }
    }
}
