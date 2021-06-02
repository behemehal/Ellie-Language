use crate::syntax::types;
use libc::c_char;

#[repr(C)]
pub struct FunctionType {
    pub complete: bool,
    pub params: *const DefinerCollecting,
    pub returning: Box<DefinerCollecting>,
    pub return_typed: bool,
    pub return_keyword: i8,
    pub parameter_collected: bool,
    pub bracket_inserted: bool,
    pub at_comma: bool,
}

#[repr(C)]
pub struct CloakType {
    pub complete: bool,
    pub rtype: *const DefinerCollecting,
    pub bracket_inserted: bool,
    pub at_comma: bool,
}

#[repr(C)]
pub struct ArrayType {
    pub complete: bool,
    pub rtype: Box<DefinerCollecting>,
    pub bracket_inserted: bool,
    pub len: types::Types,
    pub at_comma: bool,
    pub typed: bool,
}

#[repr(C)]
pub struct GrowableArrayType {
    pub complete: bool,
    pub rtype: Box<DefinerCollecting>,
    pub bracket_inserted: bool,
}

#[repr(C)]
pub struct GenericType {
    pub rtype: *const c_char,
}

#[repr(C)]
pub enum DefinerCollecting {
    Array(ArrayType),
    GrowableArray(GrowableArrayType),
    Generic(GenericType),
    Function(FunctionType),
    Cloak(CloakType),
    Dynamic,
}
