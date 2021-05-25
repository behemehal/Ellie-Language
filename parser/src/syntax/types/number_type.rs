use alloc::fmt::Display;
use alloc::fmt::Formatter;
use alloc::string::String;
use serde::Serialize;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize)]
pub enum NumberTypes {
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
    F32,
    F64,
}

impl Default for NumberTypes {
    fn default() -> Self {
        NumberTypes::I8
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize)]
pub enum NumberSize {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    F32(f32),
    F64(f64),
}

impl Display for NumberSize {
    fn fmt(&self, f: &mut Formatter) -> alloc::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

impl Default for NumberSize {
    fn default() -> Self {
        NumberSize::I64(0)
    }
}

#[derive(PartialEq, Default, Debug, Clone, Serialize)]
pub struct NumberType {
    pub value: NumberSize,
    pub raw: String,
    pub r#type: NumberTypes,
    pub complete: bool,
}