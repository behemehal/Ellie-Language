use alloc::fmt::Debug;
use alloc::format;
use alloc::string::String;
use core::any::Any;
use core::any::TypeId;
use ellie_core::definite;
use enum_as_inner::EnumAsInner;
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

impl Default for IntegerTypes {
    fn default() -> Self {
        IntegerTypes::I8
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, EnumAsInner, Deserialize)]
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

impl IntegerSize {
    pub fn greater_than(&self, raw: isize) -> bool {
        match *self {
            IntegerSize::U8(e) => e < raw as u8,
            IntegerSize::U16(e) => e < raw as u16,
            IntegerSize::U32(e) => e < raw as u32,
            IntegerSize::U64(e) => e < raw as u64,
            IntegerSize::U128(e) => e < raw as u128,
            IntegerSize::Usize(e) => e < raw as usize,
            IntegerSize::I8(e) => e < raw as i8,
            IntegerSize::I16(e) => e < raw as i16,
            IntegerSize::I32(e) => e < raw as i32,
            IntegerSize::I64(e) => e < raw as i64,
            IntegerSize::I128(e) => e < raw as i128,
            IntegerSize::Isize(e) => e < raw as isize,
        }
    }

    pub fn get_type(&self) -> String {
        let mut q: String = format!("{:?}", self);
        let bracket_offset = q.find('(').unwrap_or_else(|| q.len());
        q.replace_range(bracket_offset.., "");
        q
    }

    pub fn get_val(&self) -> String {
        let mut q: String = format!("{:?}", self);
        let bracket_open_offset = q.find('(').unwrap_or_else(|| q.len());
        q.replace_range(..bracket_open_offset + 1, "");
        q.replace(")", "")
    }
}

