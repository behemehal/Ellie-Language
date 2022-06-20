use crate::{
    assembler::{DebugHeader, DebugHeaderType, LocalHeader},
    instructions::{self, Instruction},
};
use ellie_core::{definite::items::function_parameter, defs::Cursor};

impl super::Transpiler for function_parameter::FunctionParameter {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        assembler.debug_headers.push(DebugHeader {
            id: if assembler.instructions.len() == 0 {
                0
            } else {
                assembler.instructions.len() - 1
            },
            rtype: DebugHeaderType::Variable,
            name: self.name.clone(),
            cursor: Cursor {
                range_start: self.name_pos.range_start,
                range_end: self.rtype_pos.range_end,
            },
        });

        assembler
            .instructions
            .push(instructions::Instructions::STA(Instruction::implict()));

        assembler.locals.push(LocalHeader {
            name: self.name.clone(),
            cursor: assembler.instructions.len() - 1,
            reference: None,
        });
        true
    }
}
