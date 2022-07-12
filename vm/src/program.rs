use crate::utils::{self, AddressingModes, AddressingValues, Instructions, ProgramReader};
use ellie_core::{
    defs::PlatformArchitecture,
    raw_type::{RawType, TypeId},
};

#[derive(Debug)]
pub struct ReadInstruction {
    pub instruction: Instructions,
    pub addressing_mode: AddressingModes,
    pub addressing_value: AddressingValues,
    pub op_code: u8,
    pub args: Vec<u8>,
}

#[derive(Debug)]
pub struct Program {
    pub main: usize,
    pub arch: PlatformArchitecture,
    pub instructions: Vec<ReadInstruction>,
}

impl Program {
    pub fn build_from_reader(reader: &mut ProgramReader) -> Result<Self, u8> {
        let arch = match reader.read_u8() {
            Some(byte) => PlatformArchitecture::from_byte(byte),
            None => return Err(0),
        };

        println!(
            "{}[Program]{}: Target arch {}",
            utils::Colors::Yellow,
            utils::Colors::Reset,
            arch
        );

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
        println!(
            "{}[Program]{}: Program starts at {}",
            utils::Colors::Yellow,
            utils::Colors::Reset,
            main
        );

        let mut program = Program {
            main,
            arch,
            instructions: Vec::new(),
        };

        loop {
            let read_instruction = program.read_instruction(reader);
            match read_instruction {
                Ok(instruction) => program.instructions.push(instruction),
                Err(error) => {
                    if error != 0 {
                        panic!("Might be not error {}", error);
                    }
                    break;
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
                        args: vec![],
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
                        AddressingModes::AbsoluteIndex => todo!(),
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
                println!("[VM]: Illegal op code {}", read_byte);
                return Err(1);
            }
        };
    }
}
