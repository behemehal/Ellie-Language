use alloc::vec;
use ellie_core::definite::items::brk;

use crate::{instruction_table, instructions::Instruction};

impl super::Transpiler for brk::Brk {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        _hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        let mut dependencies = vec![processed_page.hash];
        dependencies.extend(processed_page.dependencies.iter().map(|d| d.hash));

        assembler
            .instructions
            .push(instruction_table::Instructions::BRK(Instruction::implicit()));

        true
    }
}
