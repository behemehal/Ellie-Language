use serde::Serialize;

#[repr(C)]
#[derive(PartialEq, Eq, Default, Debug, Clone, Copy, Serialize)]
pub struct BoolType {
    pub value: bool,
}
