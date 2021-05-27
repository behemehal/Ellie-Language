use crate::syntax::types;
use serde::Serialize;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
#[no_mangle]
#[derive(PartialEq, Default, Debug, Clone, Serialize)]
pub struct RefferenceType {
    pub refference: Box<types::Types>,
    pub on_dot: bool,
    pub chain: Vec<String>,
}
