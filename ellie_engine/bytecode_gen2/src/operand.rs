use alloc::format;
use core::mem;
use std::string::{String, ToString};

use byteorder::{ByteOrder, LittleEndian};
use ellie_core::bytecode::{RawType, TypeId, TYPE_SIZE};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Registers {
    A,
    B,
    C,
    X,
    Y,
    SP,
    FP,
    PC
}

#[derive(Debug, Clone, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum AddressingModes {
    Implicit,       // Implicit, example: RET
    Immediate,      // Immediate, example: JMP #1
    Register,       // Register, example: JMP A
    Indirect,       // Indirect, example: MOV [A], B
    IndirectOffset, // Base + offset, example: MOV A, [FP+2]
    Direct,         // Absolute memory address, example: MOV $0x1234, A
}

#[derive(Debug, Clone, PartialEq)]
pub struct Operand {
    pub mode: AddressingModes,
    pub register: Option<Registers>,
    pub immediate: Option<RawType>, // Immediate value
}

pub const OPERAND_SIZE: usize = 2 + TYPE_SIZE; // 2 bytes for mode and register, plus size of RawType

impl Operand {
    pub fn to_bytes(&self) -> [u8; OPERAND_SIZE] {
        let mut buf = [0_u8; OPERAND_SIZE]; // 2 bytes for mode and register, plus size of isize
        buf[0] = self.mode.clone() as u8;

        if let Some(register) = &self.register {
            buf[1] = register.clone() as u8;
        } else {
            buf[1] = 0; // Default value for register
        }

        if let Some(immediate) = &self.immediate {
            buf[2..OPERAND_SIZE].copy_from_slice(&immediate.to_bytes());
        } else {
            buf[2..OPERAND_SIZE].fill(0);
        }
        buf
    }

    pub fn to_string(&self) -> String {
        match self.mode {
            AddressingModes::Implicit => String::new(),
            AddressingModes::Immediate => {
                let value = self.immediate.expect("Immediate value is None");
                format!(
                    "#({:?}){}",
                    value.type_id,
                    match value.type_id {
                        TypeId::Int => TryInto::<isize>::try_into(value).unwrap().to_string(),
                        _ => format!(
                            "0x{:X}",
                            LittleEndian::read_u64(&value.to_bytes()[..mem::size_of::<u64>()])
                        ),
                    }
                )
            }
            AddressingModes::Register => {
                let reg = self.register.expect("Register is None");
                format!("{:?}", reg)
            }
            AddressingModes::Indirect => {
                let reg = self.register.expect("Register is None");
                format!("$[{:?}]", reg)
            }
            AddressingModes::IndirectOffset => {
                let reg = self.register.expect("Register is None");
                let imm = self.immediate.expect("Immediate is None");
                let offset: isize = imm.into();
                format!("$[{:?}+{}]", reg, offset)
            }
            AddressingModes::Direct => {
                let value = self.immediate.expect("Immediate value is None");
                format!(
                    "${:X}",
                    LittleEndian::read_u64(&value.to_bytes()[..mem::size_of::<u64>()])
                )
            }
        }
    }
}
