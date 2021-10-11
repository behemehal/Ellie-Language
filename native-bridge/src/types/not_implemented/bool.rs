use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct BoolType {
    pub value: bool,
}
