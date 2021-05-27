use alloc::string::String;
use alloc::format;
use serde::Serialize;
use enum_as_inner::EnumAsInner;


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


#[derive(PartialEq, Debug, Clone, Copy, Serialize, EnumAsInner)]
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

/*

let mut s = String::from("α is alpha, β is beta");
let beta_offset = s.find('β').unwrap_or(s.len());

s.replace_range(..beta_offset, "Α is capital alpha; ");
assert_eq!(s, "Α is capital alpha; β is beta");

*/

impl NumberSize {
    pub fn get_type(&self) -> String {
        let mut q: String = format!("{:?}",self);
        let bracket_offset = q.find('(').unwrap_or_else(|| q.len());
        q.replace_range(bracket_offset.., "");
        q
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