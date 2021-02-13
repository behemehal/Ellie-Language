use serde::Serialize;

#[derive(PartialEq, Eq, Default, Debug, Clone, Serialize)]
pub struct StringType {
    pub value: String,
    pub quote_type: String,
    pub complete: bool,
}