impl Default for IntegerSize {
    fn default() -> Self {
        IntegerSize::I64(0)
    }
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct IntegerType {
    pub value: IntegerSize,
    pub rtype: IntegerTypes,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct IntegerTypeCollector {
    pub data: IntegerType,
    pub raw: String,
    pub complete: bool,
}

impl IntegerTypeCollector {
    pub fn to_definite(self) -> definite::types::integer::IntegerType {
        definite::types::integer::IntegerType {
            value: match self.data.value {
                IntegerSize::U8(e) => definite::types::integer::IntegerSize::U8(e),
                IntegerSize::U16(e) => definite::types::integer::IntegerSize::U16(e),
                IntegerSize::U32(e) => definite::types::integer::IntegerSize::U32(e),
                IntegerSize::U64(e) => definite::types::integer::IntegerSize::U64(e),
                IntegerSize::U128(e) => definite::types::integer::IntegerSize::U128(e),
                IntegerSize::Usize(e) => definite::types::integer::IntegerSize::Usize(e),
                IntegerSize::I8(e) => definite::types::integer::IntegerSize::I8(e),
                IntegerSize::I16(e) => definite::types::integer::IntegerSize::I16(e),
                IntegerSize::I32(e) => definite::types::integer::IntegerSize::I32(e),
                IntegerSize::I64(e) => definite::types::integer::IntegerSize::I64(e),
                IntegerSize::I128(e) => definite::types::integer::IntegerSize::I128(e),
                IntegerSize::Isize(e) => definite::types::integer::IntegerSize::Isize(e),
            },
            rtype: match self.data.rtype {
                IntegerTypes::U8 => definite::types::integer::IntegerTypes::U8,
                IntegerTypes::U16 => definite::types::integer::IntegerTypes::U16,
                IntegerTypes::U32 => definite::types::integer::IntegerTypes::U32,
                IntegerTypes::U64 => definite::types::integer::IntegerTypes::U64,
                IntegerTypes::U128 => definite::types::integer::IntegerTypes::U128,
                IntegerTypes::USize => definite::types::integer::IntegerTypes::USize,
                IntegerTypes::I8 => definite::types::integer::IntegerTypes::I8,
                IntegerTypes::I16 => definite::types::integer::IntegerTypes::I16,
                IntegerTypes::I32 => definite::types::integer::IntegerTypes::I32,
                IntegerTypes::I64 => definite::types::integer::IntegerTypes::I64,
                IntegerTypes::I128 => definite::types::integer::IntegerTypes::I128,
                IntegerTypes::ISize => definite::types::integer::IntegerTypes::ISize,
            },
        }
    }

    pub fn build<T: Any>(raw: T) -> IntegerTypeCollector {
        if TypeId::of::<T>() == TypeId::of::<i8>() {
            IntegerTypeCollector {
                data: IntegerType {
                    value: IntegerSize::I8(*(&raw as &dyn Any).downcast_ref::<i8>().unwrap()),
                    rtype: IntegerTypes::I8,
                },
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else if TypeId::of::<T>() == TypeId::of::<i16>() {
            IntegerTypeCollector {
                data: IntegerType {
                    value: IntegerSize::I16(*(&raw as &dyn Any).downcast_ref::<i16>().unwrap()),
                    rtype: IntegerTypes::I16,
                },
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else if TypeId::of::<T>() == TypeId::of::<i32>() {
            IntegerTypeCollector {
                data: IntegerType {
                    value: IntegerSize::I32(*(&raw as &dyn Any).downcast_ref::<i32>().unwrap()),
                    rtype: IntegerTypes::I32,
                },
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else if TypeId::of::<T>() == TypeId::of::<i64>() {
            IntegerTypeCollector {
                data: IntegerType {
                    value: IntegerSize::I64(*(&raw as &dyn Any).downcast_ref::<i64>().unwrap()),
                    rtype: IntegerTypes::I64,
                },
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else if TypeId::of::<T>() == TypeId::of::<i128>() {
            IntegerTypeCollector {
                data: IntegerType {
                    value: IntegerSize::I128(*(&raw as &dyn Any).downcast_ref::<i128>().unwrap()),
                    rtype: IntegerTypes::I128,
                },
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else if TypeId::of::<T>() == TypeId::of::<isize>() {
            IntegerTypeCollector {
                data: IntegerType {
                    value: IntegerSize::Isize(*(&raw as &dyn Any).downcast_ref::<isize>().unwrap()),
                    rtype: IntegerTypes::ISize,
                },
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else if TypeId::of::<T>() == TypeId::of::<u8>() {
            IntegerTypeCollector {
                data: IntegerType {
                    value: IntegerSize::U8(*(&raw as &dyn Any).downcast_ref::<u8>().unwrap()),
                    rtype: IntegerTypes::U8,
                },
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else if TypeId::of::<T>() == TypeId::of::<u16>() {
            IntegerTypeCollector {
                data: IntegerType {
                    value: IntegerSize::U16(*(&raw as &dyn Any).downcast_ref::<u16>().unwrap()),
                    rtype: IntegerTypes::U16,
                },
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else if TypeId::of::<T>() == TypeId::of::<u32>() {
            IntegerTypeCollector {
                data: IntegerType {
                    value: IntegerSize::U32(*(&raw as &dyn Any).downcast_ref::<u32>().unwrap()),
                    rtype: IntegerTypes::U32,
                },
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else if TypeId::of::<T>() == TypeId::of::<u64>() {
            IntegerTypeCollector {
                data: IntegerType {
                    value: IntegerSize::U64(*(&raw as &dyn Any).downcast_ref::<u64>().unwrap()),
                    rtype: IntegerTypes::U64,
                },
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else if TypeId::of::<T>() == TypeId::of::<u128>() {
            IntegerTypeCollector {
                data: IntegerType {
                    value: IntegerSize::U128(*(&raw as &dyn Any).downcast_ref::<u128>().unwrap()),
                    rtype: IntegerTypes::U128,
                },
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else if TypeId::of::<T>() == TypeId::of::<usize>() {
            IntegerTypeCollector {
                data: IntegerType {
                    value: IntegerSize::Usize(*(&raw as &dyn Any).downcast_ref::<usize>().unwrap()),
                    rtype: IntegerTypes::USize,
                },
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else {
            IntegerTypeCollector {
                data: IntegerType {
                    value: IntegerSize::I8(*(&raw as &dyn Any).downcast_ref::<i8>().unwrap()),
                    rtype: IntegerTypes::I8,
                },
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        }
    }
}
