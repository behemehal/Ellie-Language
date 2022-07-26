use alloc::{string::ToString, vec};
use ellie_core::{
    definite::items::loop_type,
    defs::{DebugHeader, DebugHeaderType},
};

use crate::instructions::{self, Instruction};

use super::type_resolver::resolve_type;

impl super::Transpiler for loop_type::Loop {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        for dependency in &processed_page.dependencies {
            assembler.assemble_dependency(&dependency.hash);
        }

        let mut dependencies = vec![processed_page.hash];
        dependencies.extend(processed_page.dependencies.iter().map(|d| d.hash));
        let mut instructions = resolve_type(
            assembler,
            &self.condition,
            instructions::Registers::A,
            &hash,
            Some(dependencies),
        );

        instructions.push(instructions::Instructions::STA(Instruction::implicit()));
        assembler.instructions.extend(instructions);
        let condition_pos = assembler.location();
        assembler.assemble_dependency(&self.inner_page_id);
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
            rtype: DebugHeaderType::Function,
            hash: self.hash,
            module: processed_page.path.clone(),
            name: "<loop>".to_string(),
            start_end: (condition_pos, assembler.location()),
            pos: self.pos,
        });
        true
    }
}
