<<<<<<< HEAD
<<<<<<< HEAD
use alloc::format;
use alloc::string::String;
use enum_as_inner::EnumAsInner;
=======
<<<<<<< HEAD
use alloc::format;
use alloc::string::String;
use enum_as_inner::EnumAsInner;
=======
use alloc::string::String;
use alloc::format;
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
use serde::Serialize;
=======
use alloc::format;
use alloc::string::String;
>>>>>>> FFI
use enum_as_inner::EnumAsInner;
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

<<<<<<< HEAD
<<<<<<< HEAD
=======
<<<<<<< HEAD
=======

>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
=======
>>>>>>> FFI
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
<<<<<<< HEAD
<<<<<<< HEAD
        let mut q: String = format!("{:?}", self);
=======
<<<<<<< HEAD
        let mut q: String = format!("{:?}", self);
=======
        let mut q: String = format!("{:?}",self);
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
=======
        let mut q: String = format!("{:?}", self);
>>>>>>> FFI
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
    pub rtype: NumberTypes,
    pub complete: bool,
}
