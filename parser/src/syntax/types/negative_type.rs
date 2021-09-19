use crate::syntax::types;
use alloc::boxed::Box;
use ellie_core::definite;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct Negative {
    pub value: Box<types::Types>,
}

impl Negative {
    pub fn to_definite(self) -> definite::types::negative::Negative {
        definite::types::negative::Negative {
            value: Box::new(self.value.to_definite()),
        }
    }

    pub fn from_definite(self, from: definite::types::negative::Negative) -> Self {
        Negative {
            value: Box::new(types::Types::default().from_definite(*from.value)),
        }
    }
}
