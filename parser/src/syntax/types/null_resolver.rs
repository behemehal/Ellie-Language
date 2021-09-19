use crate::syntax::types;
use alloc::boxed::Box;
use ellie_core::definite;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct NullResolver {
    pub value: Box<types::Types>,
}

impl NullResolver {
    pub fn to_definite(self) -> definite::types::null_resolver::NullResolver {
        definite::types::null_resolver::NullResolver {
            value: Box::new(self.value.to_definite()),
        }
    }

    pub fn from_definite(self, from: definite::types::null_resolver::NullResolver) -> Self {
        NullResolver {
            value: Box::new(types::Types::default().from_definite(*from.value)),
        }
    }
}
