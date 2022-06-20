use alloc::vec;
use alloc::vec::Vec;

use crate::{definite::types::Types, defs::PlatformArchitecture};

/// TypeId
/// TypeId is a unique identifier for a type.
/// ## ID list
/// * `0`: `void`
/// * `1`: `null`
/// * `2`: `bool`
/// * `3`: `char`
/// * `4`: `byte`
/// * `5`: `int`
/// * `6`: `float`
/// * `7`: `double`
/// * `8`: `string`
/// * `9`: `array` This type is generic, all data lives in heap
/// * `10`: `vector` This type is generic, all data lives in heap
/// * `11`: `collective` This type is generic, all data lives in heap
/// ## Fields
/// * `id`: The ID of the type.
/// * `size`: The size of the type.
pub struct TypeId {
    pub id: usize,
    pub size: usize,
}

/// RawType
/// This is the representation of a type in the language.
/// ## Fields
/// * `type_id`: The ID of the type.
///  
pub struct RawType {
    pub type_id: TypeId,
    pub data: Vec<u8>, //This is platfform dependent
}

impl RawType {
    pub fn from(from: &Types, platform: PlatformArchitecture) -> RawType {
        let type_id = match &from {
            Types::Void => TypeId { id: 0, size: 0 },
            Types::Null => TypeId { id: 1, size: 0 },
            Types::Bool(_) => TypeId { id: 2, size: 1 },
            Types::Char(_) => TypeId { id: 3, size: 1 },
            Types::Byte(_) => TypeId { id: 4, size: 1 },
            Types::Integer(_) => TypeId {
                id: 5,
                size: platform.usize_len(),
            },
            Types::Float(_) => TypeId {
                id: 6,
                size: platform.usize_len(),
            },
            Types::Double(_) => TypeId {
                id: 7,
                size: platform.usize_len(),
            },
            Types::String(string) => TypeId {
                id: 8,
                size: string.value.len(),
            },
            Types::Array(array) => TypeId {
                id: 9,
                size: array.collective.len(),
            },
            Types::Vector(vector) => TypeId {
                id: 10,
                size: vector.collective.len(),
            },
            Types::Collective(collective) => TypeId {
                id: 11,
                size: collective.entries.len(),
            },
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
                Types::Char(char) => todo!(),
                _ => unreachable!("Non static"),
            },
        }
    }
}
