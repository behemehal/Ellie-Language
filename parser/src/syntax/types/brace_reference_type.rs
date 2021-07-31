use serde::Deserialize;
use serde::Serialize;

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct BraceReferenceCollector {}
