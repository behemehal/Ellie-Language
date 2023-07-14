use alloc::{string::ToString, vec};
use ellie_core::{
    definite::items::loop_type,
    defs::{DebugHeader, DebugHeaderType},
};

use crate::{
    instruction_table,
    instructions::{self, Instruction},
    utils::limit_platform_size,
};

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

        let dependencies = vec![processed_page.hash];

        //We need to get back at this point every time we jump to the end of the loop.
        let start_pos = assembler.location() + 1;

        resolve_type(
            assembler,
            &self.condition,
            instructions::Registers::A,
            &hash,
            Some(dependencies),
        );

        assembler
            .instructions
            .push(instruction_table::Instructions::JMPA(
                Instruction::absolute(
                    assembler.location() + 3, //Skip the next JMP instruction to the loop's body start
                ),
            ));

        //If previous instruction which is JMPA is not executed, we need to jump to the end of the loop. to end it
        assembler
            .instructions
            .push(instruction_table::Instructions::JMP(Instruction::absolute(
                144,
            ))); //To be changed to loops exit point

        //We're saving location of JMP instruction, because end of the loop location is unknown until body is assembled.
        let escaper_pos = assembler.location();

        assembler.assemble_dependency(&self.inner_page_id);

        assembler
            .instructions
            .push(instruction_table::Instructions::JMP(Instruction::absolute(
                start_pos,
            ))); //Jump to the start of the loop again

        let end_of_loop_pos = assembler.location() + 1;

        //set the escaper position to the end of the loop
        assembler.instructions[escaper_pos] =
            instruction_table::Instructions::JMP(Instruction::absolute(end_of_loop_pos));

        assembler.debug_headers.push(DebugHeader {
            rtype: DebugHeaderType::Condition,
            hash: limit_platform_size(self.hash, assembler.platform_attributes.architecture),
            module: processed_page.path.clone(),
            name: "<loop>".to_string(),
            start_end: (start_pos, assembler.location()),
            pos: self.pos,
        });
        true
    }
}
