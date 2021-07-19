use serde::Serialize;

#[derive(PartialEq, Eq, Default, Debug, Clone, Serialize, Hash)]
pub struct CharType {
    pub value: char,
    pub complete: bool,
}
