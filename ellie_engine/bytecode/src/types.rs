use alloc::string::{String, ToString};
use ellie_core::defs::PlatformArchitecture;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Types {
    // 8 bit integer
    Integer,
    // 4 bit decimal
    Float,
    // 8 bit decimal
    Double,
    // 1 bit integer
    Byte,
    // 1 bit boolean
    Bool,
    // dynamic char array
    String(usize),
    // 4 bit char
    Char,
    // compile time sized array pointing to next addresses
    StaticArray(usize),
    // dynamic sized array
    Array(usize),
    // 0 bit void
    Void,
    // 0 bit null
    Null,
    // Pointer to class
    Class(usize),
    // Pointer to function
    Function,
    // Pointer to heap reference
    HeapReference,
    // Pointer to stack reference
    StackReference,
}

impl Types {
    //(type_id, size)
    // (1, platform_size) Integer
    pub fn code(&self, platform_size: PlatformArchitecture) -> (u8, usize) {
        match &self {
            Types::Integer => (1, platform_size.usize_len() as usize),
            Types::Float => (2, platform_size.usize_len() as usize),
            Types::Double => (3, platform_size.usize_len() as usize),
            Types::Byte => (4, 1),
            Types::Bool => (5, 1),
            Types::String(str_len) => (6, *str_len),
            Types::Char => (7, 4),
            Types::Void => (8, 0),
            Types::Array(array_len) => (9, *array_len),
            Types::Null => (10, 0),
            Types::Class(_) => (11, platform_size.usize_len() as usize),
            Types::Function => (12, platform_size.usize_len() as usize),
            Types::HeapReference => (13, platform_size.usize_len() as usize),
            Types::StackReference => (13, platform_size.usize_len() as usize),
            Types::StaticArray(array_size) => (14, *array_size),
        }
    }

    pub fn display(&self) -> String {
        match &self {
            Types::Integer => "int".to_string(),
            Types::Float => "float".to_string(),
            Types::Double => "double".to_string(),
            Types::Byte => "byte".to_string(),
            Types::Bool => "bool".to_string(),
            Types::String(str_len) => alloc::format!("string[{str_len}]"),
            Types::Char => alloc::format!("char"),
            Types::StaticArray(size) => alloc::format!("staticArray<{size}>"),
            Types::Array(len) => alloc::format!("array<{len} / platformSize>"),
            Types::Void => "void".to_string(),
            Types::Null => "null".to_string(),
            Types::Class(class_len) => {
                alloc::format!("class<{class_len}>")
            }
            Types::Function => {
                alloc::format!("function")
            }
            Types::HeapReference => "heapReference".to_string(),
            Types::StackReference => "stackReference".to_string(),
        }
    }
}
