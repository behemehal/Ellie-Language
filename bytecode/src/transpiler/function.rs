use std::println;

use crate::{
    assembler::{self, LocalHeader},
    instructions::{self, AddressingModes, Instruction, Instructions},
};
use alloc::vec;
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

        assembler.locals.push(LocalHeader {
            name: self.name.clone(),
            cursor: assembler.location(),
            page_hash: processed_page.hash,
            reference: Instruction::absolute(assembler.location()),
        });

        //let mut array_pointers = Vec::new();
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

            assembler.instructions.push(Instructions::LDA(Instruction::function_parameter(hash)));
            assembler.instructions.push(Instructions::STA(Instruction::implicit()));
            /*
            array_pointers.extend(vec![
                0;
                assembler.platform_attributes.architecture.usize_len()
                    as usize
            ]);
            */

            assembler.locals.push(LocalHeader {
                name: self.parameters[hash].name.clone(),
                page_hash: processed_page.hash,
                cursor: assembler.location(),
                reference: Instruction::function_parameter(hash),
            });
        }

        /*
        //Get the parameters from heap
        assembler
            .instructions
            .push(Instructions::LDX(Instruction::absolute(
                assembler.location(),
            )));
            */

        let debug_header_start = if assembler.instructions.len() == 0 {
            0
        } else {
            assembler.location()
        };

        assembler.assemble_dependency(&self.inner_page_id);

        if self.no_return {
            assembler
                .instructions
                .push(Instructions::RET(Instruction::implicit()));
        }

        let mut hash = self.hash.to_le_bytes().to_vec();
        hash.extend_from_slice(&(assembler.instructions.len()).to_le_bytes());
        //hash.extend_from_slice(
        //    &(assembler.location() - debug_header_start).to_le_bytes()
        //);

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
