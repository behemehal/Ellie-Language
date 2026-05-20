use std::collections::HashMap;

use ellie_bytecode_gen2::{
    instructions::{Instruction, INSTRUCTION_SIZE},
    opcode::OpCode,
    operand::{AddressingModes, Operand, Registers, OPERAND_SIZE},
};
use ellie_core::{
    bytecode::{RawType, TYPE_SIZE},
    defs::PlatformArchitecture,
};

pub struct NativeTrace {
    pub module_name: String,
    pub function_hash: usize,
    pub function_name: String,
}

pub struct Program {
    pub arch: PlatformArchitecture,
    pub main_start: usize,
    pub main_hash: usize,
    pub native_traces: Vec<NativeTrace>,
    pub instructions: Vec<Instruction>,
    /// Maps function hash → instruction index of the first instruction after the Fn marker (the prologue).
    pub fn_table: HashMap<usize, usize>,
}

impl Program {
    pub fn load_from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.is_empty() {
            return Err("Empty binary".to_string());
        }

        let arch = PlatformArchitecture::from_byte(bytes[0])
            .ok_or_else(|| format!("Unknown architecture byte: {}", bytes[0]))?;
        let usize_len = arch.usize_len() as usize;
        let mut cursor = 1;

        let read_usize = |data: &[u8], pos: usize, usize_len: usize| -> usize {
            let mut buf = [0u8; 8];
            buf[..usize_len].copy_from_slice(&data[pos..pos + usize_len]);
            usize::from_le_bytes(buf)
        };

        let has_main = bytes[cursor] == 1;
        cursor += 1;

        let mut main_start = 0;
        let mut main_hash = 0;
        if has_main {
            main_start = read_usize(bytes, cursor, usize_len);
            cursor += usize_len;
            let _main_end = read_usize(bytes, cursor, usize_len);
            cursor += usize_len;
            main_hash = read_usize(bytes, cursor, usize_len);
            cursor += usize_len;
        }

        let native_count = read_usize(bytes, cursor, usize_len);
        cursor += usize_len;
        let mut native_traces = Vec::with_capacity(native_count);
        for _ in 0..native_count {
            let mod_len = read_usize(bytes, cursor, usize_len);
            cursor += usize_len;
            let module_name =
                String::from_utf8(bytes[cursor..cursor + mod_len].to_vec()).unwrap_or_default();
            cursor += mod_len;
            let function_hash = read_usize(bytes, cursor, usize_len);
            cursor += usize_len;
            let fn_len = read_usize(bytes, cursor, usize_len);
            cursor += usize_len;
            let function_name =
                String::from_utf8(bytes[cursor..cursor + fn_len].to_vec()).unwrap_or_default();
            cursor += fn_len;
            native_traces.push(NativeTrace {
                module_name,
                function_hash,
                function_name,
            });
        }

        // Decode instructions
        let remaining = &bytes[cursor..];
        let instr_count = remaining.len() / INSTRUCTION_SIZE;
        let mut instructions = Vec::with_capacity(instr_count);
        for i in 0..instr_count {
            let slice = &remaining[i * INSTRUCTION_SIZE..(i + 1) * INSTRUCTION_SIZE];
            instructions.push(decode_instruction(slice));
        }

        // Build hash → PC table from Fn marker opcodes
        let mut fn_table = HashMap::new();
        for (pc, instr) in instructions.iter().enumerate() {
            if instr.op_code == OpCode::Fn {
                if let Some(op0) = &instr.operand_0 {
                    if op0.mode == AddressingModes::Immediate {
                        let hash: usize = op0.immediate.unwrap().into();
                        // jump to the instruction right after the Fn marker (the prologue)
                        fn_table.insert(hash, pc + 1);
                    }
                }
            }
        }

        Ok(Program {
            arch,
            main_start,
            main_hash,
            native_traces,
            instructions,
            fn_table,
        })
    }
}

fn decode_instruction(bytes: &[u8]) -> Instruction {
    let opcode = OpCode::try_from(bytes[0]).unwrap_or(OpCode::Fn);
    let operand_count = bytes[1] as usize;

    let op0 = if operand_count >= 1 {
        Some(decode_operand(&bytes[2..2 + OPERAND_SIZE]))
    } else {
        None
    };
    let op1 = if operand_count >= 2 {
        Some(decode_operand(&bytes[2 + OPERAND_SIZE..2 + OPERAND_SIZE * 2]))
    } else {
        None
    };
    let op2 = if operand_count >= 3 {
        Some(decode_operand(&bytes[2 + OPERAND_SIZE * 2..2 + OPERAND_SIZE * 3]))
    } else {
        None
    };

    Instruction {
        op_code: opcode,
        operand_0: op0,
        operand_1: op1,
        operand_2: op2,
    }
}

fn decode_operand(bytes: &[u8]) -> Operand {
    let mode = AddressingModes::try_from(bytes[0]).unwrap_or(AddressingModes::Implicit);
    let register = Registers::try_from(bytes[1]).ok();
    let immediate_bytes: [u8; TYPE_SIZE] = bytes[2..2 + TYPE_SIZE].try_into().unwrap();
    let immediate = match mode {
        AddressingModes::Implicit => None,
        AddressingModes::Register => None,
        _ => Some(RawType::from_bytes(&immediate_bytes)),
    };

    Operand {
        mode,
        register,
        immediate,
    }
}
