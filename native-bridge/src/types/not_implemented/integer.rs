use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum IntegerTypes {
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
    USize,
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum IntegerSize {
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
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct IntegerType {
    pub value: IntegerSize,
    pub rtype: IntegerTypes,
}
