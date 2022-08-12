use crate::utils::{AddressingModes, AddressingValues, Instructions, ProgramReader};
use alloc::vec::Vec;
use ellie_core::{
    defs::{DebugInfo, PlatformArchitecture},
    raw_type::{RawType, TypeId},
};

#[derive(Debug, Clone)]
pub struct ReadInstruction {
    pub instruction: Instructions,
    pub addressing_mode: AddressingModes,
    pub addressing_value: AddressingValues,
    pub op_code: u8,
    pub args: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub main: (usize, usize),
    pub arch: PlatformArchitecture,
    pub instructions: Vec<ReadInstruction>,
}

impl Program {
    pub fn build_from_reader(reader: &mut ProgramReader) -> Result<Self, u8> {
        let arch = match reader.read_u8() {
            Some(byte) => match PlatformArchitecture::from_byte(byte) {
                Some(e) => e,
                None => return Err(2),
            },
            None => return Err(0),
        };

        let main_exists = match reader.read_u8() {
            Some(byte) => byte,
            None => return Err(0),
        };

        if main_exists == 0 {
            return Err(3);
        }

        let main = match reader.read_usize(arch.usize_len()) {
            Some(byte) => byte,
            None => return Err(0),
        };

        let main_hash = match reader.read_usize(arch.usize_len()) {
            Some(byte) => byte,
            None => return Err(0),
        };

        let mut program = Program {
            main: (main, main_hash),
            arch,
            instructions: Vec::new(),
        };

        loop {
            let read_instruction = program.read_instruction(reader);
            match read_instruction {
                Ok(instruction) => program.instructions.push(instruction),
                Err(error) => {
                    if error != 0 {
                        return Err(error);
                    } else {
                        break;
                    }
                }
            }
        }
        Ok(program)
    }

    /// Read instruction
    /// Returns [`Result<Instruction, u8>`]
    /// ## OK
    /// * [`Instruction`]
    /// ## Err
    /// * [`u8`]
    /// 0 = Failed to read byte
    /// 1 = Used illegal op code
    /// 2 = Used invalid addressing mode
    fn read_instruction<'a>(&self, reader: &mut ProgramReader) -> Result<ReadInstruction, u8> {
        let read_byte = match reader.read_u8() {
            Some(byte) => byte,
            None => return Err(0),
        };

        match Instructions::from(&read_byte) {
            Some(instruction) => {
                let addressing_mode = instruction.addressing_mode();
                let mut addressing_value: AddressingValues = AddressingValues::Implicit;
                if addressing_mode == AddressingModes::Implicit {
                    return Ok(ReadInstruction {
                        instruction,
                        addressing_mode,
                        addressing_value,
                        op_code: read_byte,
                        args: Vec::new(),
                    });
                } else {
                    let mut args: Vec<u8> = Vec::new();
                    match addressing_mode {
                        AddressingModes::Immediate => {
                            let id = reader.read_u8().unwrap();
                            let size = reader.read_usize(self.arch.usize_len()).unwrap();
                            let type_id = TypeId::from(id, size);
                            let mut data = Vec::new();
                            for _ in 0..size {
                                data.push(match reader.read_u8() {
                                    Some(e) => e,
                                    None => return Err(0),
                                })
                            }
                            addressing_value =
                                AddressingValues::Immediate(RawType { type_id, data });
                        }
                        AddressingModes::Absolute => {
                            for _ in 0..self.arch.usize_len() {
                                let read_byte = match reader.read_u8() {
                                    Some(byte) => byte,
                                    None => return Err(0),
                                };
                                args.push(read_byte);
                            }
                            addressing_value = AddressingValues::Absolute(usize::from_le_bytes(
                                args.clone().try_into().unwrap(),
                            ));
                        }
                        AddressingModes::AbsoluteIndex => {
                            let pointer = match reader.read_usize(self.arch.usize_len()) {
                                Some(byte) => byte,
                                None => return Err(0),
                            };
                            let index = match reader.read_usize(self.arch.usize_len()) {
                                Some(byte) => byte,
                                None => return Err(0),
                            };
                            addressing_value = AddressingValues::AbsoluteIndex(pointer, index);
                        }
                        AddressingModes::AbsoluteProperty => todo!(),
                        AddressingModes::Implicit => todo!(),
                        AddressingModes::IndirectA => {
                            addressing_value = AddressingValues::IndirectA;
                        }
                        AddressingModes::IndirectB => {
                            addressing_value = AddressingValues::IndirectB;
                        }
                        AddressingModes::IndirectC => {
                            addressing_value = AddressingValues::IndirectC;
                        }
                        AddressingModes::IndirectX => {
                            addressing_value = AddressingValues::IndirectX;
                        }
                        AddressingModes::IndirectY => {
                            addressing_value = AddressingValues::IndirectY;
                        }
                    }

                    return Ok(ReadInstruction {
                        instruction,
                        addressing_mode,
                        addressing_value,
                        op_code: read_byte,
                        args,
                    });
                }
            }
            None => {
                return Err(1);
            }
        };
    }
}
