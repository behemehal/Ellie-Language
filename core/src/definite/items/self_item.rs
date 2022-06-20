use serde::Deserialize;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct SelfItem {
    pub class_page: usize,
    pub class_hash: usize,
}
