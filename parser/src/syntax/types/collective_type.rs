use crate::syntax::types;
use ellie_core::defs;
use serde::{Deserialize, Serialize};

use alloc::boxed::Box;
use alloc::vec::Vec;

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveEntry {
    pub key: Box<types::Types>,
    pub value: Box<types::Types>,
    pub key_pos: defs::Cursor,
    pub value_pos: defs::Cursor,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveEntryCollector {
    pub data: CollectiveEntry,
    pub key_collected: bool,
    pub value_collected: bool,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct Collective {
    pub entries: Vec<CollectiveEntryCollector>,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveCollector {
    pub complete: bool,
    pub at_comma: bool,
    pub entry_collector: (Box<types::Types>, Box<types::Types>), //If new data added to collective we can track it. TO-DO !?
    pub data: Collective,
}

impl CollectiveCollector {
    pub fn has_dedup(&self) -> bool {
        let mut existent_names: Vec<types::Types> = Vec::with_capacity(self.data.entries.len());
        let mut duplicate = false;
        for i in &self.data.entries {
            if existent_names.contains(&*i.data.key.clone()) {
                duplicate = true;
                break;
            } else {
                existent_names.push(*i.data.key.clone())
            }
        }
        duplicate
    }
}
