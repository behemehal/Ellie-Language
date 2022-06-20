use crate::processors::types::{Processors, TypeProcessor};
use ellie_core::{definite::Converter, defs};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetterCall {
    pub data: Processors,
    pub complete: bool,
    pub cache: TypeProcessor,
    pub hash: usize,
    pub pos: defs::Cursor,
}

impl Converter<GetterCall, ellie_core::definite::items::getter_call::GetterCall> for GetterCall {
    fn to_definite(self) -> ellie_core::definite::items::getter_call::GetterCall {
        ellie_core::definite::items::getter_call::GetterCall {
            data: self.data.to_definite(),
            pos: self.pos,
        }
    }

    fn from_definite(
        self,
        from: ellie_core::definite::items::getter_call::GetterCall,
    ) -> GetterCall {
        GetterCall {
            data: Processors::default().from_definite(from.data),
            pos: from.pos,
            ..Default::default()
        }
    }
}
