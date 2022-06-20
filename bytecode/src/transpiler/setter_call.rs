use crate::instructions::{self, Instruction};

use super::type_resolver::resolve_type;
use alloc::vec::Vec;
use ellie_core::definite::{items::setter_call, types::Types};

impl super::Transpiler for setter_call::SetterCall {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        match &self.target {
            Types::Reference(_) => todo!(),
            Types::BraceReference(_) => todo!(),
            Types::VariableType(e) => {
                let mut instructions = Vec::new();
                let resolved_instructions =
                    resolve_type(assembler, &self.value, instructions::Registers::A, &hash);

                let target = assembler.find_local(&e.value).unwrap();

                instructions.extend(resolved_instructions);
                if let Some(reference) = target.reference {
                    instructions.push(instructions::Instructions::AOL(Instruction::absolute(
                        reference,
                    )));
                }

                instructions.push(instructions::Instructions::STA(Instruction::absolute(
                    target.cursor,
                )));
                assembler.instructions.extend(instructions)
            }
            _ => unreachable!("Invalid left-side of assignment"),
        }
        true
    }
}
