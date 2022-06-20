use ellie_core::definite::items::function;

use crate::{
    assembler::LocalHeader,
    instructions::{Instruction, Instructions},
};

impl super::Transpiler for function::Function {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        for dependency in &processed_page.dependencies {
            assembler.assemble_dependency(&dependency.hash);
        }

        assembler.locals.push(LocalHeader {
            name: self.name.clone(),
            cursor: assembler.instructions.len(),
            reference: Some(self.inner_page_id as usize),
        });

        assembler.assemble_dependency(&self.inner_page_id);

        assembler
            .instructions
            .push(Instructions::RET(Instruction::implict()));

        true
    }
}
