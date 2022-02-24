use crate::processors::types;
use alloc::boxed::Box;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct NullResolver {
    pub target: Box<types::Processors>,
    pub target_pos: defs::Cursor,
    pub pos: defs::Cursor,
}

impl definite::Converter<NullResolver, definite::types::null_resolver::NullResolver>
    for NullResolver
{
    fn to_definite(self) -> definite::types::null_resolver::NullResolver {
        definite::types::null_resolver::NullResolver {
            target: Box::new(self.target.to_definite()),
            target_pos: self.target_pos,
            pos: self.pos,
        }
    }

    fn from_definite(self, _from: definite::types::null_resolver::NullResolver) -> NullResolver {
        todo!()
    }
}
