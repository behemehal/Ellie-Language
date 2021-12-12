use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfItem {
    pub class_page: u64,
    pub class_hash: String,
}
