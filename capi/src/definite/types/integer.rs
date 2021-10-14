use ellie_core::definite::types::integer;

#[repr(C)]
pub enum IntegerTypes {
    I8,
    I16,
    I32,
    I64,
    //I128,
    ISize,
    U8,
    U16,
    U32,
    U64,
    //U128,
    USize,
}

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
    pub rtype: IntegerTypes,
}

pub fn build_integer_from(target: integer::IntegerType) -> IntegerType {
    IntegerType {
        value: match target.value {
            integer::IntegerSize::U8(e) => IntegerSize::U8(e),
            integer::IntegerSize::U16(e) => IntegerSize::U16(e),
            integer::IntegerSize::U32(e) => IntegerSize::U32(e),
            integer::IntegerSize::U64(e) => IntegerSize::U64(e),
            integer::IntegerSize::U128(_) => panic!("NOT SUPPORTED"),
            integer::IntegerSize::Usize(e) => IntegerSize::Usize(e),
            integer::IntegerSize::I8(e) => IntegerSize::I8(e),
            integer::IntegerSize::I16(e) => IntegerSize::I16(e),
            integer::IntegerSize::I32(e) => IntegerSize::I32(e),
            integer::IntegerSize::I64(e) => IntegerSize::I64(e),
            integer::IntegerSize::I128(_) => panic!("NOT SUPPORTED"),
            integer::IntegerSize::Isize(e) => IntegerSize::Isize(e),
        },
        rtype: match target.rtype {
            integer::IntegerTypes::I8 => IntegerTypes::I8,
            integer::IntegerTypes::I16 => IntegerTypes::I16,
            integer::IntegerTypes::I32 => IntegerTypes::I32,
            integer::IntegerTypes::I64 => IntegerTypes::I64,
            integer::IntegerTypes::I128 => panic!("NOT SUPPORTED"),
            integer::IntegerTypes::ISize => IntegerTypes::ISize,
            integer::IntegerTypes::U8 => IntegerTypes::U8,
            integer::IntegerTypes::U16 => IntegerTypes::U16,
            integer::IntegerTypes::U32 => IntegerTypes::U32,
            integer::IntegerTypes::U64 => IntegerTypes::U64,
            integer::IntegerTypes::U128 => panic!("NOT SUPPORTED"),
            integer::IntegerTypes::USize => IntegerTypes::USize,
        },
    }
}
