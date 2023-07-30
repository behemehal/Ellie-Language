use alloc::vec::Vec;
use alloc::{string::String, vec};
use core::fmt::{Display, Error, Formatter};
use core::mem;
use ellie_core::defs::PlatformArchitecture;

const INTEGER_SIZE: usize = mem::size_of::<usize>();
const TYPE_SIZE: usize = INTEGER_SIZE + 1;

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
    StaticArray,
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
/// * `15`: `static_array`
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
            15 => write!(f, "StaticArray"),
            _ => panic!("Unexpected type_id"),
        }
    }
}

impl TypeId {
    pub fn to_bytes(&self) -> [u8; TYPE_SIZE] {
        let mut bytes = [0; TYPE_SIZE];
        let mut size_bytes = [0; INTEGER_SIZE];
        size_bytes.copy_from_slice(&self.size.to_le_bytes()[0..INTEGER_SIZE]);
        bytes[0] = self.id;
        bytes[1..].copy_from_slice(&size_bytes);
        bytes
    }

    pub fn from_bytes(bytes: &[u8; TYPE_SIZE]) -> Self {
        let mut usize_bytes = [0; mem::size_of::<usize>()];
        usize_bytes[0..INTEGER_SIZE].copy_from_slice(&bytes[1..]);
        Self {
            id: bytes[0],
            size: usize::from_le_bytes(usize_bytes),
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
            15 => TypeIds::StaticArray,
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

    pub fn is_static_array(&self) -> bool {
        self.id == 15
    }

    pub fn is_core_type(&self) -> bool {
        match self.id {
            1..=12 | 15 => true,
            _ => false,
        }
    }

    pub fn integer() -> Self {
        Self {
            id: 1,
            size: INTEGER_SIZE,
        }
    }

    pub fn float() -> Self {
        Self {
            id: 2,
            size: mem::size_of::<f64>(),
        }
    }

    pub fn double() -> Self {
        Self {
            id: 3,
            size: mem::size_of::<f32>(),
        }
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
        Self {
            id: 12,
            size: INTEGER_SIZE,
        }
    }

    pub fn stack_reference() -> Self {
        Self {
            id: 13,
            size: INTEGER_SIZE,
        }
    }

    pub fn heap_reference() -> Self {
        Self {
            id: 14,
            size: INTEGER_SIZE,
        }
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
        TypeId::from_bytes(self.data[0..TYPE_SIZE].try_into().unwrap())
    }

    pub fn set_type_id(&mut self, type_id: TypeId) {
        //TODO: This is not platform safe 0x00
        self.data[0..TYPE_SIZE].copy_from_slice(&type_id.to_bytes());
    }

    pub fn get_raw_type(&self) -> RawType {
        RawType::from_bytes(self.data)
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data[TYPE_SIZE..].copy_from_slice(&data);
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
    pub fn to_bytes(&self) -> Vec<u8> {
        let type_id = self.type_id.to_bytes();
        let mut bytes = Vec::from(type_id);
        bytes.extend(self.data.clone());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> RawType {
        let type_id = TypeId::from_bytes(&bytes[..TYPE_SIZE].try_into().unwrap());
        let data = bytes[TYPE_SIZE..].to_vec();
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

    pub fn to_uint(&self) -> usize {
        usize::from_le_bytes(self.data.clone().try_into().unwrap())
    }

    pub fn to_float(&self) -> f64 {
        f64::from_le_bytes(self.data.clone().try_into().unwrap())
    }

    pub fn to_double(&self) -> f32 {
        f32::from_le_bytes(self.data.clone().try_into().unwrap())
    }

    pub fn to_byte(&self) -> u8 {
        u8::from_le_bytes(self.data.clone().try_into().unwrap())
    }

    pub fn to_bool(&self) -> bool {
        self.data[0] == 1
    }

    pub fn to_string(&self) -> String {
        let mut string = String::new();
        // Get data as 4 byte chunks
        for byte_chunk in self.data.chunks(4) {
            // convert data to u32
            let u32 = u32::from_le_bytes(byte_chunk.try_into().unwrap());
            // convert u32 to char if failed add � [U+FFFD] instead.
            let char = char::from_u32(u32).unwrap_or(char::REPLACEMENT_CHARACTER);
            string.push(char);
        }
        string
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
            type_id: TypeId {
                id: 1,
                size: mem::size_of::<isize>(),
            },
            data,
        }
    }

    pub fn float(data: Vec<u8>) -> RawType {
        RawType {
            type_id: TypeId {
                id: 2,
                size: mem::size_of::<f64>(),
            },
            data,
        }
    }

    pub fn double(data: Vec<u8>) -> RawType {
        RawType {
            type_id: TypeId {
                id: 3,
                size: mem::size_of::<f32>(),
            },
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
}

#[derive(Debug, Copy, Clone)]
pub struct StaticRawType {
    pub type_id: TypeId,
    pub data: [u8; 8],
}

impl StaticRawType {
    /// Converts the type to a raw type
    /// TODO: Example doc
    pub fn to_raw(&self) -> RawType {
        RawType {
            type_id: TypeId {
                id: self.type_id.id,
                size: self.type_id.size,
            },
            data: self.data.to_vec(),
        }
    }

    /// Converts the type to bytes
    /// TODO: Example doc
    pub fn to_bytes(&self) -> Vec<u8> {
        let type_id = self.type_id.to_bytes();
        let mut bytes = Vec::from(type_id);
        bytes.extend(self.data);
        bytes
    }

    /// Converts bytes to a type
    /// TODO: Example doc
    pub fn from_bytes(bytes: &[u8]) -> StaticRawType {
        let type_id = TypeId::from_bytes(&bytes[..TYPE_SIZE].try_into().unwrap());
        let data: [u8; 8] = bytes[TYPE_SIZE..].try_into().unwrap();
        StaticRawType { type_id, data }
    }

    /// Converts the integer to a isize
    /// TODO: Example doc
    pub fn to_int(&self) -> isize {
        let mut integer = [0; mem::size_of::<isize>()];
        integer[0..].copy_from_slice(&self.data[0..mem::size_of::<isize>()]);
        isize::from_le_bytes(integer)
    }

    /// Converts the integer to a usize
    /// TODO: Example doc
    pub fn to_uint(&self) -> usize {
        let mut integer = [0; mem::size_of::<usize>()];
        integer[0..].copy_from_slice(&self.data[0..mem::size_of::<usize>()]);
        usize::from_le_bytes(integer)
    }

    /// Converts the float to a f64
    /// TODO: Example doc
    pub fn to_float(&self) -> f64 {
        f64::from_le_bytes(self.data[0..mem::size_of::<f64>()].try_into().unwrap())
    }

    /// Converts the double to a f32
    /// TODO: Example doc
    pub fn to_double(&self) -> f32 {
        f32::from_le_bytes(self.data[0..mem::size_of::<f32>()].try_into().unwrap())
    }

    /// Converts the byte to a u8
    /// TODO: Example doc
    pub fn to_byte(&self) -> u8 {
        self.data[0]
    }

    /// Converts the bool to a bool
    /// TODO: Example doc
    pub fn to_bool(&self) -> bool {
        self.data[0] == 1
    }

    /// Converts the char to a char
    /// TODO: Example doc
    pub fn to_char(&self) -> char {
        char::from_u32(u32::from_le_bytes(self.data[0..4].try_into().unwrap())).unwrap()
    }

    pub fn from_int(data: isize) -> StaticRawType {
        let mut bytes = [0; 8];
        bytes[0..mem::size_of::<isize>()].copy_from_slice(&data.to_le_bytes());
        StaticRawType {
            type_id: TypeId {
                id: 1,
                size: mem::size_of::<isize>(),
            },
            data: bytes,
        }
    }

    pub fn from_float(data: f64) -> StaticRawType {
        let mut bytes = [0; 8];
        bytes[0..mem::size_of::<f64>()].copy_from_slice(&data.to_le_bytes());
        StaticRawType {
            type_id: TypeId {
                id: 2,
                size: mem::size_of::<f32>(),
            },
            data: bytes,
        }
    }

    pub fn from_double(data: f32) -> StaticRawType {
        let mut bytes = [0; 8];
        bytes[0..mem::size_of::<f32>()].copy_from_slice(&data.to_le_bytes());
        StaticRawType {
            type_id: TypeId {
                id: 3,
                size: mem::size_of::<f32>(),
            },
            data: bytes,
        }
    }

    pub fn from_byte(data: u8) -> StaticRawType {
        StaticRawType {
            type_id: TypeId { id: 4, size: 1 },
            data: [data, 0, 0, 0, 0, 0, 0, 0],
        }
    }

    pub fn from_bool(data: bool) -> StaticRawType {
        StaticRawType {
            type_id: TypeId { id: 5, size: 1 },
            data: [data as u8, 0, 0, 0, 0, 0, 0, 0],
        }
    }

    pub fn from_char(data: char) -> StaticRawType {
        let mut bytes = [0; 8];
        bytes[0..4].copy_from_slice(&(data as u32).to_le_bytes());
        StaticRawType {
            type_id: TypeId { id: 6, size: 4 },
            data: bytes,
        }
    }

    pub fn from_void() -> StaticRawType {
        StaticRawType {
            type_id: TypeId { id: 8, size: 0 },
            data: [0; 8],
        }
    }

    pub fn from_null() -> StaticRawType {
        StaticRawType {
            type_id: TypeId { id: 10, size: 0 },
            data: [0; 8],
        }
    }

    pub fn from_function(data: usize) -> StaticRawType {
        let mut bytes = [0; 8];
        bytes[0..mem::size_of::<usize>()].copy_from_slice(&data.to_le_bytes());
        StaticRawType {
            type_id: TypeId {
                id: 12,
                size: mem::size_of::<usize>(),
            },
            data: bytes,
        }
    }

    pub fn from_stack_reference(data: usize) -> StaticRawType {
        let mut bytes = [0; 8];
        bytes[0..mem::size_of::<usize>()].copy_from_slice(&data.to_le_bytes());
        StaticRawType {
            type_id: TypeId {
                id: 13,
                size: mem::size_of::<usize>(),
            },
            data: bytes,
        }
    }

    pub fn from_heap_reference(data: usize) -> StaticRawType {
        let mut bytes = [0; 8];
        bytes[0..mem::size_of::<usize>()].copy_from_slice(&data.to_le_bytes());
        StaticRawType {
            type_id: TypeId {
                id: 14,
                size: mem::size_of::<usize>(),
            },
            data: bytes,
        }
    }
}
