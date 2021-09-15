use crate::definite::types;
use alloc::boxed::Box;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct NullResolver {
    pub value: Box<types::Types>,
}