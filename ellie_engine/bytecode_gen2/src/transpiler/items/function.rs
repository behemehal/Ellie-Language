use std::println;

use crate::{
    assembler::LocalHeader,
    create_instruction,
    instructions::Instruction,
    opcode::OpCode,
    operand::{AddressingModes, Operand, Registers},
    types::Types,
    utils::{limit_platform_size, usize_to_le_bytes},
};
use ellie_core::{
    definite::items::function,
    defs::{Cursor, DebugHeader, DebugHeaderType},
};

impl super::Transpiler for function::Function {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        _hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        for dependency in &processed_page.dependencies {
            assembler.assemble_dependency(&dependency.hash);
        }

      /*   assembler.add_local(LocalHeader {
            name: self.name.clone(),
            cursor: assembler.location(),
            page_hash: processed_page.hash,
            hash: Some(self.hash),
            reference: None,
            borrowed: None,
        }); */

        println!("function transpile: {}", self.name);

        let instruction1 = create_instruction!(
            //Save previous frame pointer
            OpCode::Push,
            Operand {
                mode: AddressingModes::Register,
                register: Some(Registers::FP),
                immediate: None
            }
        );
        let instruction2 = create_instruction!(
            //Set new frame pointer as current stack pointer
            OpCode::Mov,
            Operand {
                mode: AddressingModes::Register,
                register: Some(Registers::SP),
                immediate: None
            },
            Operand {
                mode: AddressingModes::Register,
                register: Some(Registers::FP),
                immediate: None
            }
        );

        let local_page = assembler.processed

        let local_variable_count 
        //allocate variables space in stack
        let instruction3 = create_instruction!(
            //Allocate space for local variables
            OpCode::Sub,
            Operand {
                mode: AddressingModes::Register,
                register: Some(Registers::SP),
                immediate: None
            },
            Operand {
                mode: AddressingModes::Immediate,
                register: None,
                immediate: Some(self.parameters.len().into())
            }
        );


        /* let instruction3 = create_instruction!(
            OpCode::Jmp,
            Operand {
                mode: AddressingModes::Immediate,
                register: None,
                immediate: Some(10.into())
            }
        ); */

        assembler.instructions.push(instruction1);
        assembler.instructions.push(instruction2);
        assembler.instructions.push(instruction3);

        assembler.assemble_dependency(&self.inner_page_id);

        true
    }
}
