use crate::instructions::{self, Instruction};

use super::type_resolver::resolve_type;
use ellie_core::definite::{items::getter_call, types::Types};

impl super::Transpiler for getter_call::GetterCall {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        match &self.data {
            Types::Collective(_) => todo!(),
            Types::Reference(_) => todo!(),
            Types::BraceReference(_) => todo!(),
            Types::Operator(_) => todo!(),
            Types::Cloak(_) => todo!(),
            Types::Array(_) => todo!(),
            Types::Vector(_) => todo!(),
            Types::Function(_) => todo!(),
            Types::ClassCall(_) => todo!(),
            Types::FunctionCall(function_call) => {
                let target_local = match *function_call.target.clone() {
                    Types::VariableType(e) => e.value,
                    _ => unreachable!(),
                };

                let target = assembler.find_local(&target_local).unwrap().clone();

                assembler
                    .instructions
                    .push(instructions::Instructions::CALL(Instruction::absolute(
                        target.cursor,
                    )));
            }
            Types::SetterCall(_) => todo!(),
            Types::NullResolver(_) => todo!(),
            Types::Negative(_) => todo!(),
            Types::VariableType(_) => todo!(),
            Types::AsKeyword(_) => todo!(),
            Types::Null => todo!(),
            Types::Dynamic => todo!(),
            _ => {
                let resolved_instructions =
                    resolve_type(assembler, &self.data, instructions::Registers::A, &hash);
                todo!("GetterCall: {:?}", resolved_instructions);
            }
            Types::Byte(_) => todo!(),
            Types::Integer(_) => todo!(),
            Types::Float(_) => todo!(),
            Types::Double(_) => todo!(),
            Types::Bool(_) => todo!(),
            Types::String(_) => todo!(),
            Types::Char(_) => todo!(),
            Types::Void => todo!(),
        }
        true
    }
}
