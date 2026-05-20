use crate::{
    assembler::LocalHeader,
    create_instruction,
    instructions::Instruction,
    opcode::OpCode,
    operand::{AddressingModes, Operand, Registers},
    transpiler::types::{TypeTranspiler, TypeTranspilerOptions},
    utils::limit_platform_size,
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
        let mut deps = vec![processed_page.hash];
        deps.extend(processed_page.dependencies.iter().map(|d| d.hash));

        let location = assembler.instructions.len();

        {
            let mut opts = TypeTranspilerOptions::new();
            opts.set_assembler(assembler)
                .set_dependencies(deps)
                .set_target_page(hash);
            self.value.transpile(&mut opts);
        }

        // value is now in register A — push to stack to allocate local slot
        assembler.instructions.push(create_instruction!(
            OpCode::Push,
            Operand {
                mode: AddressingModes::Register,
                register: Some(Registers::A),
                immediate: None
            }
        ));

        let stack_pos = assembler.stack_depth as isize;
        assembler.stack_depth += 1;

        assembler.debug_headers.push(DebugHeader {
            rtype: DebugHeaderType::Variable,
            hash: limit_platform_size(self.hash, assembler.platform_attributes.architecture),
            start_end: (location, assembler.instructions.len() - 1),
            module_name: processed_page.path.clone(),
            module_hash: processed_page.hash,
            name: self.name.clone(),
            pos: self.pos,
        });

        assembler.add_local(LocalHeader {
            name: self.name.clone(),
            cursor: stack_pos,
            page_hash: processed_page.hash,
            hash: Some(self.hash),
            reference: None,
            borrowed: None,
        });

        true
    }
}
