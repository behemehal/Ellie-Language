use std::{format, string::String};

use alloc::vec::Vec;
use byteorder::{ByteOrder, LittleEndian};

use crate::{
    opcode::OpCode,
    operand::{Operand, OPERAND_SIZE},
    types::Types,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Instruction {
    pub op_code: OpCode,
    pub operand_0: Option<Operand>,
    pub operand_1: Option<Operand>,
    pub operand_2: Option<Operand>,
}

pub(crate) const INSTRUCTION_SIZE: usize = 2 + (OPERAND_SIZE * 3);

impl Instruction {
    pub fn to_bytes(&self) -> [u8; INSTRUCTION_SIZE] {
        //0: OpCode u8,
        //1: Operand Size: u8,
        let mut buf = [0_u8; INSTRUCTION_SIZE]; // 2 bytes for op_code and operand size, plus size of operands

        buf[0] = self.op_code as u8;
        buf[1] = self.operand_size() as u8;

        if let Some(operand) = &self.operand_0 {
            buf[2..OPERAND_SIZE].copy_from_slice(&operand.to_bytes()[..]);
        } else {
            buf[2..OPERAND_SIZE].copy_from_slice(&0u8.to_le_bytes()[..]);
        }

        if let Some(operand) = &self.operand_1 {
            buf[2 + OPERAND_SIZE..2 + (OPERAND_SIZE * 2)].copy_from_slice(&operand.to_bytes()[..]);
        } else {
            buf[2 + OPERAND_SIZE..2 + (OPERAND_SIZE * 2)].copy_from_slice(&0u8.to_le_bytes()[..]);
        }

        if let Some(operand) = &self.operand_2 {
            buf[2 + (OPERAND_SIZE * 2)..2 + (OPERAND_SIZE * 3)]
                .copy_from_slice(&operand.to_bytes()[..]);
        } else {
            buf[2 + (OPERAND_SIZE * 2)..2 + (OPERAND_SIZE * 3)]
                .copy_from_slice(&0u8.to_le_bytes()[..]);
        }

        buf
    }

    fn operand_size(&self) -> usize {
        let size = self.operand_0.is_some() as usize;
        let size = size + self.operand_1.is_some() as usize;
        let size = size + self.operand_2.is_some() as usize;

        size
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!("{:?}", self.op_code));
        if let Some(operand) = &self.operand_0 {
            result.push_str(&format!(" {}", operand.to_string()));
        }
        if let Some(operand) = &self.operand_1 {
            result.push_str(&format!(" {}", operand.to_string()));
        }
        if let Some(operand) = &self.operand_2 {
            result.push_str(&format!(" {}", operand.to_string()));
        }
        result
    }
}
