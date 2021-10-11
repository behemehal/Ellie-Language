use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct CharType {
    pub value: char,
}
