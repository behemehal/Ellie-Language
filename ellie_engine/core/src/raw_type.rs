#[cfg(feature = "compiler_utils")]
use crate::definite::types::Types;
use crate::defs::PlatformArchitecture;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::{string::String, vec};
use core::fmt::{Display, Error, Formatter};

pub enum TypeIds {
    Integer,
    Float,
    Double,
    Byte,
    Bool,
    String,
    Char,
    Void,
    Array,
    Null,
    Class,
    Function,
    StackReference,
    HeapReference,
}

#[derive(Clone, Debug, Copy)]
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
/// * `11`: `class`
/// * `12`: `function`
/// * `13`: `stack_reference`
/// * `14`: `heap_reference`
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
            11 => write!(f, "Class"),
            12 => write!(f, "Function"),
            13 => write!(f, "StackReference"),
            14 => write!(f, "HeapReference"),
            _ => panic!("Unexpected type_id"),
        }
    }
}

impl TypeId {
    pub fn to_bytes(&self) -> [u8; 9] {
        [
            self.id,
            self.size as u8,
            (self.size >> 8) as u8,
            (self.size >> 16) as u8,
            (self.size >> 24) as u8,
            (self.size >> 32) as u8,
            (self.size >> 40) as u8,
            (self.size >> 48) as u8,
            (self.size >> 56) as u8,
        ]
    }

    pub fn from_bytes(bytes: &[u8; 9]) -> Self {
        Self {
            id: bytes[0],
            size: usize::from_le_bytes(bytes[1..].try_into().unwrap()),
        }
    }

    pub fn id(&self) -> TypeIds {
        match self.id {
            1 => TypeIds::Integer,
            2 => TypeIds::Float,
            3 => TypeIds::Double,
            4 => TypeIds::Byte,
            5 => TypeIds::Bool,
            6 => TypeIds::String,
            7 => TypeIds::Char,
            8 => TypeIds::Void,
            9 => TypeIds::Array,
            10 => TypeIds::Null,
            11 => TypeIds::Class,
            12 => TypeIds::Function,
            13 => TypeIds::StackReference,
            14 => TypeIds::HeapReference,
            _ => panic!("Unexpected type_id"),
        }
    }

    pub fn is_stack_storable(&self) -> bool {
        self.id != 6 || self.id != 9
    }

    pub fn is_int(&self) -> bool {
        self.id == 1
    }

    pub fn is_float(&self) -> bool {
        self.id == 2
    }

    pub fn is_double(&self) -> bool {
        self.id == 3
    }

    pub fn is_byte(&self) -> bool {
        self.id == 4
    }

    pub fn is_bool(&self) -> bool {
        self.id == 5
    }

    pub fn is_string(&self) -> bool {
        self.id == 6
    }

    pub fn is_char(&self) -> bool {
        self.id == 7
    }

    pub fn is_void(&self) -> bool {
        self.id == 8
    }

    pub fn is_array(&self) -> bool {
        self.id == 9
    }

    pub fn is_null(&self) -> bool {
        self.id == 10
    }

    pub fn is_class(&self) -> bool {
        self.id == 11
    }

    pub fn is_function(&self) -> bool {
        self.id == 12
    }

    pub fn is_stack_reference(&self) -> bool {
        self.id == 13
    }

    pub fn is_heap_reference(&self) -> bool {
        self.id == 14
    }

    pub fn integer() -> Self {
        Self { id: 1, size: 8 }
    }

    pub fn float() -> Self {
        Self { id: 2, size: 4 }
    }

    pub fn double() -> Self {
        Self { id: 3, size: 8 }
    }

    pub fn byte() -> Self {
        Self { id: 4, size: 1 }
    }

    pub fn bool() -> Self {
        Self { id: 5, size: 1 }
    }

    pub fn string(size: usize) -> Self {
        Self { id: 6, size }
    }

    pub fn char() -> Self {
        Self { id: 7, size: 1 }
    }

    pub fn void() -> Self {
        Self { id: 8, size: 0 }
    }

    pub fn array(size: usize) -> Self {
        Self { id: 9, size }
    }

    pub fn null() -> Self {
        Self { id: 10, size: 0 }
    }

    pub fn class(size: usize) -> Self {
        Self { id: 11, size }
    }

    pub fn function() -> Self {
        Self { id: 12, size: 8 }
    }

    pub fn stack_reference() -> Self {
        Self { id: 13, size: 8 }
    }

    pub fn heap_reference() -> Self {
        Self { id: 14, size: 8 }
    }

