use serde::Serialize;

#[repr(C)]
#[no_mangle]
#[derive(PartialEq, Eq, Default, Debug, Clone, Serialize)]
pub struct CharType {
    pub value: char,
    pub complete: bool,
}
