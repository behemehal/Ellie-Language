use crate::processors::types;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveEntry {
    pub key: types::Processors,
    pub value: types::Processors,
    pub key_pos: defs::Cursor,
    pub value_pos: defs::Cursor,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveType {
    pub entries: Vec<CollectiveEntry>,
    pub pos: defs::Cursor,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveTypeCollector {
    pub data: CollectiveType,
    pub complete: bool,
    pub brace_started: bool,
    pub key_collected: bool,
    pub itered_cache: Box<types::TypeProcessor>,
}

impl definite::Converter<CollectiveTypeCollector, definite::types::collective::CollectiveType>
    for CollectiveTypeCollector
{
    fn to_definite(self) -> definite::types::collective::CollectiveType {
        definite::types::collective::CollectiveType {
            entries: self
                .data
                .entries
                .into_iter()
                .map(|x| definite::types::collective::CollectiveEntry {
                    value: x.value.to_definite(),
                    key: x.key.to_definite(),
                    key_pos: x.key_pos,
                    value_pos: x.value_pos,
                })
                .collect(),
            pos: self.data.pos,
        }
    }

    fn from_definite(
        self,
        from: definite::types::collective::CollectiveType,
    ) -> CollectiveTypeCollector {
        CollectiveTypeCollector {
            data: CollectiveType {
                entries: from
                    .entries
                    .into_iter()
                    .map(|x| CollectiveEntry {
                        value: types::Processors::default().from_definite(x.value),
                        key: types::Processors::default().from_definite(x.key),
                        key_pos: x.key_pos,
                        value_pos: x.value_pos,
                    })
                    .collect(),
                pos: from.pos,
            },
            ..Default::default()
        }
    }
}
