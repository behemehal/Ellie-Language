use ellie_core::definite::items::class;

use crate::{assembler::LocalHeader, instructions::Instruction};

impl super::Transpiler for class::Class {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        _hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        for dependency in &processed_page.dependencies {
            assembler.assemble_dependency(&dependency.hash);
        }

        assembler.locals.push(LocalHeader {
            name: self.name.clone(),
            cursor: assembler.location(),
            page_hash: processed_page.hash,
            reference: Instruction::absolute(assembler.location()),
        });

        assembler.assemble_dependency(&self.inner_page_id);
        true
    }
}
