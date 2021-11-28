use crate::{definite::types::{Types, assignment_type::AssignmentOperators}, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct SetterCall {
    pub target: Types,
    pub value: Types,
    pub pos: defs::Cursor,
    pub operator: AssignmentOperators,
}
