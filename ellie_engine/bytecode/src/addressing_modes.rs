use core::mem;

use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use ellie_core::defs::PlatformArchitecture;

use crate::{types::Types, utils::usize_to_le_bytes};

#[derive(Clone, Debug)]
pub struct Immediate;

#[derive(Clone, Debug)]
pub struct Implict;

#[derive(Clone, Debug)]
pub struct Absolute(usize);

#[derive(Clone, Debug)]
pub struct IndirectA;

#[derive(Clone, Debug)]
pub struct IndirectB;

#[derive(Clone, Debug)]
pub struct IndirectC;

#[derive(Clone, Debug)]
pub struct IndirectX;

#[derive(Clone, Debug)]
pub struct IndirectY;

#[derive(Clone, Debug, PartialEq)]
pub enum AddressingModes {
    Implicit,
    Immediate(Types, Vec<u8>),
    Absolute(usize),
    AbsoluteIndex(usize, usize),
    AbsoluteProperty(usize, usize),
    AbsoluteStatic(usize),
    IndirectA,
    IndirectB,
    IndirectC,
    IndirectX,
    IndirectY,
}

impl AddressingModes {
    pub fn to_string(&self) -> String {
        match self {
            AddressingModes::Implicit => "implicit",
            AddressingModes::Immediate(_, _) => "immediate",
            AddressingModes::Absolute(_) => "absolute",
            AddressingModes::AbsoluteIndex(_, _) => "absolute_index",
            AddressingModes::AbsoluteProperty(_, _) => "absolute_index",
            AddressingModes::AbsoluteStatic(_) => "absolute_static",
            AddressingModes::IndirectA => "indirect_a",
            AddressingModes::IndirectB => "indirect_b",
            AddressingModes::IndirectC => "indirect_c",
            AddressingModes::IndirectX => "indirect_x",
            AddressingModes::IndirectY => "indirect_y",
        }
        .to_string()
    }

    pub fn idx(&self) -> usize {
        match self {
            AddressingModes::Implicit => 0,
            AddressingModes::Immediate(_, _) => 1,
            AddressingModes::Absolute(_) => 2,
            AddressingModes::AbsoluteIndex(_, _) => 3,
            AddressingModes::AbsoluteProperty(_, _) => 4,
            AddressingModes::AbsoluteStatic(_) => 5,
            AddressingModes::IndirectA => 6,
            AddressingModes::IndirectB => 7,
            AddressingModes::IndirectC => 8,
            AddressingModes::IndirectX => 9,
            AddressingModes::IndirectY => 10,
        }
    }

    pub fn arg(&self, platform_size: PlatformArchitecture) -> Vec<u8> {
        match self {
            AddressingModes::Immediate(rtype, x) => {
                let mut v = vec![];
                let (type_size, type_data) = rtype.code(platform_size);
                v.push(type_size);
                v.extend(usize_to_le_bytes(type_data, platform_size));
                v.extend(x);
                v
            }
            AddressingModes::Absolute(x) => usize_to_le_bytes(*x, platform_size),
            AddressingModes::AbsoluteIndex(array_pointer, index_pointer) => {
                let mut v = vec![];
                v.extend(usize_to_le_bytes(*array_pointer, platform_size));
                v.extend(usize_to_le_bytes(*index_pointer, platform_size));
                v
            }
            AddressingModes::AbsoluteProperty(array_pointer, index_pointer) => {
                let mut v = vec![];
                v.extend(usize_to_le_bytes(*array_pointer, platform_size));
                v.extend(usize_to_le_bytes(*index_pointer, platform_size));
                v
            }
            AddressingModes::AbsoluteStatic(x) => usize_to_le_bytes(*x, platform_size),
            _ => vec![],
        }
    }
}

