use ellie_core::definite::items::function;

use crate::{
    assembler::LocalHeader,
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
        for _ in &self.parameters {
            assembler
                .instructions
                .push(Instructions::STA(Instruction::implict()))
        }

        assembler.locals.push(LocalHeader {
            name: self.name.clone(),
            cursor: assembler.instructions.len(),
            page_hash: processed_page.hash,
            reference: Some(self.inner_page_id as usize),
        });

        assembler.assemble_dependency(&self.inner_page_id);

        assembler
            .instructions
            .push(Instructions::RET(Instruction::implict()));

        true
    }
}
