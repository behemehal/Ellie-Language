use crate::syntax::types;
use serde::Serialize;

#[derive(PartialEq, Default, Debug, Clone, Serialize)]
pub struct RefferenceType {
    pub refference: Box<types::Types>,
    pub on_dot: bool,
    pub chain: Vec<String>,
}