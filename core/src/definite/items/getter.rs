use crate::definite::definers;
use crate::definite::items::Collecting;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]

pub struct Getter {
    pub name: String,
    pub rtype: definers::DefinerCollecting,
    pub code: Vec<Collecting>,
}
