use crate::{
    create_instruction,
    instructions::Instruction,
    opcode::OpCode,
    operand::{AddressingModes, Operand, Registers},
    transpiler::types::{TypeTranspiler, TypeTranspilerOptions},
};
use alloc::vec;
use ellie_core::definite::{items::ret, types::Types};

impl super::Transpiler for ret::Ret {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        let mut deps = vec![processed_page.hash];
        deps.extend(processed_page.dependencies.iter().map(|d| d.hash));

        // Transpile return value into A (skip for void/null)
        if !matches!(self.value, Types::Void | Types::Null) {
            let mut opts = TypeTranspilerOptions::new();
            opts.set_assembler(assembler)
                .set_dependencies(deps)
                .set_target_page(hash);
            self.value.transpile(&mut opts);
        }

        // Epilogue: discard locals (Mov SP, FP), restore FP, return
        assembler.instructions.push(create_instruction!(
            OpCode::Mov,
            Operand {
                mode: AddressingModes::Register,
                register: Some(Registers::SP),
                immediate: None
            },
            Operand {
                mode: AddressingModes::Register,
                register: Some(Registers::FP),
                immediate: None
            }
        ));
        assembler.instructions.push(create_instruction!(
            OpCode::Pop,
            Operand {
                mode: AddressingModes::Register,
                register: Some(Registers::FP),
                immediate: None
            }
        ));
        assembler.instructions.push(create_instruction!(OpCode::Ret));

        true
    }
}
