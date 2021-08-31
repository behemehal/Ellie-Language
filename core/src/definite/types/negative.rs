use crate::definite::types;
use alloc::boxed::Box;
use serde::{Deserialize, Serialize};

#[derive(PartialEq,  Debug, Clone, Serialize, Deserialize)]
pub struct Negative {
    pub value: Box<types::Types>,
}