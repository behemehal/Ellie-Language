use super::type_resolver::resolve_type;
use crate::instructions::{self, Instruction};
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::definite::items::setter_call;

impl super::Transpiler for setter_call::SetterCall {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        let mut dependencies = vec![processed_page.hash];
        dependencies.extend(processed_page.dependencies.iter().map(|d| d.hash));

        let mut instructions = Vec::new();

        let target = resolve_type(
            assembler,
            &self.target,
            instructions::Registers::B,
            &hash,
            Some(dependencies.clone()),
        );

        let value = resolve_type(
            assembler,
            &self.value,
            instructions::Registers::C,
            &hash,
            Some(dependencies.clone()),
        );

        instructions.extend(target.clone());
        instructions.extend(value);

        match self.operator {
        ellie_core::definite::types::operator::AssignmentOperators::Assignment => {
            match target.last().unwrap() {
                instructions::Instructions::LDB(e) => match e.addressing_mode {
                    instructions::AddressingModes::Absolute(e) => {
                        instructions.push(instructions::Instructions::STC(Instruction::absolute(e)));
                        assembler.instructions.extend(instructions);
                            return true;
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

        match target.last().unwrap() {
            instructions::Instructions::LDB(e) => match e.addressing_mode {
                instructions::AddressingModes::Absolute(e) => {
                    instructions.push(instructions::Instructions::STA(Instruction::absolute(e)));
                }
                _ => unreachable!(
                    "Since this is setter its impossible to get a no absolute addressing mode"
                ),
            },
            _ => unreachable!(),
        }

        assembler.instructions.extend(instructions);
        true
    }
}
