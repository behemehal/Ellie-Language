use std::println;

use super::type_resolver::resolve_type;
use crate::instructions::{self, Instruction};
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::definite::items::setter_call;
use ellie_core::defs::{self, DebugHeader, DebugHeaderType};

impl super::Transpiler for setter_call::SetterCall {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        let debug_header_start = assembler.location();
        let mut dependencies = vec![processed_page.hash];
        dependencies.extend(processed_page.dependencies.iter().map(|d| d.hash));

        let mut instructions = Vec::new();

        resolve_type(
            assembler,
            &self.target,
            instructions::Registers::B,
            &hash,
            Some(dependencies.clone()),
        );

        let mut assignment = match &self.target {
            ellie_core::definite::types::Types::BraceReference(_) => true,
            _ => false,
        };

        let target_last_instruction = assembler.instructions.last().unwrap().clone();

        resolve_type(
            assembler,
            &self.value,
            instructions::Registers::C,
            &hash,
            Some(dependencies.clone()),
        );

        match self.operator {
        ellie_core::definite::types::operator::AssignmentOperators::Assignment => {
            match &target_last_instruction {
                instructions::Instructions::LDB(e) => match e.addressing_mode {
                    instructions::AddressingModes::Absolute(e) => {
                        instructions.push(instructions::Instructions::STC(Instruction::absolute(e)));
                        assembler.instructions.extend(instructions.clone());
                        assignment = true;
                    }
                    instructions::AddressingModes::AbsoluteIndex(pointer_address, index_address) => {
                        instructions.push(instructions::Instructions::STC(Instruction::absolute_index(pointer_address, index_address)));
                        assembler.instructions.extend(instructions.clone());
                    }
                    _ => unreachable!(
                        "Since this is setter its impossible to get a no absolute addressing mode"
                    ),
                },
                _ => unreachable!(),
            }
        },
        ellie_core::definite::types::operator::AssignmentOperators::AdditionAssignment => {
            instructions.push(instructions::Instructions::ADD(Instruction::implicit()));
        },
        ellie_core::definite::types::operator::AssignmentOperators::SubtractionAssignment => {
            instructions.push(instructions::Instructions::SUB(Instruction::implicit()));
        },
        ellie_core::definite::types::operator::AssignmentOperators::MultiplicationAssignment => {
            instructions.push(instructions::Instructions::MUL(Instruction::implicit()));
        },
        ellie_core::definite::types::operator::AssignmentOperators::DivisionAssignment => {
            instructions.push(instructions::Instructions::DIV(Instruction::implicit()));
        },
        ellie_core::definite::types::operator::AssignmentOperators::ModulusAssignment => {
            instructions.push(instructions::Instructions::MOD(Instruction::implicit()));
        },
        ellie_core::definite::types::operator::AssignmentOperators::ExponentiationAssignment => {
            instructions.push(instructions::Instructions::EXP(Instruction::implicit()));
        },
        ellie_core::definite::types::operator::AssignmentOperators::Null => unreachable!(),
    }

        if !assignment {
            match target_last_instruction {
                instructions::Instructions::LDB(e) => match e.addressing_mode {
                    instructions::AddressingModes::Absolute(e) => {
                        instructions
                            .push(instructions::Instructions::STA(Instruction::absolute(e)));
                    }
                    _ => unreachable!(
                        "Since this is setter its impossible to get a no absolute addressing mode"
                    ),
                },
                _ => unreachable!(),
            }
        }

        assembler.instructions.extend(instructions);
        assembler.debug_headers.push(DebugHeader {
            rtype: DebugHeaderType::SetterCall,
            hash: 00009999999,
            module: processed_page.path.clone(),
            name: "@setter".to_string(),
            start_end: (debug_header_start, assembler.location()),
            pos: defs::Cursor {
                range_start: self.target_pos.range_start,
                range_end: self.value_pos.range_end,
            },
        });
        true
    }
}
