use alloc::vec;
use ellie_core::definite::items::ret;

use crate::instructions::{self, Instruction};

use super::type_resolver::resolve_type;

impl super::Transpiler for ret::Ret {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        let mut dependencies = vec![processed_page.hash];
        dependencies.extend(processed_page.dependencies.iter().map(|d| d.hash));

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
            .push(instructions::Instructions::STA(Instruction::implict()));
        true
    }
}
