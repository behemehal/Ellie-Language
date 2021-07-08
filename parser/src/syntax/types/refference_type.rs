use crate::syntax::types;
use serde::Serialize;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

pub enum ChainType {
    Getter,
    Setter,
    FunctionCall,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Hash)]
pub struct RefferenceType {
    pub refference: Box<types::Types>,
    pub on_dot: bool,
    pub chain: Vec<String>,
}
