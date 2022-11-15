use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum AttributeType {
    Property,
    Method,
    Setter,
    Getter,
    EnumItemData,
    EnumItemNoData,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub _rtype: AttributeType,
    pub name: String,
    pub page: usize,
    pub hash: usize,
    pub class_hash: usize,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ClassInstance {
    pub class_name: String,
    pub class_hash: usize,
    pub class_page: usize,
    pub attributes: Vec<Attribute>,
}
