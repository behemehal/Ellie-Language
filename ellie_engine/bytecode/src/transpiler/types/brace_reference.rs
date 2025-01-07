use ellie_core::definite::types::brace_reference::BraceReferenceType;

use crate::{
    instruction_table::Instructions,
    instructions::{Instruction, Registers},
};

use super::TypeTranspiler;

impl TypeTranspiler for BraceReferenceType {
    fn transpile(&self, options: &mut super::TypeTranspilerOptions) {
        self.reference.transpile(
            options
                .copy()
                .set_assembler(options.assembler_mut())
                .set_target_register(Registers::B),
        );
        options
            .assembler_mut()
            .instructions
            .push(Instructions::STB(Instruction::implicit()));

        let location_of_pointer = options.assembler().location();

        self.value.transpile(
            options
                .copy()
                .set_assembler(options.assembler_mut())
                .set_target_register(Registers::C),
        );

        options
            .assembler_mut()
            .instructions
            .push(Instructions::STC(Instruction::implicit()));

        let index_pointer = options.assembler().location();

        match options.target_register() {
            Registers::A => options.assembler_mut().instructions.push(Instructions::LDA(
                Instruction::absolute_index(location_of_pointer, index_pointer),
            )),
            Registers::B => options.assembler_mut().instructions.push(Instructions::LDB(
                Instruction::absolute_index(location_of_pointer, index_pointer),
            )),
            Registers::C => options.assembler_mut().instructions.push(Instructions::LDC(
                Instruction::absolute_index(location_of_pointer, index_pointer),
            )),
            Registers::X => options.assembler_mut().instructions.push(Instructions::LDX(
                Instruction::absolute_index(location_of_pointer, index_pointer),
            )),
            Registers::Y => options.assembler_mut().instructions.push(Instructions::LDY(
                Instruction::absolute_index(location_of_pointer, index_pointer),
            )),
        }
    }
}
