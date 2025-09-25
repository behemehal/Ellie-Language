use core::mem;

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Copy, Clone, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum TypeId {
    Int,
    Float,
    Double,
    Byte,
    Bool,
    Char,
    Reference,
    Array,
}

pub const TYPE_SIZE: usize = mem::size_of::<usize>() * 2 + 1;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RawType {
    pub type_id: TypeId,
    pub size: usize,
    pub data: [u8; 8],
}

impl RawType {
    pub fn to_bytes(&self) -> [u8; TYPE_SIZE] {
        let mut buf = [0_u8; TYPE_SIZE];
        buf[0] = self.type_id as u8;

        let mut size_le_bytes = [0_u8; 8];
        size_le_bytes.copy_from_slice(&self.size.to_le_bytes()[..mem::size_of::<usize>()]);
        buf[1..9].copy_from_slice(&size_le_bytes);

        let mut data_le_bytes = [0_u8; 8];
        data_le_bytes.copy_from_slice(&self.data[..mem::size_of::<usize>()]);
        buf[9..17].copy_from_slice(&data_le_bytes);

        buf
    }

    pub fn from_bytes(bytes: &[u8; TYPE_SIZE]) -> Self {
        let type_id = TypeId::try_from(bytes[0]).unwrap_or(TypeId::Int);
        let size = usize::from_le_bytes(bytes[1..9].try_into().unwrap());
        let data = bytes[9..17].try_into().unwrap();

        RawType {
            type_id,
            size,
            data,
        }
    }
}

//Implement From<usize> for RawType
impl From<usize> for RawType {
    fn from(value: usize) -> Self {
        let mut data = [0_u8; 8];
        data.copy_from_slice(&value.to_le_bytes()[..mem::size_of::<usize>()]);
        RawType {
            type_id: TypeId::Int,
            size: mem::size_of::<usize>(),
            data,
        }
    }
}

impl From<u8> for RawType {
    fn from(value: u8) -> Self {
        let mut data = [0_u8; 8];
        data.copy_from_slice(&value.to_le_bytes()[..mem::size_of::<u8>()]);
        RawType {
            type_id: TypeId::Byte,
            size: mem::size_of::<u8>(),
            data,
        }
    }
}

impl From<isize> for RawType {
    fn from(value: isize) -> Self {
        let mut data = [0_u8; 8];
        data.copy_from_slice(&value.to_le_bytes()[..mem::size_of::<isize>()]);
        RawType {
            type_id: TypeId::Int,
            size: mem::size_of::<isize>(),
            data,
        }
    }
}

impl From<i32> for RawType {
    fn from(value: i32) -> Self {
        let mut data = [0_u8; 8];
        data[0..mem::size_of::<i32>()].copy_from_slice(&value.to_le_bytes()[..mem::size_of::<i32>()]);
        RawType {
            type_id: TypeId::Int,
            size: mem::size_of::<i32>(),
            data,
        }
    }
}

impl Into<usize> for RawType {
    fn into(self) -> usize {
        usize::from_le_bytes(self.data[0..mem::size_of::<usize>()].try_into().unwrap())
    }
}

impl Into<u8> for RawType {
    fn into(self) -> u8 {
        u8::from_le_bytes(self.data[0..mem::size_of::<u8>()].try_into().unwrap())
    }
}

impl Into<isize> for RawType {
    fn into(self) -> isize {
        isize::from_le_bytes(self.data[0..mem::size_of::<isize>()].try_into().unwrap())
    }
}