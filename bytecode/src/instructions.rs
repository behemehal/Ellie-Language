#![allow(dead_code)]
use alloc::{string::String, vec, vec::Vec};

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
            _ => vec![],
        }
    }
}

impl core::fmt::Display for AddressingModes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match self {
            AddressingModes::Absolute(value) => write!(f, "${}", value),
            AddressingModes::Immediate(value) => write!(f, "#{}", value),
            AddressingModes::IndirectA => write!(f, "@A"),
            AddressingModes::IndirectB => write!(f, "@B"),
            AddressingModes::IndirectC => write!(f, "@C"),
            AddressingModes::IndirectX => write!(f, "@X"),
            AddressingModes::IndirectY => write!(f, "@Y"),
            _ => write!(f, ""),
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
}

impl Instructions {
    pub fn op_code(&self) -> Vec<u8> {
        todo!()
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

#[derive(Clone, Debug, PartialEq)]
pub enum AddressingModesStruct {
    Implicit,
    Immediate,
    Absolute,
    IndirectA,
    IndirectB,
    IndirectC,
    IndirectX,
    IndirectY,
}

#[derive(Clone, Debug)]
pub struct InstructionStruct {
    pub op_code: String,
    pub rtype: String,
    pub modes: Vec<AddressingModesStruct>,
}
