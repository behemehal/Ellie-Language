use super::type_resolver::resolve_type;
use crate::{
    assembler::LocalHeader,
    instructions::{self, Instruction},
};
use ellie_core::definite::items::variable;

impl super::Transpiler for variable::Variable {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        let resolved_instructions =
            resolve_type(assembler, &self.value, instructions::Registers::A, &hash);

        assembler.instructions.extend(resolved_instructions);

        assembler
            .instructions
            .push(instructions::Instructions::STA(Instruction::implict()));

        assembler.locals.push(LocalHeader {
            name: self.name.clone(),
            cursor: assembler.instructions.len() - 1,
            reference: None,
        });

        true
    }
}
