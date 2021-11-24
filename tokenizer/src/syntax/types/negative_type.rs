use crate::processors::types;
use alloc::boxed::Box;
use ellie_core::definite;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Negative {
    pub value: Box<types::Processors>,
    pub char_available: bool,
    pub itered_cache: Box<types::TypeProcessor>,
}

impl Negative {
    pub fn to_definite(self) -> definite::types::negative::Negative {
        definite::types::negative::Negative {
            value: Box::new(self.value.to_definite()),
        }
    }

    pub fn from_definite(self, from: definite::types::negative::Negative) -> Self {
        Negative {
            value: Box::new(
                types::Processors::default()
                    .from_definite(ellie_core::definite::types::Types::Negative(from)),
            ),
            char_available: false,
            itered_cache: Box::new(types::TypeProcessor::default()),
        }
    }
}
