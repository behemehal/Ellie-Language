use serde::Serialize;

#[derive(PartialEq, Default, Debug, Clone, Copy, Serialize)]
pub struct DoubleType {
    pub value: f32,
    pub complete: bool,
}