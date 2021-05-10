use serde::Serialize;
use alloc::string::String;

#[derive(PartialEq, Eq, Default, Debug, Clone, Serialize)]
pub struct CharType {
    pub value: char,
    pub complete: bool,
}