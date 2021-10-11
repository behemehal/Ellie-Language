#[repr(C)]
pub enum IntegerSize {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    //U128(u128),
    Usize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    //I128(i128),
    Isize(isize),
}

#[repr(C)]
pub struct IntegerType {
    pub value: IntegerSize,
}
