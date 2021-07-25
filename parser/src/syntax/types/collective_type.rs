use ellie_core::defs;
use crate::syntax::types;
use serde::Serialize;

use alloc::string::String;
use alloc::boxed::Box;
use alloc::vec::Vec;

#[derive(PartialEq, Default, Debug, Clone, Serialize)]
pub struct CollectiveEntry {
    pub key: String,
    pub value: Box<types::Types>,
    pub key_pos: defs::Cursor,
    pub value_pos: defs::Cursor
}

#[derive(PartialEq, Default, Debug, Clone, Serialize)]
pub struct CollectiveEntryCollector {
    pub data: Box<types::Types>,
    pub key_collected: bool,
    pub value_collected: bool,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize)]
pub struct Collective {
    pub entries: Vec<CollectiveEntryCollector>
}

#[derive(PartialEq, Default, Debug, Clone, Serialize)]
pub struct CollectiveCollector {
    pub complete: bool,
    pub data: Collective,
}