impl core::fmt::Display for AddressingModes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match self {
            AddressingModes::Absolute(value) => write!(f, "${}", value),
            //AddressingModes::AbsoluteRef(page, value) => write!(f, "${}~{}", value, page),
            AddressingModes::Immediate(rtype, value) => {
                write!(
                    f,
                    "#({}){}",
                    rtype.display(),
                    match rtype {
                        Types::Integer => {
                            let mut array = [0; mem::size_of::<isize>()];
                            array[0..value.len()].copy_from_slice(&value[..]);
                            isize::from_le_bytes(array)
                        }
                        .to_string(),
                        Types::Float =>
                            f64::from_le_bytes(value[0..mem::size_of::<f64>()].try_into().unwrap())
                                .to_string(),
                        Types::Double =>
                            f32::from_le_bytes(value[0..mem::size_of::<f32>()].try_into().unwrap())
                                .to_string(),
                        Types::Byte => format!("0x{}", value[0]),
                        Types::Bool =>
                            if value[0] == 1 {
                                "true".to_string()
                            } else {
                                "false".to_string()
                            },
                        Types::String(_) => format!("string[{:?}]", {
                            let mut array = [0; mem::size_of::<isize>()];
                            array.copy_from_slice(&value[0..mem::size_of::<isize>()]);
                            isize::from_le_bytes(array)
                        }),
                        Types::Char => {
                            format!(
                                "'{:?}'",
                                char::from_u32(u32::from_le_bytes(value[0..4].try_into().unwrap()))
                                    .unwrap()
                            )
                        }
                        Types::StaticArray => "static_array".to_string(),
                        Types::Array(_) => format!("array[{:?}]", {
                            let mut array = [0; mem::size_of::<isize>()];
                            array.copy_from_slice(&value[0..mem::size_of::<isize>()]);
                            isize::from_le_bytes(array)
                        }),
                        Types::Void => String::new(),
                        Types::Null => String::new(),
                        Types::Class(_) => format!("class({:?})", {
                            let mut array = [0; mem::size_of::<isize>()];
                            array.copy_from_slice(&value[0..mem::size_of::<isize>()]);
                            usize::from_le_bytes(array)
                        }),
                        Types::Function => format!("fn({:?})", {
                            let mut array = [0; mem::size_of::<isize>()];
                            array.copy_from_slice(&value[0..mem::size_of::<isize>()]);
                            usize::from_le_bytes(array)
                        }),
                        Types::HeapReference => format!("href({:?})", {
                            let mut array = [0; mem::size_of::<isize>()];
                            array.copy_from_slice(&value[0..mem::size_of::<isize>()]);
                            usize::from_le_bytes(array)
                        }),
                        Types::StackReference => format!("sref({:?})", {
                            let mut array = [0; mem::size_of::<isize>()];
                            array.copy_from_slice(&value[0..mem::size_of::<isize>()]);
                            usize::from_le_bytes(array)
                        }),
                    }
                )
            }
            AddressingModes::IndirectA => write!(f, "@A"),
            AddressingModes::IndirectB => write!(f, "@B"),
            AddressingModes::IndirectC => write!(f, "@C"),
            AddressingModes::IndirectX => write!(f, "@X"),
            AddressingModes::IndirectY => write!(f, "@Y"),
            AddressingModes::Implicit => write!(f, ""),
            AddressingModes::AbsoluteIndex(pointer, idx_pointer) => {
                write!(f, "${pointer}[${idx_pointer}]")
            }
            AddressingModes::AbsoluteProperty(pointer, idx_pointer) => {
                write!(f, "@{pointer}[{idx_pointer}]")
            }
            AddressingModes::AbsoluteStatic(value) => write!(f, "$x{}", value),
        }
    }
}

#[derive(Clone, Debug)]
pub enum AddressingModesStruct {
    Implicit(u8),
    Immediate(u8),
    Absolute(u8),
    AbsoluteIndex(u8, u8),
    AbsoluteProperty(u8, u8),
    AbsoluteStatic(u8),
    IndirectA(u8),
    IndirectB(u8),
    IndirectC(u8),
    IndirectX(u8),
    IndirectY(u8),
}

impl PartialEq for AddressingModesStruct {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Implicit(_), Self::Implicit(_)) => true,
            (Self::Immediate(_), Self::Immediate(_)) => true,
            (Self::Absolute(_), Self::Absolute(_)) => true,
            (Self::AbsoluteIndex(_, _), Self::AbsoluteIndex(_, _)) => true,
            (Self::AbsoluteProperty(_, _), Self::AbsoluteProperty(_, _)) => true,
            (Self::AbsoluteStatic(_), Self::AbsoluteStatic(_)) => true,
            (Self::IndirectA(_), Self::IndirectA(_)) => true,
            (Self::IndirectB(_), Self::IndirectB(_)) => true,
            (Self::IndirectC(_), Self::IndirectC(_)) => true,
            (Self::IndirectX(_), Self::IndirectX(_)) => true,
            (Self::IndirectY(_), Self::IndirectY(_)) => true,
            _ => false,
        }
    }
}

impl AddressingModesStruct {
    pub fn from_str(mode: &str, op_code: u8) -> AddressingModesStruct {
        match mode {
            "implicit" => AddressingModesStruct::Implicit(op_code),
            "immediate" => AddressingModesStruct::Immediate(op_code),
            "absolute" => AddressingModesStruct::Absolute(op_code),
            "absolute_index" => AddressingModesStruct::AbsoluteIndex(op_code, op_code),
            "absolute_property" => AddressingModesStruct::AbsoluteProperty(op_code, op_code),
            "absolute_static" => AddressingModesStruct::AbsoluteStatic(op_code),
            "indirect_a" => AddressingModesStruct::IndirectA(op_code),
            "indirect_b" => AddressingModesStruct::IndirectB(op_code),
            "indirect_c" => AddressingModesStruct::IndirectC(op_code),
            "indirect_x" => AddressingModesStruct::IndirectX(op_code),
            "indirect_y" => AddressingModesStruct::IndirectY(op_code),
            _ => panic!("Unknown addressing mode: {}", mode),
        }
    }

    pub fn val(&self) -> &u8 {
        match self {
            AddressingModesStruct::Implicit(value) => value,
            AddressingModesStruct::Immediate(value) => value,
            AddressingModesStruct::Absolute(value) => value,
            AddressingModesStruct::AbsoluteIndex(value, _) => value,
            AddressingModesStruct::AbsoluteProperty(value, _) => value,
            AddressingModesStruct::AbsoluteStatic(value) => value,
            AddressingModesStruct::IndirectA(value) => value,
            AddressingModesStruct::IndirectB(value) => value,
            AddressingModesStruct::IndirectC(value) => value,
            AddressingModesStruct::IndirectX(value) => value,
            AddressingModesStruct::IndirectY(value) => value,
        }
    }
}

#[derive(Clone, Debug)]
pub struct InstructionStruct<'a> {
    pub op_code: u8,
    pub rtype: &'a str,
    pub modes: Vec<AddressingModesStruct>,
}
