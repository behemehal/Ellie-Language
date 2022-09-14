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

        resolve_type(
            assembler,
            &self.value,
            instructions::Registers::Y,
            &hash,
            Some(dependencies),
        );
        std::println!("RET: {:?}", self.value);
        std::println!("RET: {:?}", assembler.instructions.last());
        std::println!("RET: {:?}", assembler.instructions.last().unwrap().get_arg(&ellie_core::defs::PlatformArchitecture::B32));
        std::println!("RET: {:?}", assembler.instructions.last().unwrap().get_addressing_mode());
        std::println!("RET: {:?}", assembler.instructions.last().unwrap().op_code(&ellie_core::defs::PlatformArchitecture::B32));

        assembler
            .instructions
            .push(instructions::Instructions::RET(Instruction::implicit()));
        true
    }
}
