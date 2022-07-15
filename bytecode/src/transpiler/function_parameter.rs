use crate::assembler::LocalHeader;
use ellie_core::definite::items::function_parameter;

impl super::Transpiler for function_parameter::FunctionParameter {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        _hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        //assembler
        //    .instructions
        //    .push(instructions::Instructions::STA(Instruction::implicit()));

        assembler.locals.push(LocalHeader {
            name: self.name.clone(),
            cursor: assembler.instructions.len() - 1,
            page_hash: processed_page.hash,
            reference: None,
        });
        true
    }
}
