use crate::utils::{
    build_immediate, AddressingModes, AddressingValues, Instructions, ProgramReader, Reader,
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
    pub(crate) arch: u8,
    pub(crate) instructions: Vec<ReadInstruction>,
}

impl Program {
    pub fn build_from_reader(reader: &mut ProgramReader) -> Result<Self, u8> {
        let arch = match reader.read_u8() {
            Some(byte) => byte,
            None => return Err(0),
        };

        println!("[Program]: Target arch {}", arch);

        let mut program = Program {
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
                    let size = self.arch / 8;
                    match addressing_mode {
                        AddressingModes::Immediate => {
                            let type_defination_size = reader.read_usize(size).unwrap();
                            let mut chains: Vec<[u8; 2]> = Vec::new();
                            for _ in 0..type_defination_size {
                                let type_1 = match reader.read_u8() {
                                    Some(byte) => byte,
                                    None => return Err(1),
                                };
                                let type_2 = match reader.read_u8() {
                                    Some(byte) => byte,
                                    None => return Err(1),
                                };
                                chains.push([type_1, type_2]);
                            }

                            for _ in 0..size {
                                let read_byte = match reader.read_u8() {
                                    Some(byte) => byte,
                                    None => return Err(0),
                                };
                                args.push(read_byte);
                            }

                            let value = isize::from_le_bytes(args.clone().try_into().unwrap());
                            //TODO make this safe remove .unwrap
                            addressing_value =
                                build_immediate(type_defination_size, chains, value).unwrap();
                        }
                        AddressingModes::Absolute => {
                            for _ in 0..size {
                                let read_byte = match reader.read_u8() {
                                    Some(byte) => byte,
                                    None => return Err(0),
                                };
                                args.push(read_byte);
                            }
                            addressing_value = AddressingValues::Absolute(usize::from_le_bytes(args.clone().try_into().unwrap()));
                        }
                        AddressingModes::AbsoluteIndex => todo!(),
                        AddressingModes::AbsoluteProperty => todo!(),
                        _ => println!("[VM]: Ignore Indirect[?] modes"),
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
