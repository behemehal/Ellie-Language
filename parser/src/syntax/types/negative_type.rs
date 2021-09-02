use crate::syntax::types;
use alloc::boxed::Box;
use serde::{Deserialize, Serialize};
use ellie_core::definite;

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
}
