#![allow(dead_code)]

use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use ellie_core::defs::PlatformArchitecture;

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

#[derive(Clone, Debug, PartialEq)]
pub enum Types {
    Integer,
    Float,
    Double,
    Byte,
    Bool,
    String(usize),
    Char,
    Array(usize),
    //Vector,
    Void,
    Null,
}

impl Types {
    pub fn display(&self) -> String {
        match &self {
            Types::Integer => "int".to_string(),
            Types::Float => "float".to_string(),
            Types::Double => "double".to_string(),
            Types::Byte => "byte".to_string(),
            Types::Bool => "bool".to_string(),
            Types::String(str_len) => alloc::format!("string[{str_len}]"),
            Types::Char => alloc::format!("char"),
            Types::Array(len) => alloc::format!("array<{len}>"),
            Types::Void => "void".to_string(),
            Types::Null => "null".to_string(),
        }
    }

    //(type_id, size)
    // (1, platform_size) Integer
    pub fn code(&self, platform_size: &PlatformArchitecture) -> (u8, usize) {
        match self {
            Types::Integer => (1, platform_size.usize_len() as usize),
            Types::Float => (2, platform_size.usize_len() as usize),
            Types::Double => (3, platform_size.usize_len() as usize),
            Types::Byte => (4, 1),
            Types::Bool => (5, 1),
            Types::String(str_len) => (6, *str_len),
            Types::Char => (7, 4),
            Types::Void => (8, 0),
            Types::Array(len) => (9, *len),
            Types::Null => (10, 0),
        }
    }
}

#[derive(Clone, Debug)]
pub enum AddressingModes {
    Implicit,
    Immediate(Types, Vec<u8>),
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
    pub fn to_string(&self) -> String {
        match self {
            AddressingModes::Implicit => "implicit",
            AddressingModes::Immediate(_, _) => "immediate",
            AddressingModes::Absolute(_) => "absolute",
            AddressingModes::AbsoluteIndex(_, _) => "absolute_index",
            AddressingModes::AbsoluteProperty(_, _) => "absolute_property",
            AddressingModes::IndirectA => "indirect_a",
            AddressingModes::IndirectB => "indirect_b",
            AddressingModes::IndirectC => "indirect_c",
            AddressingModes::IndirectX => "indirect_x",
            AddressingModes::IndirectY => "indirect_y",
        }
        .to_string()
    }

