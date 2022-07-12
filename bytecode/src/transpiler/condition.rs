use alloc::{string::ToString, vec};
use ellie_core::definite::items::condition;

use crate::{
    assembler::LocalHeader,
    instructions::{self, Instruction},
};

use super::type_resolver::resolve_type;

impl super::Transpiler for condition::Condition {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        let _has_ret = self.returns.is_some();
        let mut data_cursor = 0;
        {
            assembler
                .instructions
                .push(instructions::Instructions::LDA(Instruction::immediate(
                    crate::instructions::Types::Void,
                    vec![],
                )));
            assembler
                .instructions
                .push(instructions::Instructions::STA(Instruction::implict()));
            //Register a ret point
            assembler.locals.push(LocalHeader {
                name: "$0".to_string(),
                cursor: assembler.instructions.len() - 1,
                page_hash: processed_page.hash,
                reference: None,
            });
            data_cursor = assembler.instructions.len() - 1;
        }

        let mut dependencies = vec![processed_page.hash];
        dependencies.extend(processed_page.dependencies.iter().map(|d| d.hash));

        for chain in &self.chains {
            if chain.rtype != ellie_core::definite::items::condition::ConditionType::Else {
                let resolved_instructions = resolve_type(
                    assembler,
                    &chain.condition,
                    instructions::Registers::A,
                    &hash,
                    Some(dependencies.clone()),
                );
                assembler.instructions.extend(resolved_instructions);
            }
            assembler
                .instructions
                .push(instructions::Instructions::JMPA(Instruction::absolute(
                    01234,
                )));
            assembler.assemble_dependency(&chain.inner_page_id);
        }

        let mut ret_location = 0;
        {
            assembler
                .instructions
                .push(instructions::Instructions::RET(Instruction::absolute(
                    data_cursor,
                )));
            ret_location = assembler.instructions.len() - 1;
        }
        for _ in &self.chains {
            assembler
                .instructions
                .push(instructions::Instructions::ACP(Instruction::absolute(
                    ret_location,
                )));
            assembler
                .instructions
                .push(instructions::Instructions::JMP(Instruction::absolute(hash)));
        }

        true
    }
}
