use crate::processors::types;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallParameter {
    pub value: types::Processors,
    pub pos: defs::Cursor,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub target: Box<types::Processors>,
    pub target_pos: defs::Cursor,
    pub parameters: Vec<FunctionCallParameter>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallCollector {
    pub data: FunctionCall,
    pub param_started: bool,
    pub complete: bool,
    pub itered_cache: Box<types::TypeProcessor>,
}

impl definite::Converter<FunctionCallCollector, definite::types::function_call::FunctionCall>
    for FunctionCallCollector
{
    fn to_definite(self) -> definite::types::function_call::FunctionCall {
        definite::types::function_call::FunctionCall {
            target: Box::new(self.data.target.to_definite()),
            target_pos: self.data.target_pos,
            params: self
                .data
                .parameters
                .into_iter()
                .map(|x| definite::types::function_call::FunctionCallParameter {
                    value: x.value.to_definite(),
                    pos: x.pos,
                })
                .collect(),
        }
    }

    fn from_definite(
        self,
        from: definite::types::function_call::FunctionCall,
    ) -> FunctionCallCollector {
        FunctionCallCollector {
            data: FunctionCall {
                target: Box::new(types::Processors::default().from_definite(*from.target)),
                target_pos: from.target_pos,
                parameters: from
                    .params
                    .into_iter()
                    .map(|x| FunctionCallParameter {
                        value: types::Processors::default().from_definite(x.value),
                        pos: x.pos,
                    })
                    .collect(),
            },
            ..Default::default()
        }
    }
}
