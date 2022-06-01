#![allow(dead_code)]
use core::fmt::Display;

use alloc::{vec, vec::Vec};

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

#[derive(Clone, Debug)]
pub struct Reference;

#[derive(Clone, Debug)]
pub enum AddressingModes {
    Implicit,
    Immediate(isize),
    Absolute(usize),
    AbsoluteIndex(usize, usize),
    AbsoluteProperty(usize, usize),
    //AbsoluteRef(usize, usize),
    IndirectA,
    IndirectB,
    IndirectC,
    IndirectX,
    IndirectY,
}

impl AddressingModes {
    pub fn arg(&self) -> Vec<u8> {
        match self {
            AddressingModes::Absolute(x) => x.to_le_bytes().to_vec(),
            AddressingModes::AbsoluteIndex(arr, index) => {
                let mut v = vec![];
                v.extend_from_slice(&arr.to_le_bytes());
                v.extend_from_slice(&index.to_le_bytes());
                v
            }
            AddressingModes::AbsoluteProperty(obj, property) => {
                let mut v = vec![];
                v.extend_from_slice(&obj.to_le_bytes());
                v.extend_from_slice(&property.to_le_bytes());
                v
            }
            _ => vec![],
        }
    }
}

impl core::fmt::Display for AddressingModes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match self {
            AddressingModes::Absolute(value) => write!(f, "${}", value),
            //AddressingModes::AbsoluteRef(page, value) => write!(f, "${}~{}", value, page),
            AddressingModes::Immediate(value) => write!(f, "#{}", value),
            AddressingModes::IndirectA => write!(f, "@A"),
            AddressingModes::IndirectB => write!(f, "@B"),
            AddressingModes::IndirectC => write!(f, "@C"),
            AddressingModes::IndirectX => write!(f, "@X"),
            AddressingModes::IndirectY => write!(f, "@Y"),
            AddressingModes::Implicit => write!(f, ""),
            AddressingModes::AbsoluteIndex(value, index) => write!(f, "${}[{}]", value, index),
            AddressingModes::AbsoluteProperty(value, property) => {
                write!(f, "${}.{}", value, property)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Instruction {
    addressing_mode: AddressingModes,
}

impl Instruction {
    pub fn implict() -> Instruction {
        Instruction {
            addressing_mode: AddressingModes::Implicit,
        }
    }

    pub fn immediate(val: isize) -> Instruction {
        Instruction {
            addressing_mode: AddressingModes::Immediate(val),
        }
    }

    pub fn absolute(val: usize) -> Instruction {
        Instruction {
            addressing_mode: AddressingModes::Absolute(val),
        }
    }

    //pub fn absolute_ref(page: usize, val: usize) -> Instruction {
    //    Instruction {
    //        addressing_mode: AddressingModes::AbsoluteRef(page, val),
    //    }
    //}

    pub fn indirect_a() -> Instruction {
        Instruction {
            addressing_mode: AddressingModes::IndirectA,
        }
    }

    pub fn indirect_b() -> Instruction {
        Instruction {
            addressing_mode: AddressingModes::IndirectB,
        }
    }

    pub fn indirect_c() -> Instruction {
        Instruction {
            addressing_mode: AddressingModes::IndirectC,
        }
    }

    pub fn indirect_x() -> Instruction {
        Instruction {
            addressing_mode: AddressingModes::IndirectX,
        }
    }

    pub fn indirect_y() -> Instruction {
        Instruction {
            addressing_mode: AddressingModes::IndirectY,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Instructions {
    LDA(Instruction),
    LDB(Instruction),
    LDC(Instruction),
    LDX(Instruction),
    LDY(Instruction),
    STA(Instruction),
    STB(Instruction),
    STC(Instruction),
    STX(Instruction),
    STY(Instruction),
    EQ(Instruction),
    NE(Instruction),
    GT(Instruction),
    LT(Instruction),
    GQ(Instruction),
    LQ(Instruction),
    AND(Instruction),
    OR(Instruction),
    ADD(Instruction),
    SUB(Instruction),
    MUL(Instruction),
    EXP(Instruction),
    DIV(Instruction),
    MOD(Instruction),
    CALL(Instruction),
    AOL(Instruction),
    PUSH(Instruction),
    LEN(Instruction),
    A2I(Instruction),
    A2F(Instruction),
    A2D(Instruction),
    A2B(Instruction),
    A2S(Instruction),
    A2C(Instruction),
    A2O(Instruction),
}

impl Instructions {
    pub fn op_code(&self) -> Vec<u8> {
        //let entries = crate::instruction_table::Instructions.entries();

        match self {
            Instructions::LDA(_) => todo!(),
            Instructions::LDB(_) => todo!(),
            Instructions::LDC(_) => todo!(),
            Instructions::LDX(_) => todo!(),
            Instructions::LDY(_) => todo!(),
            Instructions::STA(_) => todo!(),
            Instructions::STB(_) => todo!(),
            Instructions::STC(_) => todo!(),
            Instructions::STX(_) => todo!(),
            Instructions::STY(_) => todo!(),
            Instructions::EQ(_) => todo!(),
            Instructions::NE(_) => todo!(),
            Instructions::GT(_) => todo!(),
            Instructions::LT(_) => todo!(),
            Instructions::GQ(_) => todo!(),
            Instructions::LQ(_) => todo!(),
            Instructions::AND(_) => todo!(),
            Instructions::OR(_) => todo!(),
            Instructions::ADD(_) => todo!(),
            Instructions::SUB(_) => todo!(),
            Instructions::MUL(_) => todo!(),
            Instructions::EXP(_) => todo!(),
            Instructions::DIV(_) => todo!(),
            Instructions::MOD(_) => todo!(),
            Instructions::CALL(_) => todo!(),
            Instructions::AOL(_) => todo!(),
            Instructions::PUSH(_) => todo!(),
            Instructions::LEN(_) => todo!(),
            Instructions::A2I(_) => todo!(),
            Instructions::A2F(_) => todo!(),
            Instructions::A2D(_) => todo!(),
            Instructions::A2B(_) => todo!(),
            Instructions::A2S(_) => todo!(),
            Instructions::A2C(_) => todo!(),
            Instructions::A2O(_) => todo!(),
        }
    }
}

impl core::fmt::Display for Instructions {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match self {
            Instructions::LDA(instruction) => write!(f, "LDA {}", instruction.addressing_mode),
            Instructions::LDB(instruction) => write!(f, "LDB {}", instruction.addressing_mode),
            Instructions::LDC(instruction) => write!(f, "LDC {}", instruction.addressing_mode),
            Instructions::LDX(instruction) => write!(f, "LDX {}", instruction.addressing_mode),
            Instructions::LDY(instruction) => write!(f, "LDY {}", instruction.addressing_mode),
            Instructions::STA(instruction) => write!(f, "STA {}", instruction.addressing_mode),
            Instructions::STB(instruction) => write!(f, "STB {}", instruction.addressing_mode),
            Instructions::STC(instruction) => write!(f, "STC {}", instruction.addressing_mode),
            Instructions::STX(instruction) => write!(f, "STX {}", instruction.addressing_mode),
            Instructions::STY(instruction) => write!(f, "STY {}", instruction.addressing_mode),
            Instructions::EQ(_) => write!(f, "EQ"),
            Instructions::NE(_) => write!(f, "NE"),
            Instructions::GT(_) => write!(f, "GT"),
            Instructions::LT(_) => write!(f, "LT"),
            Instructions::GQ(_) => write!(f, "GQ"),
            Instructions::LQ(_) => write!(f, "LQ"),
            Instructions::AND(_) => write!(f, "AND"),
            Instructions::OR(_) => write!(f, "OR"),
            Instructions::ADD(_) => write!(f, "ADD"),
            Instructions::SUB(_) => write!(f, "SUB"),
            Instructions::MUL(_) => write!(f, "MUL"),
            Instructions::EXP(_) => write!(f, "EXP"),
            Instructions::DIV(_) => write!(f, "DIV"),
            Instructions::MOD(_) => write!(f, "MOD"),
            Instructions::CALL(instruction) => write!(f, "CALL {}", instruction.addressing_mode),
            Instructions::AOL(instruction) => write!(f, "AOL {}", instruction.addressing_mode),
            Instructions::PUSH(instruction) => write!(f, "PUSH {}", instruction.addressing_mode),
            Instructions::LEN(instruction) => write!(f, "LEN {}", instruction.addressing_mode),
            Instructions::A2I(instruction) => write!(f, "A2I {}", instruction.addressing_mode),
            Instructions::A2F(instruction) => write!(f, "A2F {}", instruction.addressing_mode),
            Instructions::A2D(instruction) => write!(f, "A2D {}", instruction.addressing_mode),
            Instructions::A2B(instruction) => write!(f, "A2B {}", instruction.addressing_mode),
            Instructions::A2S(instruction) => write!(f, "A2S {}", instruction.addressing_mode),
            Instructions::A2C(instruction) => write!(f, "A2C {}", instruction.addressing_mode),
            Instructions::A2O(instruction) => write!(f, "A2O {}", instruction.addressing_mode),
        }
    }
}

pub enum Registers {
    A,
    B,
    C,
    X,
    Y,
}

#[derive(Clone, Debug)]
pub enum AddressingModesStruct {
    Implicit(u8),
    Immediate(u8),
    Absolute(u8),
    AbsoluteIndex(u8),
    AbsoluteProperty(u8),
    IndirectA(u8),
    IndirectB(u8),
    IndirectC(u8),
    IndirectX(u8),
    IndirectY(u8),
}

impl PartialEq for AddressingModesStruct {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Implicit(l0), Self::Implicit(r0)) => true,
            (Self::Immediate(l0), Self::Immediate(r0)) => true,
            (Self::Absolute(l0), Self::Absolute(r0)) => true,
            (Self::AbsoluteIndex(l0), Self::AbsoluteIndex(r0)) => true,
            (Self::AbsoluteProperty(l0), Self::AbsoluteProperty(r0)) => true,
            (Self::IndirectA(l0), Self::IndirectA(r0)) => true,
            (Self::IndirectB(l0), Self::IndirectB(r0)) => true,
            (Self::IndirectC(l0), Self::IndirectC(r0)) => true,
            (Self::IndirectX(l0), Self::IndirectX(r0)) => true,
            (Self::IndirectY(l0), Self::IndirectY(r0)) => true,
            _ => false,
        }
    }
}

impl AddressingModesStruct {
    pub fn from_str<'a>(mode: &'a str, op_code: u8) -> AddressingModesStruct {
        match mode {
            "implict" => AddressingModesStruct::Implicit(op_code),
            "immediate" => AddressingModesStruct::Immediate(op_code),
            "absolute" => AddressingModesStruct::Absolute(op_code),
            "absolute_index" => AddressingModesStruct::AbsoluteIndex(op_code),
            "absolute_property" => AddressingModesStruct::AbsoluteProperty(op_code),
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
            AddressingModesStruct::AbsoluteIndex(value) => value,
            AddressingModesStruct::AbsoluteProperty(value) => value,
            AddressingModesStruct::IndirectA(value) => value,
            AddressingModesStruct::IndirectB(value) => value,
            AddressingModesStruct::IndirectC(value) => value,
            AddressingModesStruct::IndirectX(value) => value,
            AddressingModesStruct::IndirectY(value) => value
        }
    }
}

#[derive(Clone, Debug)]
pub struct InstructionStruct<'a> {
    pub op_code: u8,
    pub rtype: &'a str,
    pub modes: Vec<AddressingModesStruct>,
}
