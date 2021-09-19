use crate::alloc::borrow::ToOwned;
use crate::syntax::types;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

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
    //pub entry_collector: (Box<types::Types>, Box<types::Types>), //If new data added to collective we can track it. TO-DO !?
    pub data: Collective,
}

impl CollectiveCollector {
    pub fn to_definite(self) -> definite::types::collective::Collective {
        definite::types::collective::Collective {
            entries: self
                .data
                .entries
                .into_iter()
                .map(|x| definite::types::collective::CollectiveEntry {
                    key: Box::new(x.data.key.to_definite()),
                    value: Box::new(x.data.value.to_definite()),
                    key_pos: x.data.key_pos,
                    value_pos: x.data.value_pos,
                })
                .collect(),
        }
    }

    pub fn from_definite(self, from: definite::types::collective::Collective) -> Self {
        CollectiveCollector {
            complete: true,
            at_comma: false,
            data: Collective {
                entries: from
                    .entries
                    .into_iter()
                    .map(|x| CollectiveEntryCollector {
                        data: CollectiveEntry {
                            key: Box::new(types::Types::default().from_definite(*x.key)),
                            value: Box::new(types::Types::default().from_definite(*x.value)),
                            key_pos: x.key_pos,
                            value_pos: x.value_pos,
                        },
                        key_collected: true,
                        value_collected: true,
                    })
                    .collect::<Vec<_>>(),
            },
        }
    }

    pub fn has_dedup(&self) -> bool {
        let mut existent_names: Vec<String> = Vec::with_capacity(self.data.entries.len());
        let mut duplicate = false;
        for i in &self.data.entries {
            let current_key = match *i.data.key.clone() {
                types::Types::String(e) => e.data.value,
                types::Types::Integer(e) => e.raw,
                types::Types::Char(e) => e.value.to_string(),
                types::Types::Float(e) => e.data.raw,
                _ => "".to_owned(),
            };

            if existent_names.contains(&current_key) {
                duplicate = true;
                break;
            } else {
                existent_names.push(current_key.to_string());
            }
        }
        duplicate
    }
}
