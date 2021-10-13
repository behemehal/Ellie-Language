use ellie_core::definite::types::ellie_char;
use libc::c_char;

#[repr(C)]
pub struct CharType {
    pub value: c_char,
}

pub fn build_char_from(target: ellie_char::CharType) -> CharType {
    CharType {
        value: target.value as i8,
    }
}
