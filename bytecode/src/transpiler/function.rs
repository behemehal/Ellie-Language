use std::println;

use crate::{
    assembler::{self, LocalHeader},
    instructions::{self, AddressingModes, Instruction, Instructions},
};
use alloc::vec::Vec;
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

        let start = assembler.location();
        //Skip to the end of the function
        assembler
            .instructions
            .push(instructions::Instructions::FN(Instruction::absolute(144)));

        //Reserve memory spaces for parameters
        for hash in 0..self.parameters.len() {
            assembler.debug_headers.push(DebugHeader {
                rtype: DebugHeaderType::Parameter,
                hash,
                module: processed_page.path.clone(),
                name: self.parameters[hash].name.clone(),
                start_end: (assembler.location(), assembler.location()),
                pos: Cursor {
                    range_start: self.parameters[hash].name_pos.range_start,
                    range_end: self.parameters[hash].rtype_pos.range_end,
                },
            });

            assembler
                .instructions
                .push(Instructions::STA(Instruction::implicit()));
            assembler.locals.push(LocalHeader {
                name: self.parameters[hash].name.clone(),
                cursor: assembler.location(),
                page_hash: processed_page.hash,
                reference: None,
            });
        }
        let debug_header_start = if assembler.instructions.len() == 0 {
            0
        } else {
            assembler.location()
        };

        assembler.locals.push(LocalHeader {
            name: self.name.clone(),
            cursor: assembler.instructions.len(),
            page_hash: processed_page.hash,
            reference: Some(self.inner_page_id as usize),
        });

        let inner_body_start = assembler.instructions.len();

        assembler.assemble_dependency(&self.inner_page_id);

        let inner_body_end = assembler.instructions.len();

        //Search for a call for debug_header_start in inner_body_start to inner_body_end

        // If function has parameter its likely to be calling himself.
        // If it calls himself it probably will change current parameter values
        // So we need to save them before calling himself

        if self.parameters.len() != 0 {
            //Save current parameter values
            let self_calls: Vec<usize> = assembler.instructions[inner_body_start..inner_body_end]
                .iter()
                .enumerate()
                .filter_map(|(idx, element)| match element {
                    Instructions::CALL(instruction) => match &instruction.addressing_mode {
                        AddressingModes::Absolute(addr) => {
                            if *addr == debug_header_start {
                                Some(idx)
                            } else {
                                None
                            }
                        }
                        _ => unreachable!(),
                    },
                    _ => None,
                })
                .collect();

            for (call_idx, self_call) in assembler.instructions[inner_body_start..inner_body_end]
                .iter()
                .enumerate()
            {
                match self_call {
                    Instructions::CALL(instruction) => match &instruction.addressing_mode {
                        AddressingModes::Absolute(addr) => {
                            if *addr == debug_header_start + 1 {
                                for (idx, parameter) in self.parameters.iter().enumerate() {
                                    let instruction_idx = addr - (self.parameters.len() - idx);
                                    let assembler_instruction =
                                        &assembler.instructions[instruction_idx];
                                    println!("Call: ${addr}: param: ${:?}: ", assembler_instruction)
                                }
                            }
                        }
                        _ => unreachable!(),
                    },
                    _ => (),
                }
            }
        }

        assembler
            .instructions
            .push(Instructions::RET(Instruction::implicit()));

        let mut hash = self.hash.to_le_bytes().to_vec();
        hash.extend_from_slice(&(assembler.instructions.len()).to_le_bytes());

        assembler.instructions[start + 1] = Instructions::FN(Instruction::immediate(
            instructions::Types::String(hash.len()),
            hash.to_vec(),
        ));

        assembler.debug_headers.push(DebugHeader {
            rtype: DebugHeaderType::Function,
            hash: self.hash,
            module: processed_page.path.clone(),
            name: self.name.clone(),
            start_end: (debug_header_start, assembler.location()),
            pos: self.pos,
        });
        true
    }
}
