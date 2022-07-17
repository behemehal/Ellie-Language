use super::type_resolver::resolve_type;
use crate::{
    assembler::{DebugHeader, DebugHeaderType, LocalHeader},
    instructions::{self, Instruction},
};
use alloc::vec;
use ellie_core::definite::items::variable;

impl super::Transpiler for variable::Variable {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        let mut dependencies = vec![processed_page.hash];
        dependencies.extend(processed_page.dependencies.iter().map(|d| d.hash));

        let location = if assembler.instructions.len() == 0 {
            0
        } else {
            assembler.location()
        };

        let resolved_instructions = resolve_type(
            assembler,
            &self.value,
            instructions::Registers::A,
            &hash,
            Some(dependencies),
        );

        assembler.instructions.extend(resolved_instructions);

        assembler
            .instructions
            .push(instructions::Instructions::STA(Instruction::implicit()));

        assembler.debug_headers.push(DebugHeader {
            rtype: DebugHeaderType::Variable,
            hash: self.hash,
            start_end: (location, assembler.location()),
            module: processed_page.path.clone(),
            name: self.name.clone(),
            pos: self.pos,
        });

        assembler.locals.push(LocalHeader {
            name: self.name.clone(),
            cursor: assembler.location(),
            page_hash: processed_page.hash,
            reference: None,
        });

        true
    }
}
