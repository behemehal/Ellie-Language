use crate::{
    definite::types::{operator::AssignmentOperators, Types},
    defs,
};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct SetterCall {
    pub target: Types,
    pub value: Types,
    pub target_pos: defs::Cursor,
    pub value_pos: defs::Cursor,
    pub operator: AssignmentOperators,
}
