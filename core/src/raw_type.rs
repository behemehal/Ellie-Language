use core::fmt::{Display, Error, Formatter};
use alloc::vec;
use alloc::vec::Vec;
use crate::{definite::types::Types, defs::PlatformArchitecture};

#[derive(Clone, Debug)]
/// TypeId
/// TypeId is a unique identifier for a type.
/// ## ID list
/// * `1`: `integer`
/// * `2`: `float`
/// * `3`: `double`
/// * `4`: `byte`
/// * `5`: `bool`
/// * `6`: `string`
/// * `7`: `char`
/// * `8`: `void`
/// * `9`: `array`
/// * `10`: `null`
/// ## Fields
/// * `id`: The ID of the type.
/// * `size`: The size of the type.
pub struct TypeId {
    pub id: u8,
    pub size: usize,
}

impl PartialEq for TypeId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Display for TypeId {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self.id {
            1 => write!(f, "Integer"),
            2 => write!(f, "Float"),
            3 => write!(f, "Double"),
            4 => write!(f, "Byte"),
            5 => write!(f, "Bool"),
            6 => write!(f, "String"),
            7 => write!(f, "Char"),
            8 => write!(f, "Void"),
            9 => write!(f, "Array"),
            10 => write!(f, "Null"),
            _ => panic!("Unexpected type_id"),
        }
    }
}

impl TypeId {
    pub fn from(id: u8, size: usize) -> Self {
        Self { id, size }
    }
}

#[derive(Clone, Debug)]
/// RawType
/// This is the representation of a type in the language.
/// ## Fields
/// * `type_id`: The ID of the type.
pub struct RawType {
    pub type_id: TypeId,
    pub data: Vec<u8>, //This is platfform dependent
}

impl PartialEq for RawType {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id && self.data == other.data
    }
}

impl RawType {
    pub fn void() -> RawType {
        RawType {
            type_id: TypeId { id: 8, size: 0 },
            data: vec![],
        }
    }

    pub fn bool(boolity: bool) -> RawType {
        RawType {
            type_id: TypeId { id: 8, size: 0 },
            data: vec![if boolity { 1 } else { 0 }],
        }
    }

    pub fn integer(data: Vec<u8>) -> RawType {
        RawType {
            type_id: TypeId { id: 1, size: 0 },
            data,
        }
    }

    pub fn string(data: Vec<u8>) -> RawType {
        RawType {
            type_id: TypeId {
                id: 6,
                size: data.len(),
            },
            data,
        }
    }

    pub fn is_bool(&self) -> bool {
        self.type_id.id == 5
    }

    pub fn from(from: &Types, platform: PlatformArchitecture) -> RawType {
        let type_id = match &from {
            Types::Integer(_) => TypeId {
                id: 1,
                size: platform.usize_len() as usize,
            },
            Types::Float(_) => TypeId {
                id: 2,
                size: platform.usize_len() as usize,
            },
            Types::Double(_) => TypeId {
                id: 3,
                size: platform.usize_len() as usize,
            },
            Types::Byte(_) => TypeId { id: 4, size: 1 },
            Types::Bool(_) => TypeId { id: 5, size: 1 },
            Types::String(e) => TypeId {
                id: 6,
                size: e.value.len(),
            },
            Types::Char(_) => TypeId { id: 7, size: 4 },
            Types::Void => TypeId { id: 8, size: 0 },
            Types::Array(e) => TypeId {
                id: 9,
                size: e.collective.len(),
            },
            Types::Null => TypeId { id: 10, size: 0 },
            _ => unreachable!("Non static"),
        };

        RawType {
            type_id,
            data: match &from {
                Types::Void => Vec::new(),
                Types::Null => Vec::new(),
                Types::Bool(bool) => vec![bool.value as u8],
                Types::Byte(byte) => vec![byte.value as u8],
                Types::Integer(integer) => integer.value.to_le_bytes().to_vec(),
                Types::Float(float) => float.value.to_be_bytes().to_vec(),
                Types::Double(double) => double.value.to_be_bytes().to_vec(),
                Types::Char(char) => vec![char.value as u8],
                Types::String(string) => string.value.clone().as_bytes().to_vec(),
                Types::Array(array) => array.collective.len().to_le_bytes().to_vec(),
                _ => unreachable!("Non static"),
            },
        }
    }
}
