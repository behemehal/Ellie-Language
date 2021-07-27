use crate::syntax::types;
use alloc::boxed::Box;
use serde::Serialize;

#[derive(PartialEq, Default, Debug, Clone, Serialize)]
pub struct Negative {
    pub value: Box<types::Types>,
}
