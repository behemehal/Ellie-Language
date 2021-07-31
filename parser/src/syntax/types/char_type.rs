use serde::Deserialize;
use serde::Serialize;

#[derive(PartialEq, Eq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct CharType {
    pub value: char,
    pub complete: bool,
}