    pub fn from(id: u8, size: usize) -> Self {
        Self { id, size }
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    pub hash: usize,
}

pub struct MutatableRawType<'a> {
    pub data: &'a mut Vec<u8>,
}

impl MutatableRawType<'_> {
    pub fn get_type_id(&self) -> TypeId {
        //TODO: This is not platform safe 0x00
        TypeId::from_bytes(self.data[0..9].try_into().unwrap())
    }

    pub fn set_type_id(&mut self, type_id: TypeId) {
        //TODO: This is not platform safe 0x00
        self.data[0..9].copy_from_slice(&type_id.to_bytes());
    }

    pub fn get_raw_type(&self) -> RawType {
        RawType::from_bytes(&self.data)
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        //TODO: This is not platform safe 0x00
        self.data[9..].copy_from_slice(&data);
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

impl From<RawType> for String {
    fn from(c: RawType) -> Self {
        match c.type_id {
            TypeId { id: 6, size: 1 } => String::from_utf8(c.data).unwrap(),
            _ => panic!("Unexpected type_id"),
        }
    }
}

impl From<RawType> for char {
    fn from(c: RawType) -> Self {
        match c.type_id {
            TypeId { id: 7, size: 1 } => char::from(c.data[0]),
            _ => panic!("Unexpected type_id"),
        }
    }
}

impl From<RawType> for i8 {
    fn from(c: RawType) -> Self {
        match c.type_id {
            TypeId { id: 4, size: 1 } => c.data[0] as i8,
            _ => panic!("Unexpected type_id"),
        }
    }
}

impl From<RawType> for isize {
    fn from(c: RawType) -> Self {
        match c.type_id {
            TypeId { id: 1, size: 8 } => isize::from_le_bytes(c.data.try_into().unwrap()),
            _ => panic!("Unexpected type_id"),
        }
    }
}

impl From<RawType> for usize {
    fn from(c: RawType) -> Self {
        match c.type_id {
            TypeId { id: 1, size: 8 } => usize::from_le_bytes(c.data.try_into().unwrap()),
            _ => panic!("Unexpected type_id"),
        }
    }
}

impl From<RawType> for f32 {
    fn from(c: RawType) -> Self {
        match c.type_id {
            TypeId { id: 3, size: 4 } => f32::from_le_bytes(c.data.try_into().unwrap()),
            _ => panic!("Unexpected type_id"),
        }
    }
}

impl From<RawType> for f64 {
    fn from(c: RawType) -> Self {
        match c.type_id {
            TypeId { id: 3, size: 8 } => f64::from_le_bytes(c.data.try_into().unwrap()),
            _ => panic!("Unexpected type_id"),
        }
    }
}

impl Into<RawType> for String {
    fn into(self) -> RawType {
        RawType {
            type_id: TypeId::from(6, self.len()),
            data: self.into_bytes(),
        }
    }
}

impl Into<RawType> for &str {
    fn into(self) -> RawType {
        RawType {
            type_id: TypeId::from(6, self.len()),
            data: self.to_string().into_bytes(),
        }
    }
}

impl Into<RawType> for char {
    fn into(self) -> RawType {
        RawType {
            type_id: TypeId::from(7, 1),
            data: vec![self as u8],
        }
    }
}

impl Into<RawType> for i8 {
    fn into(self) -> RawType {
        RawType {
            type_id: TypeId::from(4, 1),
            data: vec![self as u8],
        }
    }
}

impl Into<RawType> for () {
    fn into(self) -> RawType {
        RawType {
            type_id: TypeId::from(8, 0),
            data: vec![],
        }
    }
}

impl Into<RawType> for bool {
    fn into(self) -> RawType {
        RawType {
            type_id: TypeId::from(5, 1),
            data: vec![self as u8],
        }
    }
}

impl Into<RawType> for f32 {
    fn into(self) -> RawType {
        RawType {
            type_id: TypeId::from(2, 4),
            data: self.to_le_bytes().to_vec(),
        }
    }
}

impl Into<RawType> for f64 {
    fn into(self) -> RawType {
        RawType {
            type_id: TypeId::from(3, 8),
            data: self.to_le_bytes().to_vec(),
        }
    }
}

impl Into<RawType> for isize {
    fn into(self) -> RawType {
        RawType {
            type_id: TypeId::from(1, 8),
            data: self.to_le_bytes().to_vec(),
        }
    }
}

impl Into<RawType> for u8 {
    fn into(self) -> RawType {
        RawType {
            type_id: TypeId::from(1, 1),
            data: vec![self],
        }
    }
}

impl Into<RawType> for usize {
    fn into(self) -> RawType {
        RawType {
            type_id: TypeId::from(1, 8),
            data: self.to_le_bytes().to_vec(),
        }
    }
}

impl PartialEq for RawType {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id && self.data == other.data
    }
}

