use ellie_core::{definite::items::function, defs::Cursor};
use crate::{
    assembler::{DebugHeader, DebugHeaderType, LocalHeader},
    instructions::{Instruction, Instructions},
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

        //Reserve memory spaces for parameters
        for hash in 0..self.parameters.len() {
            assembler.debug_headers.push(DebugHeader {
                rtype: DebugHeaderType::Parameter,
                hash,
                module: processed_page.path.clone(),
                name: self.parameters[hash].name.clone(),
                start_end: (assembler.location(), assembler.location()),
                pos: Cursor { range_start: self.parameters[hash].name_pos.range_start, range_end: self.parameters[hash].rtype_pos.range_end },
            });
            assembler
                .instructions
                .push(Instructions::STA(Instruction::implicit()));
        }
        let debug_header_start = assembler.location();

        assembler.locals.push(LocalHeader {
            name: self.name.clone(),
            cursor: assembler.instructions.len(),
            page_hash: processed_page.hash,
            reference: Some(self.inner_page_id as usize),
        });

        assembler.assemble_dependency(&self.inner_page_id);

        assembler
            .instructions
            .push(Instructions::RET(Instruction::implicit()));
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