    pub fn arg(&self, platform_size: &PlatformArchitecture) -> Vec<u8> {
        match self {
            AddressingModes::Immediate(rtype, x) => {
                let mut v = vec![];
                let code = rtype.code(platform_size);
                v.extend(code.0.to_le_bytes().to_vec());
                v.extend(code.1.to_le_bytes().to_vec());
                v.extend(x);
                v
            }
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
            AddressingModes::Immediate(rtype, value) => {
                write!(f, "#({}){:?}", rtype.display(), value)
            }
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

    pub fn immediate(rtype: Types, val: Vec<u8>) -> Instruction {
        Instruction {
            addressing_mode: AddressingModes::Immediate(rtype, val),
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
    JMP(Instruction),
    CALL(Instruction),
    CALLN(Instruction),
    RET(Instruction),
    AOL(Instruction),
    PUSHA(Instruction),
    LEN(Instruction),
    A2I(Instruction),
    A2F(Instruction),
    A2D(Instruction),
    A2B(Instruction),
    A2S(Instruction),
    A2C(Instruction),
    A2O(Instruction),
    JMPA(Instruction),
    POPS(Instruction),
    ACP(Instruction),
    BRK(Instruction),
}

impl Instructions {
    pub fn op_code(&self, platform_size: &PlatformArchitecture) -> Vec<u8> {
        //let entries = crate::instruction_table::Instructions.entries();

        match self {
            Instructions::LDA(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "lda_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::LDB(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "ldb_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::LDC(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "ldc_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::LDX(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "ldx_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::LDY(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "ldy_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::STA(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "sta_".to_string() + &e.addressing_mode.to_string())
                    .unwrap_or_else(|| panic!("sta_{}", &e.addressing_mode.to_string()));
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::STB(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "stb_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::STC(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "stc_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::STX(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "stx_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::STY(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "sty_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::EQ(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "eq_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::NE(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "ne_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::GT(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "gt_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::LT(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "lt_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::GQ(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "gq_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::LQ(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "lq_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::AND(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "and_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::OR(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "or_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::ADD(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "add_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::SUB(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "sub_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::MUL(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "mul_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::EXP(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "exp_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::DIV(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "div_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::MOD(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "mod_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::CALL(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "call_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::AOL(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "aol_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::PUSHA(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "push_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::LEN(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "len_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::A2I(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "a2i_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::A2F(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "a2f_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::A2D(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "a2d_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::A2B(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "a2b_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::A2S(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "a2s_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::A2C(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "a2c_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::A2O(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "a2o_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::JMP(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "jmp_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::JMPA(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "jmpa_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::POPS(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "pops_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::RET(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "ret_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::ACP(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "acp_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::BRK(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "brk_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
            Instructions::CALLN(e) => {
                let instruction = crate::instruction_table::INSTRUCTIONS
                    .clone()
                    .drain()
                    .find(|(k, _)| *k == "calln_".to_string() + &e.addressing_mode.to_string())
                    .unwrap();
                let mut op_code: Vec<u8> = vec![instruction.1.code];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            }
        }
    }

    pub fn get_addressing_mode(&self) -> String {
        match &self {
            Instructions::LDA(e) => e.addressing_mode.clone(),
            Instructions::LDB(e) => e.addressing_mode.clone(),
            Instructions::LDC(e) => e.addressing_mode.clone(),
            Instructions::LDX(e) => e.addressing_mode.clone(),
            Instructions::LDY(e) => e.addressing_mode.clone(),
            Instructions::STA(e) => e.addressing_mode.clone(),
            Instructions::STB(e) => e.addressing_mode.clone(),
            Instructions::STC(e) => e.addressing_mode.clone(),
            Instructions::STX(e) => e.addressing_mode.clone(),
            Instructions::STY(e) => e.addressing_mode.clone(),
            Instructions::EQ(e) => e.addressing_mode.clone(),
            Instructions::NE(e) => e.addressing_mode.clone(),
            Instructions::GT(e) => e.addressing_mode.clone(),
            Instructions::LT(e) => e.addressing_mode.clone(),
            Instructions::GQ(e) => e.addressing_mode.clone(),
            Instructions::LQ(e) => e.addressing_mode.clone(),
            Instructions::AND(e) => e.addressing_mode.clone(),
            Instructions::OR(e) => e.addressing_mode.clone(),
            Instructions::ADD(e) => e.addressing_mode.clone(),
            Instructions::SUB(e) => e.addressing_mode.clone(),
            Instructions::MUL(e) => e.addressing_mode.clone(),
            Instructions::EXP(e) => e.addressing_mode.clone(),
            Instructions::DIV(e) => e.addressing_mode.clone(),
            Instructions::MOD(e) => e.addressing_mode.clone(),
            Instructions::JMP(e) => e.addressing_mode.clone(),
            Instructions::CALL(e) => e.addressing_mode.clone(),
            Instructions::CALLN(e) => e.addressing_mode.clone(),
            Instructions::RET(e) => e.addressing_mode.clone(),
            Instructions::AOL(e) => e.addressing_mode.clone(),
            Instructions::PUSHA(e) => e.addressing_mode.clone(),
            Instructions::LEN(e) => e.addressing_mode.clone(),
            Instructions::A2I(e) => e.addressing_mode.clone(),
            Instructions::A2F(e) => e.addressing_mode.clone(),
            Instructions::A2D(e) => e.addressing_mode.clone(),
            Instructions::A2B(e) => e.addressing_mode.clone(),
            Instructions::A2S(e) => e.addressing_mode.clone(),
            Instructions::A2C(e) => e.addressing_mode.clone(),
            Instructions::A2O(e) => e.addressing_mode.clone(),
            Instructions::JMPA(e) => e.addressing_mode.clone(),
            Instructions::POPS(e) => e.addressing_mode.clone(),
            Instructions::ACP(e) => e.addressing_mode.clone(),
            Instructions::BRK(e) => e.addressing_mode.clone(),
        }
        .to_string()
    }

    pub fn get_arg(&self, platform_size: &PlatformArchitecture) -> Vec<u8> {
        match self {
            Instructions::LDA(e) => e.addressing_mode.arg(platform_size),
            Instructions::LDB(e) => e.addressing_mode.arg(platform_size),
            Instructions::LDC(e) => e.addressing_mode.arg(platform_size),
            Instructions::LDX(e) => e.addressing_mode.arg(platform_size),
            Instructions::LDY(e) => e.addressing_mode.arg(platform_size),
            Instructions::STA(e) => e.addressing_mode.arg(platform_size),
            Instructions::STB(e) => e.addressing_mode.arg(platform_size),
            Instructions::STC(e) => e.addressing_mode.arg(platform_size),
            Instructions::STX(e) => e.addressing_mode.arg(platform_size),
            Instructions::STY(e) => e.addressing_mode.arg(platform_size),
            Instructions::EQ(e) => e.addressing_mode.arg(platform_size),
            Instructions::NE(e) => e.addressing_mode.arg(platform_size),
            Instructions::GT(e) => e.addressing_mode.arg(platform_size),
            Instructions::LT(e) => e.addressing_mode.arg(platform_size),
            Instructions::GQ(e) => e.addressing_mode.arg(platform_size),
            Instructions::LQ(e) => e.addressing_mode.arg(platform_size),
            Instructions::AND(e) => e.addressing_mode.arg(platform_size),
            Instructions::OR(e) => e.addressing_mode.arg(platform_size),
            Instructions::ADD(e) => e.addressing_mode.arg(platform_size),
            Instructions::SUB(e) => e.addressing_mode.arg(platform_size),
            Instructions::MUL(e) => e.addressing_mode.arg(platform_size),
            Instructions::EXP(e) => e.addressing_mode.arg(platform_size),
            Instructions::DIV(e) => e.addressing_mode.arg(platform_size),
            Instructions::MOD(e) => e.addressing_mode.arg(platform_size),
            Instructions::JMP(e) => e.addressing_mode.arg(platform_size),
            Instructions::CALL(e) => e.addressing_mode.arg(platform_size),
            Instructions::CALLN(e) => e.addressing_mode.arg(platform_size),
            Instructions::RET(e) => e.addressing_mode.arg(platform_size),
            Instructions::AOL(e) => e.addressing_mode.arg(platform_size),
            Instructions::PUSHA(e) => e.addressing_mode.arg(platform_size),
            Instructions::LEN(e) => e.addressing_mode.arg(platform_size),
            Instructions::A2I(e) => e.addressing_mode.arg(platform_size),
            Instructions::A2F(e) => e.addressing_mode.arg(platform_size),
            Instructions::A2D(e) => e.addressing_mode.arg(platform_size),
            Instructions::A2B(e) => e.addressing_mode.arg(platform_size),
            Instructions::A2S(e) => e.addressing_mode.arg(platform_size),
            Instructions::A2C(e) => e.addressing_mode.arg(platform_size),
            Instructions::A2O(e) => e.addressing_mode.arg(platform_size),
            Instructions::JMPA(e) => e.addressing_mode.arg(platform_size),
            Instructions::POPS(e) => e.addressing_mode.arg(platform_size),
            Instructions::ACP(e) => e.addressing_mode.arg(platform_size),
            Instructions::BRK(e) => e.addressing_mode.arg(platform_size),
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
            Instructions::CALLN(instruction) => write!(f, "CALLN {}", instruction.addressing_mode),
            Instructions::AOL(instruction) => write!(f, "AOL {}", instruction.addressing_mode),
            Instructions::PUSHA(instruction) => write!(f, "PUSH {}", instruction.addressing_mode),
            Instructions::LEN(instruction) => write!(f, "LEN {}", instruction.addressing_mode),
            Instructions::A2I(instruction) => write!(f, "A2I {}", instruction.addressing_mode),
            Instructions::A2F(instruction) => write!(f, "A2F {}", instruction.addressing_mode),
            Instructions::A2D(instruction) => write!(f, "A2D {}", instruction.addressing_mode),
            Instructions::A2B(instruction) => write!(f, "A2B {}", instruction.addressing_mode),
            Instructions::A2S(instruction) => write!(f, "A2S {}", instruction.addressing_mode),
            Instructions::A2C(instruction) => write!(f, "A2C {}", instruction.addressing_mode),
            Instructions::A2O(instruction) => write!(f, "A2O {}", instruction.addressing_mode),
            Instructions::JMP(instruction) => write!(f, "JMP {}", instruction.addressing_mode),
            Instructions::JMPA(instruction) => write!(f, "JMPA {}", instruction.addressing_mode),
            Instructions::POPS(instruction) => write!(f, "POPS {}", instruction.addressing_mode),
            Instructions::RET(instruction) => write!(f, "RET {}", instruction.addressing_mode),
            Instructions::ACP(instruction) => write!(f, "ACP {}", instruction.addressing_mode),
            Instructions::BRK(_) => write!(f, "BRK"),
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
            (Self::Implicit(_), Self::Implicit(_)) => true,
            (Self::Immediate(_), Self::Immediate(_)) => true,
            (Self::Absolute(_), Self::Absolute(_)) => true,
            (Self::AbsoluteIndex(_), Self::AbsoluteIndex(_)) => true,
            (Self::AbsoluteProperty(_), Self::AbsoluteProperty(_)) => true,
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
    pub fn from_str<'a>(mode: &'a str, op_code: u8) -> AddressingModesStruct {
        match mode {
            "implicit" => AddressingModesStruct::Implicit(op_code),
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
