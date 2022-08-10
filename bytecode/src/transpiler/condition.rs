use alloc::{vec, vec::Vec};
use ellie_core::definite::items::condition;

use crate::instructions::{self, Instruction, Types};

use super::type_resolver::resolve_type;

impl super::Transpiler for condition::Condition {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        let mut dependencies = vec![processed_page.hash];
        dependencies.extend(processed_page.dependencies.iter().map(|d| d.hash));

        let mut condition_end_requests = Vec::new();
        let mut condition_body_starts = Vec::new();
        let mut condition_body_start_requests = Vec::new();

        for (_, chain) in self.chains.iter().enumerate() {
            if chain.rtype != ellie_core::definite::items::condition::ConditionType::Else {
                resolve_type(
                    assembler,
                    &chain.condition,
                    instructions::Registers::A,
                    &hash,
                    Some(dependencies.clone()),
                );
            } else {
                assembler.instructions.push(instructions::Instructions::LDA(
                    Instruction::immediate(Types::Bool, vec![1]),
                ));
            }
            assembler
                .instructions
                .push(instructions::Instructions::JMPA(Instruction::absolute(144)));
            condition_body_start_requests.push(assembler.location());
        }

        if self.chains.last().unwrap().rtype != condition::ConditionType::Else {
            assembler
                .instructions
                .push(instructions::Instructions::JMP(Instruction::absolute(133)));
            condition_end_requests.push(assembler.location());
        }

        for (_, chain) in self.chains.iter().enumerate() {
            let start = assembler.location() + 1;
            condition_body_starts.push(start);
            assembler.assemble_dependency(&chain.inner_page_id);
            //if idx != self.chains.len() - 1 {
            assembler
                .instructions
                .push(instructions::Instructions::JMPA(Instruction::absolute(123)));
            let end = assembler.location();
            condition_end_requests.push(end);
            //}
        }
        let end = assembler.location() + 1;

        for location in &condition_end_requests {
            if assembler.instructions[*location]
                == instructions::Instructions::JMPA(Instruction::absolute(133))
            {
                assembler.instructions[*location] =
                    instructions::Instructions::JMPA(Instruction::absolute(end));
            } else {
                assembler.instructions[*location] =
                    instructions::Instructions::JMP(Instruction::absolute(end));
            }
        }

        for (idx, location) in condition_body_starts.iter().enumerate() {
            let pos = condition_body_start_requests[idx];
            assembler.instructions[pos] =
                instructions::Instructions::JMPA(Instruction::absolute(*location));
        }
        true
    }
}