impl RawType {
    pub fn to_bytes(&self) -> Vec<u8> {
        let type_id = self.type_id.to_bytes();
        let mut bytes = Vec::from(type_id);
        bytes.extend(self.data.clone());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> RawType {
        let type_id = TypeId::from_bytes(&bytes[..9].try_into().unwrap());
        let data = bytes[9..].to_vec();
        RawType { type_id, data }
    }

    pub fn to_register_raw(&self) -> Result<StaticRawType, u8> {
        match self.type_id.id {
            0..=5 | 7 | 8 | 10 | 13 | 14 => Ok(StaticRawType {
                type_id: TypeId {
                    id: self.type_id.id,
                    size: self.type_id.size,
                },
                data: {
                    if self.data.len() < 8 {
                        let mut data = [0; 8];
                        data[..self.data.len()].copy_from_slice(&self.data);
                        data
                    } else {
                        self.data.clone().try_into().unwrap()
                    }
                },
            }),
            id => Err(id),
        }
    }

    pub fn to_int(&self) -> isize {
        isize::from_le_bytes(self.data.clone().try_into().unwrap())
    }

    pub fn to_float(&self) -> f32 {
        f32::from_le_bytes(self.data.clone().try_into().unwrap())
    }

    pub fn to_double(&self) -> f64 {
        f64::from_le_bytes(self.data.clone().try_into().unwrap())
    }

    pub fn to_byte(&self) -> u8 {
        u8::from_le_bytes(self.data.clone().try_into().unwrap())
    }

    pub fn to_bool(&self) -> bool {
        self.data[0] == 1
    }

    pub fn to_string(&self) -> String {
        match String::from_utf8(self.data.clone()) {
            Ok(e) => e,
            Err(_) => "Invalid UTF-8 sequence".to_string(),
        }
    }

    pub fn to_function(&self, arch: PlatformArchitecture) -> Function {
        let data = self
            .data
            .chunks(arch.usize_len() as usize)
            .collect::<Vec<_>>();
        let hash = usize::from_le_bytes(data[0].try_into().unwrap());

        Function { hash }
    }

    pub fn to_char(&self) -> char {
        char::from_u32(u32::from_le_bytes(self.data.clone().try_into().unwrap())).unwrap()
    }

    pub fn integer(data: Vec<u8>) -> RawType {
        RawType {
            type_id: TypeId { id: 1, size: 8 },
            data,
        }
    }

    pub fn float(data: Vec<u8>) -> RawType {
        RawType {
            type_id: TypeId { id: 2, size: 4 },
            data,
        }
    }

    pub fn double(data: Vec<u8>) -> RawType {
        RawType {
            type_id: TypeId { id: 3, size: 8 },
            data,
        }
    }

    pub fn byte(data: u8) -> RawType {
        RawType {
            type_id: TypeId { id: 4, size: 1 },
            data: vec![data],
        }
    }

    pub fn bool(boolity: bool) -> RawType {
        RawType {
            type_id: TypeId { id: 5, size: 1 },
            data: vec![if boolity { 1 } else { 0 }],
        }
    }

    pub fn string(data: Vec<char>) -> RawType {
        let mut char_arr = Vec::new();
        for char in data {
            char_arr.extend((char as u32).to_le_bytes().to_vec());
        }
        RawType {
            type_id: TypeId {
                id: 6,
                size: char_arr.len(),
            },
            data: char_arr,
        }
    }

    pub fn generate_string(data: String) -> RawType {
        let chars = data.chars().collect::<Vec<_>>();
        let mut data = Vec::new();
        for char in chars {
            data.extend((char as u32).to_le_bytes().to_vec());
        }
        RawType {
            type_id: TypeId {
                id: 6,
                size: data.len(),
            },
            data,
        }
    }

    pub fn char(data: Vec<u8>) -> RawType {
        RawType {
            type_id: TypeId {
                id: 7,
                size: data.len(),
            },
            data,
        }
    }

    pub fn void() -> RawType {
        RawType {
            type_id: TypeId { id: 8, size: 0 },
            data: vec![],
        }
    }

    pub fn is_int(&self) -> bool {
        self.type_id.id == 1
    }

    pub fn is_float(&self) -> bool {
        self.type_id.id == 2
    }

    pub fn is_double(&self) -> bool {
        self.type_id.id == 3
    }

    pub fn is_byte(&self) -> bool {
        self.type_id.id == 4
    }

    pub fn is_bool(&self) -> bool {
        self.type_id.id == 5
    }

    pub fn is_string(&self) -> bool {
        self.type_id.id == 6
    }

    #[cfg(feature = "compiler_utils")]
    pub fn from(from: &Types, platform: PlatformArchitecture) -> RawType {
        let type_id = match &from {
            Types::Integer(_) => TypeId {
                id: 1,
                size: platform.usize_len() as usize,
            },
            Types::Decimal(e) => TypeId {
                id: if e.is_double { 3 } else { 2 },
                size: if e.is_double { 8 } else { 4 },
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
                Types::Decimal(decimal) => match decimal.value {
                    crate::definite::types::decimal::DecimalTypeEnum::Float(e) => {
                        e.to_le_bytes().to_vec()
                    }
                    crate::definite::types::decimal::DecimalTypeEnum::Double(e) => {
                        e.to_le_bytes().to_vec()
                    }
                },
                Types::Char(char) => vec![char.value as u8],
                Types::String(string) => string.value.clone().as_bytes().to_vec(),
                Types::Array(array) => array.collective.len().to_le_bytes().to_vec(),
                _ => unreachable!("Non static"),
            },
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct StaticRawType {
    pub type_id: TypeId,
    pub data: [u8; 8],
}

impl StaticRawType {
    pub fn to_raw(&self) -> RawType {
        RawType {
            type_id: TypeId {
                id: self.type_id.id,
                size: self.type_id.size,
            },
            data: self.data.to_vec(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let type_id = self.type_id.to_bytes();
        let mut bytes = Vec::from(type_id);
        bytes.extend(self.data.clone());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> StaticRawType {
        let type_id = TypeId::from_bytes(&bytes[..9].try_into().unwrap());
        let data: [u8; 8] = bytes[9..].try_into().unwrap();
        StaticRawType { type_id, data }
    }

    pub fn to_int(&self) -> isize {
        isize::from_le_bytes(self.data)
    }

    pub fn to_float(&self) -> f32 {
        f32::from_le_bytes(self.data[0..4].try_into().unwrap())
    }

    pub fn to_double(&self) -> f64 {
        f64::from_le_bytes(self.data)
    }

    pub fn to_byte(&self) -> u8 {
        self.data[0]
    }

    pub fn to_bool(&self) -> bool {
        self.data[0] == 1
    }

    pub fn to_char(&self) -> char {
        char::from_u32(u32::from_le_bytes(self.data[0..4].try_into().unwrap())).unwrap()
    }

    pub fn integer(data: Vec<u8>) -> StaticRawType {
        StaticRawType {
            type_id: TypeId { id: 1, size: 8 },
            data: data.try_into().unwrap(),
        }
    }

    pub fn function(data: Vec<u8>) -> StaticRawType {
        StaticRawType {
            type_id: TypeId { id: 12, size: 8 },
            data: data.try_into().unwrap(),
        }
    }

    pub fn float(data: Vec<u8>) -> StaticRawType {
        StaticRawType {
            type_id: TypeId { id: 2, size: 4 },
            data: data.try_into().unwrap(),
        }
    }

    pub fn double(data: Vec<u8>) -> StaticRawType {
        StaticRawType {
            type_id: TypeId { id: 3, size: 8 },
            data: data.try_into().unwrap(),
        }
    }

    pub fn byte(data: u8) -> StaticRawType {
        StaticRawType {
            type_id: TypeId { id: 1, size: 1 },
            data: [data; 8],
        }
    }

    pub fn bool(boolity: bool) -> StaticRawType {
        StaticRawType {
            type_id: TypeId { id: 5, size: 1 },
            data: [if boolity { 1 } else { 0 }; 8],
        }
    }

    pub fn char(data: Vec<u8>) -> StaticRawType {
        StaticRawType {
            type_id: TypeId { id: 7, size: 4 },
            data: data.try_into().unwrap(),
        }
    }

    pub fn void() -> StaticRawType {
        StaticRawType {
            type_id: TypeId { id: 8, size: 0 },
            data: [0; 8],
        }
    }

    pub fn stack_reference(data: [u8; 8]) -> StaticRawType {
        StaticRawType {
            type_id: TypeId { id: 13, size: 8 },
            data,
        }
    }

    pub fn heap_reference(data: [u8; 8]) -> StaticRawType {
        StaticRawType {
            type_id: TypeId { id: 14, size: 8 },
            data,
        }
    }
}
