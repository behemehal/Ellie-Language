use std::println;

use alloc::{format, string::ToString, vec, vec::Vec};
use ellie_core::definite::items::condition;

use crate::{
    assembler::{DebugHeader, DebugHeaderType},
    instructions::{self, Instruction, Types},
};

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

        for (idx, chain) in self.chains.iter().enumerate() {
            let mut chain_instructions = Vec::new();
            if chain.rtype != ellie_core::definite::items::condition::ConditionType::Else {
                let resolved_instructions = resolve_type(
                    assembler,
                    &chain.condition,
                    instructions::Registers::A,
                    &hash,
                    Some(dependencies.clone()),
                );
                chain_instructions.extend(resolved_instructions);
            } else {
                chain_instructions.push(instructions::Instructions::LDA(Instruction::immediate(
                    Types::Bool,
                    vec![1],
                )));
            }
            assembler.instructions.extend(chain_instructions);
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

        for (idx, chain) in self.chains.iter().enumerate() {
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

        println!(
            "condition_end_requests: {:?}\ncondition_body_jmp_positions: {:?}\ncondition_body_starts: {:?}\nend: {:?}",
            condition_end_requests, condition_body_start_requests, condition_body_starts, end
        );

        /*
        let mut condition_code_positions = vec![];

        for chain in &self.chains {
            let start_pos = assembler.location() + 1;
            assembler.assemble_dependency(&chain.inner_page_id);
            let end_pos = assembler.location();

            condition_code_positions.push((chain.rtype.clone(), start_pos, end_pos));
        }

        let mut condition_chain_instructions = Vec::new();
        for (idx, chain) in self.chains.iter().enumerate() {
            let condition_pos = condition_code_positions[idx].clone();

            let mut chain_instructions = Vec::new();
            if chain.rtype != ellie_core::definite::items::condition::ConditionType::Else {
                let resolved_instructions = resolve_type(
                    assembler,
                    &chain.condition,
                    instructions::Registers::A,
                    &hash,
                    Some(dependencies.clone()),
                );
                chain_instructions.extend(resolved_instructions);
            } else {
                chain_instructions.push(instructions::Instructions::LDA(Instruction::immediate(
                    Types::Bool,
                    vec![1],
                )));
            }
            assembler.instructions.insert(
                condition_pos.2 + 1 + idx,
                instructions::Instructions::JMP(Instruction::immediate(
                    Types::Null,
                    format!("GOTO:{}", condition_pos.2 + 1).as_bytes().to_vec(),
                )),
            );
            chain_instructions.push(instructions::Instructions::JMPA(Instruction::immediate(
                Types::Null,
                format!("GOTO:{}", condition_pos.2 + 1 + idx)
                    .as_bytes()
                    .to_vec(),
            )));
            condition_chain_instructions.extend(chain_instructions);
        }

        println!(
            "condition_code_positions: \n{:?}\n",
            condition_code_positions
        );

        for (idx, instruction) in condition_chain_instructions.iter().enumerate() {
            assembler.instructions.insert(
                condition_code_positions.first().unwrap().1 + idx,
                instruction.clone(),
            );
        }

        condition_code_positions.iter_mut().enumerate().for_each(
            |(idx, (_, start_pos, end_pos))| {
                *start_pos += condition_chain_instructions.len() + idx;
                *end_pos += condition_chain_instructions.len() + 1 + idx;
            },
        );

        println!(
            "modified_code_positions: \n{:?}\n",
            condition_code_positions
        );

        */
        true
    }
}

/*

let mut chain_instructions = Vec::new();
            if chain.rtype != ellie_core::definite::items::condition::ConditionType::Else {
                let resolved_instructions = resolve_type(
                    assembler,
                    &chain.condition,
                    instructions::Registers::A,
                    &hash,
                    Some(dependencies.clone()),
                );
                chain_instructions.extend(resolved_instructions);
            } else {
                chain_instructions.push(instructions::Instructions::LDA(Instruction::immediate(
                    Types::Bool,
                    vec![1],
                )));
            }
            chain_instructions.push(instructions::Instructions::STA(Instruction::implicit()));
            assembler.instructions.extend(chain_instructions);
            let condition_pos = assembler.location();
            assembler.assemble_dependency(&chain.inner_page_id);
            assembler
                .instructions
                .push(instructions::Instructions::LDA(Instruction::absolute(
                    condition_pos,
                )));
            assembler
                .instructions
                .push(instructions::Instructions::JMPA(Instruction::absolute(
                    condition_pos - 1,
                )));
            assembler.debug_headers.push(DebugHeader {
                rtype: DebugHeaderType::Condition,
                hash: 0,
                module: processed_page.path.clone(),
                name: "<condition>".to_string(),
                start_end: (condition_pos, assembler.location()),
                pos: self.pos,
            });


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
                .push(instructions::Instructions::STA(Instruction::implicit()));
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

    */
