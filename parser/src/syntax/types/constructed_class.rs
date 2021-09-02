use crate::syntax::types;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct ConstructedClassParameter {
    pub value: types::Types,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct ConstructedClass {
    pub value: Box<types::Types>,
    pub keyword_pos: defs::Cursor,
    pub value_pos: defs::Cursor,
    pub params: Vec<ConstructedClassParameter>,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct ConstructedClassCollector {
    pub data: ConstructedClass,
    pub keyword_collected: bool,
    pub keyword_index: i8,
    pub raw_value: String,
    pub value_collected: bool,
    pub comma: bool,
    pub complete: bool,
}

impl ConstructedClassCollector {
    pub fn to_definite(self) -> definite::types::constructed_class::ConstructedClass {
        definite::types::constructed_class::ConstructedClass {
            value: Box::new(self.data.value.to_definite()),
            keyword_pos: self.data.keyword_pos,
            value_pos: self.data.value_pos,
            params: self
                .data
                .params
                .into_iter()
                .map(
                    |x| definite::types::constructed_class::ConstructedClassParameter {
                        value: x.value.to_definite(),
                        pos: x.pos,
                    },
                )
                .collect(),
        }
    }
}
