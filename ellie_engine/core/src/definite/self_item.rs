use serde::Deserialize;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct SelfItem {
    pub class_page: u64,
    pub class_hash: u64,
}
