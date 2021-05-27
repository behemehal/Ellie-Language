use serde::Serialize;

#[repr(C)]
#[no_mangle]
#[derive(PartialEq, Eq, Default, Debug, Clone, Copy, Serialize)]
pub struct BoolType {
    pub value: bool,
}
