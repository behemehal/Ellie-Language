use crate::processors::types;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CloakEntry {
    pub value: types::Processors,
    pub location: defs::Cursor,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CloakType {
    pub collective: Vec<CloakEntry>,
    pub pos: defs::Cursor,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CloakTypeCollector {
    pub data: CloakType,
    pub complete: bool,
    pub brace_started: bool,
    pub itered_cache: Box<types::TypeProcessor>,
}

impl definite::Converter<CloakTypeCollector, definite::types::cloak::CloakType>
    for CloakTypeCollector
{
    fn to_definite(self) -> definite::types::cloak::CloakType {
        definite::types::cloak::CloakType {
            collective: self
                .data
                .collective
                .into_iter()
                .map(|x| definite::types::cloak::CloakEntry {
                    value: x.value.to_definite(),
                    location: x.location,
                })
                .collect(),
            pos: self.data.pos,
        }
    }

    fn from_definite(self, from: definite::types::cloak::CloakType) -> CloakTypeCollector {
        CloakTypeCollector {
            data: CloakType {
                collective: from
                    .collective
                    .into_iter()
                    .map(|x| CloakEntry {
                        value: types::Processors::default().from_definite(x.value),
                        location: x.location,
                    })
                    .collect(),
                pos: from.pos,
            },
            ..Default::default()
        }
    }
}
