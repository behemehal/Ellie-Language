use crate::syntax::types;
use alloc::boxed::Box;
use serde::Deserialize;
use serde::Serialize;

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct Negative {
    pub value: Box<types::Types>,
}
