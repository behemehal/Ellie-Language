use crate::syntax::types;
use serde::Serialize;

use alloc::vec::Vec;
use alloc::string::String;
use alloc::boxed::Box;


#[derive(PartialEq, Default, Debug, Clone, Serialize)]
pub struct RefferenceType {
    pub refference: Box<types::Types>,
    pub on_dot: bool,
    pub chain: Vec<String>,
}