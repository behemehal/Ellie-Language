use serde::Serialize;

#[derive(PartialEq, Eq, Default, Debug, Clone, Copy, Serialize)]
pub struct NumberType {
    pub value: usize,
    pub complete: bool,
}