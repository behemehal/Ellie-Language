use crate::{
    assembler::LocalHeader,
    instruction_table::{self, Instructions},
    instructions::Instruction,
    types::Types,
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

        assembler
            .instructions
            .push(instruction_table::Instructions::FN(Instruction::immediate(
                Types::Integer,
                self.hash.to_le_bytes(),
            ))); //Function hash

        assembler.locals.push(LocalHeader {
            name: self.name.clone(),
            cursor: assembler.location(),
            page_hash: processed_page.hash,
            hash: Some(self.hash),
            reference: Instruction::absolute_static(assembler.location()),
        });

        assembler
            .instructions
            .push(instruction_table::Instructions::STA(Instruction::absolute(
                144,
            ))); //Escape pos

        let escape_pos_instruction_location = assembler.location();

        assembler
            .instructions
            .push(instruction_table::Instructions::STA(
                Instruction::immediate(Types::Integer, self.parameters.len().to_le_bytes()),
            ));
        //Reserve memory spaces for parameters
        for (idx, parameter) in self.parameters.iter().enumerate() {
            assembler.debug_headers.push(DebugHeader {
                rtype: DebugHeaderType::Parameter,
                hash: idx,
                module: processed_page.path.clone(),
                name: parameter.name.clone(),
                start_end: (assembler.location(), assembler.location()),
                pos: Cursor {
                    range_start: parameter.name_pos.range_start,
                    range_end: parameter.rtype_pos.range_end,
                },
            });

            assembler
                .instructions
                .push(Instructions::STA(Instruction::implicit()));
            assembler.locals.push(LocalHeader {
                name: self.parameters[idx].name.clone(),
                page_hash: processed_page.hash,
                cursor: assembler.location(),
                hash: None,
                reference: Instruction::absolute(assembler.location()),
            });
        }

        let debug_header_start = if assembler.instructions.len() == 0 {
            0
        } else {
            assembler.location()
        };

        assembler.assemble_dependency(&self.inner_page_id);

        assembler
            .instructions
            .push(Instructions::RET(Instruction::implicit()));

        assembler.instructions[escape_pos_instruction_location] = instruction_table::Instructions::STA(
            Instruction::immediate(Types::Integer, assembler.location().to_le_bytes()),
        );

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
