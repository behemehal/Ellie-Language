use alloc::vec;
use ellie_core::definite::items::ret;

use crate::{
    instruction_table,
    instructions::{self, Instruction},
    transpiler::types::{TypeTranspiler, TypeTranspilerOptions},
};

impl super::Transpiler for ret::Ret {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        let mut dependencies = vec![processed_page.hash];
        dependencies.extend(processed_page.dependencies.iter().map(|d| d.hash));

        let mut binding = TypeTranspilerOptions::new();
        let type_transpiler_options = binding
            .set_assembler(assembler)
            .set_target_register(instructions::Registers::Y)
            .set_dependencies(dependencies)
            .set_target_page(hash);

        self.value.transpile(type_transpiler_options);

        assembler
            .instructions
            .push(instruction_table::Instructions::STY(Instruction::implicit()));

        assembler
            .instructions
            .push(instruction_table::Instructions::LDY(Instruction::absolute(
                assembler.location(),
            )));

        assembler
            .instructions
            .push(instruction_table::Instructions::RET(Instruction::implicit()));

        true
    }
}
