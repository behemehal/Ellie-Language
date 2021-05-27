use libc::c_char;

#[repr(C)]
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

#[repr(C)]
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

#[repr(C)]
pub struct NumberType {
    pub value: NumberSize,
    pub raw: *const c_char,
    pub r#type: NumberTypes,
    pub complete: bool,
}
