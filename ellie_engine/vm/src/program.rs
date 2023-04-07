use crate::{
    instruction_utils::{Instructions, A2B},
    utils::{AddressingModes, AddressingValues, ProgramReader},
};
use alloc::vec::Vec;
use ellie_core::{
    defs::PlatformArchitecture,
    raw_type::{StaticRawType, TypeId},
};

#[derive(Debug, Clone, Copy)]
pub struct ReadInstruction {
    pub instruction: Instructions,
    pub addressing_mode: AddressingModes,
    pub addressing_value: AddressingValues,
    pub op_code: u8,
    pub args: [u8; 8],
}

impl Default for ReadInstruction {
    fn default() -> Self {
        Self {
            instruction: Instructions::A2B(A2B {
                addressing_mode: AddressingModes::Implicit,
            }),
            addressing_mode: AddressingModes::Implicit,
            addressing_value: AddressingValues::Implicit,
            op_code: 0,
            args: [0_u8; 8],
        }
    }
}

#[derive(Debug, Clone)]
pub struct MainProgram {
    pub hash: usize,
    pub start: usize,
    pub length: usize,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub main: MainProgram,
    pub arch: PlatformArchitecture,
    pub instructions: Vec<ReadInstruction>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            main: MainProgram {
                hash: 0,
                start: 0,
                length: 0,
            },
            arch: PlatformArchitecture::B32,
            instructions: Vec::new(),
        }
    }

    /// Generate main struct from function hash.
    /// This function will return an error if the function is not found or the instruction is malformed
    /// Err 1: Wrong addresing value
    /// Err 2: Wrong immediate type
    /// Err 3: Not found
    pub fn generate_main_from_function(&self, target_hash: usize) -> Result<MainProgram, u8> {
        let mut i = 0;
        while i < self.instructions.len() {
            let instruction = self.instructions[i];
            match instruction.instruction {
                Instructions::FN(_) => match instruction.addressing_value {
                    AddressingValues::Immediate(static_raw_type) => {
                        if static_raw_type.type_id.id == 1 {
                            let hash = static_raw_type.to_int();
                            if static_raw_type.to_int() as usize == target_hash {
                                let program_len_instruction = self.instructions[i + 1];
                                let program_len = match program_len_instruction.instruction {
                                    Instructions::STA(_) => {
                                        match program_len_instruction.addressing_value {
                                            AddressingValues::Immediate(e) => e.to_int(),
                                            _ => return Err(2),
                                        }
                                    }
                                    _ => return Err(1),
                                };
                                return Ok(MainProgram {
                                    hash: hash as usize,
                                    start: i,
                                    length: program_len as usize,
                                });
                            }
                        } else {
                            return Err(2);
                        }
                    }
                    _ => return Err(1),
                },
                _ => (),
            }
            i += 1;
        }
        Err(3)
    }

    pub fn build_from_reader(&mut self, reader: &mut ProgramReader) -> Result<(), u8> {
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

        let start = match reader.read_usize(arch.usize_len()) {
            Some(byte) => byte,
            None => return Err(0),
        };

        let end = match reader.read_usize(arch.usize_len()) {
            Some(byte) => byte,
            None => return Err(0),
        };

        let hash = match reader.read_usize(arch.usize_len()) {
            Some(byte) => byte,
            None => return Err(0),
        };
        self.main = MainProgram {
            hash,
            start,
            length: end,
        };
        self.arch = arch;
        loop {
            let read_instruction = self.read_instruction(reader);
            match read_instruction {
                Ok(instruction) => {
                    self.instructions.push(instruction);
                }
                Err(error) => {
                    if error != 0 {
                        return Err(error);
                    } else {
                        break;
                    }
                }
            }
        }
        Ok(())
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
                        args: [0_u8; 8],
                    });
                } else {
                    let mut args: [u8; 8] = [0; 8];
                    match addressing_mode {
                        AddressingModes::Immediate => {
                            let id = reader.read_u8().unwrap();
                            let size = reader.read_usize(self.arch.usize_len()).unwrap();
                            let type_id = TypeId::from(id, size);
                            let mut data: [u8; 8] = [0; 8];
                            for i in 0..8 {
                                data[i] = reader.read_u8().unwrap();
                            }
                            addressing_value =
                                AddressingValues::Immediate(StaticRawType { type_id, data });
                        }
                        AddressingModes::Absolute => {
                            for i in 0..self.arch.usize_len() {
                                let read_byte = match reader.read_u8() {
                                    Some(byte) => byte,
                                    None => return Err(0),
                                };
                                args[i as usize] = read_byte;
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
                        AddressingModes::AbsoluteProperty => {
                            let pointer = match reader.read_usize(self.arch.usize_len()) {
                                Some(byte) => byte,
                                None => return Err(0),
                            };
                            let index = match reader.read_usize(self.arch.usize_len()) {
                                Some(byte) => byte,
                                None => return Err(0),
                            };
                            addressing_value = AddressingValues::AbsoluteProperty(pointer, index);
                        }
                        AddressingModes::AbsoluteStatic => {
                            for i in 0..self.arch.usize_len() {
                                let read_byte = match reader.read_u8() {
                                    Some(byte) => byte,
                                    None => return Err(0),
                                };
                                args[i as usize] = read_byte;
                            }
                            addressing_value = AddressingValues::AbsoluteStatic(
                                usize::from_le_bytes(args.clone().try_into().unwrap()),
                            );
                        }
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
