use serde::Serialize;

#[derive(PartialEq, Eq, Default, Debug, Clone, Serialize)]
pub struct VariableType {
    pub value_complete: bool,
    pub value: String,
}