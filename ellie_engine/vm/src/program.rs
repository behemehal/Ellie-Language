use crate::{
    config::PROGRAM_MAX_SIZE,
    instruction_utils::{Instructions, A2B},
    raw_type::{StaticRawType, TypeId},
    utils::{AddressingModes, AddressingValues, ProgramReader},
};
use alloc::vec::Vec;
use core::mem;
use ellie_core::defs::PlatformArchitecture;

#[derive(Debug, Clone, Copy)]
pub struct ReadInstruction {
    pub instruction: Instructions,
    pub addressing_mode: AddressingModes,
    pub addressing_value: AddressingValues,
    pub op_code: u8,
}

#[derive(PartialEq, Debug)]
pub enum ProgramReadErrors {
    ReadError,
    UnexpectedPlatformArchitecture,
    UnmatchedPlatformArchitecture(PlatformArchitecture, PlatformArchitecture),
    NoMainFunction,
    BrokenMainFunction,
    IllegalOpCode,
    Complete,
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
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

    pub fn build_from_reader(
        &mut self,
        reader: &mut ProgramReader,
    ) -> Result<(), ProgramReadErrors> {
        let arch = match reader.read_u8() {
            Some(byte) => match PlatformArchitecture::from_byte(byte) {
                Some(e) => e,
                None => return Err(ProgramReadErrors::UnexpectedPlatformArchitecture),
            },
            None => return Err(ProgramReadErrors::ReadError),
        };

        if arch.usize_len() > mem::size_of::<usize>() as u8 {
            return Err(ProgramReadErrors::UnmatchedPlatformArchitecture(
                arch,
                PlatformArchitecture::from_byte(mem::size_of::<usize>() as u8 * 8).unwrap(),
            ));
        }

        let main_exists = match reader.read_u8() {
            Some(byte) => byte,
            None => return Err(ProgramReadErrors::ReadError),
        };

        if main_exists == 0 {
            return Err(ProgramReadErrors::NoMainFunction);
        }

        let start = match reader.read_usize(arch.usize_len()) {
            Some(byte) => byte,
            None => return Err(ProgramReadErrors::ReadError),
        };

        let end = match reader.read_usize(arch.usize_len()) {
            Some(byte) => byte,
            None => return Err(ProgramReadErrors::ReadError),
        };

        let hash = match reader.read_usize(arch.usize_len()) {
            Some(byte) => byte,
            None => return Err(ProgramReadErrors::ReadError),
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
                    if self.instructions.len() - 1 == self.main.length {
                        break;
                    }
                }
                Err(error) => {
                    return Err(error);
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
    fn read_instruction<'a>(
        &self,
        reader: &mut ProgramReader,
    ) -> Result<ReadInstruction, ProgramReadErrors> {
        let read_byte = match reader.read_u8() {
            Some(byte) => byte,
            None => return Err(ProgramReadErrors::ReadError),
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
                    });
                } else {
                    match addressing_mode {
                        AddressingModes::Immediate => {
                            let id = reader.read_u8().unwrap();
                            let size = reader.read_usize(self.arch.usize_len()).unwrap();
                            let type_id = TypeId::from(id, size);
                            let mut data: [u8; 8] = [0; 8];
                            for i in 0..type_id.size {
                                data[i] = reader.read_u8().unwrap();
                            }
                            addressing_value =
                                AddressingValues::Immediate(StaticRawType { type_id, data });
                        }
                        AddressingModes::Absolute => {
                            let address = match reader.read_usize(self.arch.usize_len()) {
                                Some(byte) => byte,
                                None => return Err(ProgramReadErrors::ReadError),
                            };
                            addressing_value = AddressingValues::Absolute(address);
                        }
                        AddressingModes::AbsoluteIndex => {
                            let pointer = match reader.read_usize(self.arch.usize_len()) {
                                Some(byte) => byte,
                                None => return Err(ProgramReadErrors::ReadError),
                            };
                            let index = match reader.read_usize(self.arch.usize_len()) {
                                Some(byte) => byte,
                                None => return Err(ProgramReadErrors::ReadError),
                            };
                            addressing_value = AddressingValues::AbsoluteIndex(pointer, index);
                        }
                        AddressingModes::AbsoluteProperty => {
                            let pointer = match reader.read_usize(self.arch.usize_len()) {
                                Some(byte) => byte,
                                None => return Err(ProgramReadErrors::ReadError),
                            };
                            let index = match reader.read_usize(self.arch.usize_len()) {
                                Some(byte) => byte,
                                None => return Err(ProgramReadErrors::ReadError),
                            };
                            addressing_value = AddressingValues::AbsoluteProperty(pointer, index);
                        }
                        AddressingModes::AbsoluteStatic => {
                            let address = match reader.read_usize(self.arch.usize_len()) {
                                Some(byte) => byte,
                                None => return Err(ProgramReadErrors::ReadError),
                            };
                            addressing_value = AddressingValues::AbsoluteStatic(address);
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
                    });
                }
            }
            None => {
                return Err(ProgramReadErrors::IllegalOpCode);
            }
        };
    }
}

pub struct VmProgram {
    pub instructions: [ReadInstruction; PROGRAM_MAX_SIZE],
    pub length: usize,
}

impl VmProgram {
    pub fn new() -> Self {
        VmProgram {
            instructions: [ReadInstruction::default(); PROGRAM_MAX_SIZE],
            length: 0,
        }
    }

    pub fn new_from_vector(program: Vec<ReadInstruction>) -> Self {
        let mut vm_program = VmProgram::new();
        vm_program.fill_from_vector(program);
        vm_program
    }

    pub fn fill_from_vector(&mut self, program: Vec<ReadInstruction>) {
        for (idx, instruction) in program.iter().enumerate() {
            self.instructions[idx] = *instruction;
        }
        self.length = program.len();
    }
}
