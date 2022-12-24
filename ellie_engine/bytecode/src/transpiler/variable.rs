use super::type_resolver::resolve_type;
use crate::{
    assembler::LocalHeader,
    instructions::{self, Instruction},
};
use alloc::vec;
use ellie_core::{
    definite::items::variable,
    defs::{DebugHeader, DebugHeaderType},
};

impl super::Transpiler for variable::Variable {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        let mut dependencies = vec![processed_page.hash];
        dependencies.extend(processed_page.dependencies.iter().map(|d| d.hash));

        let location = if assembler.instructions.len() == 0 {
            0
        } else {
            assembler.location()
        };

        let first_instruction_index = assembler.instructions.len();

        resolve_type(
            assembler,
            &self.value,
            instructions::Registers::A,
            &hash,
            Some(dependencies),
        );

        if self.constant {
            assembler.instructions[first_instruction_index] = instructions::Instructions::STA(
                match assembler.instructions[first_instruction_index].clone() {
                    instructions::Instructions::LDA(e) => e,
                    _ => panic!("Constant variable must have ben a LDA"),
                },
            );
            assembler.debug_headers.push(DebugHeader {
                rtype: DebugHeaderType::Variable,
                hash: self.hash,
                start_end: (location, assembler.location()),
                module: processed_page.path.clone(),
                name: self.name.clone(),
                pos: self.pos,
            });

            assembler.locals.push(LocalHeader {
                name: self.name.clone(),
                cursor: assembler.location(),
                page_hash: processed_page.hash,
                hash: Some(self.hash),
                reference: Instruction::absolute(assembler.location()),
            });
            return true;
        }

        assembler
            .instructions
            .push(instructions::Instructions::STA(Instruction::implicit()));

        assembler.debug_headers.push(DebugHeader {
            rtype: DebugHeaderType::Variable,
            hash: self.hash,
            start_end: (location, assembler.location()),
            module: processed_page.path.clone(),
            name: self.name.clone(),
            pos: self.pos,
        });

        assembler.locals.push(LocalHeader {
            name: self.name.clone(),
            cursor: assembler.location(),
            page_hash: processed_page.hash,
            hash: Some(self.hash),
            reference: Instruction::absolute(assembler.location()),
        });

        true
    }
}
