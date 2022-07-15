use super::type_resolver::resolve_type;
use crate::instructions;
use alloc::vec;
use ellie_core::definite::items::getter_call;

impl super::Transpiler for getter_call::GetterCall {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        let mut dependencies = vec![processed_page.hash];
        dependencies.extend(processed_page.dependencies.iter().map(|d| d.hash));
        let instructions = resolve_type(
            assembler,
            &self.data,
            instructions::Registers::A,
            &hash,
            Some(dependencies),
        );
        assembler.instructions.extend(instructions);
        true
    }
}
