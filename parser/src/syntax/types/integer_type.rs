use alloc::fmt::Debug;
use alloc::format;
use alloc::string::String;
use core::any::Any;
use core::any::TypeId;
use enum_as_inner::EnumAsInner;
use serde::Serialize;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize)]
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
    Usize,
}

impl Default for IntegerTypes {
    fn default() -> Self {
        IntegerTypes::I8
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, EnumAsInner)]
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
            IntegerSize::U8(e) => {
                e < raw as u8
            }
            IntegerSize::U16(e) => {
                e < raw as u16
            }
            IntegerSize::U32(e) => {
                e < raw as u32
            }
            IntegerSize::U64(e) => {
                e < raw as u64
            }
            IntegerSize::U128(e) => {
                e < raw as u128
            }
            IntegerSize::Usize(e) => {
                e < raw as usize
            }
            IntegerSize::I8(e) => {
                e < raw as i8
            }
            IntegerSize::I16(e) => {
                e < raw as i16
            }
            IntegerSize::I32(e) => {
                e < raw as i32
            }
            IntegerSize::I64(e) => {
                e < raw as i64
            }
            IntegerSize::I128(e) => {
                e < raw as i128
            }
            IntegerSize::Isize(e) => {
                e < raw as isize
            }
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

#[derive(PartialEq, Default, Debug, Clone, Serialize)]
pub struct IntegerType {
    pub value: IntegerSize,
    pub raw: String,
    pub rtype: IntegerTypes,
    pub complete: bool,
}

impl IntegerType {
    pub fn build<T: Any>(raw: T) -> IntegerType {
        if TypeId::of::<T>() == TypeId::of::<i8>() {
            IntegerType {
                value: IntegerSize::I8(*(&raw as &dyn Any).downcast_ref::<i8>().unwrap()),
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else if TypeId::of::<T>() == TypeId::of::<i16>() {
            IntegerType {
                value: IntegerSize::I16(*(&raw as &dyn Any).downcast_ref::<i16>().unwrap()),
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else if TypeId::of::<T>() == TypeId::of::<i32>() {
            IntegerType {
                value: IntegerSize::I32(*(&raw as &dyn Any).downcast_ref::<i32>().unwrap()),
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else if TypeId::of::<T>() == TypeId::of::<i64>() {
            IntegerType {
                value: IntegerSize::I64(*(&raw as &dyn Any).downcast_ref::<i64>().unwrap()),
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else if TypeId::of::<T>() == TypeId::of::<i128>() {
            IntegerType {
                value: IntegerSize::I128(*(&raw as &dyn Any).downcast_ref::<i128>().unwrap()),
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else if TypeId::of::<T>() == TypeId::of::<isize>() {
            IntegerType {
                value: IntegerSize::Isize(*(&raw as &dyn Any).downcast_ref::<isize>().unwrap()),
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else if TypeId::of::<T>() == TypeId::of::<u8>() {
            IntegerType {
                value: IntegerSize::U8(*(&raw as &dyn Any).downcast_ref::<u8>().unwrap()),
                raw:(*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else if TypeId::of::<T>() == TypeId::of::<u16>() {
            IntegerType {
                value: IntegerSize::U16(*(&raw as &dyn Any).downcast_ref::<u16>().unwrap()),
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else if TypeId::of::<T>() == TypeId::of::<u32>() {
            IntegerType {
                value: IntegerSize::U32(*(&raw as &dyn Any).downcast_ref::<u32>().unwrap()),
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else if TypeId::of::<T>() == TypeId::of::<u64>() {
            IntegerType {
                value: IntegerSize::U64(*(&raw as &dyn Any).downcast_ref::<u64>().unwrap()),
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else if TypeId::of::<T>() == TypeId::of::<u128>() {
            IntegerType {
                value: IntegerSize::U128(*(&raw as &dyn Any).downcast_ref::<u128>().unwrap()),
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else if TypeId::of::<T>() == TypeId::of::<usize>() {
            IntegerType {
                value: IntegerSize::Usize(*(&raw as &dyn Any).downcast_ref::<usize>().unwrap()),
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else {
            IntegerType {
                value: IntegerSize::I8(*(&raw as &dyn Any).downcast_ref::<i8>().unwrap()),
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        }
    }
}
