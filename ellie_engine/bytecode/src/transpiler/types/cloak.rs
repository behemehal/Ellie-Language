use super::TypeTranspiler;
use crate::{
    instruction_table::Instructions,
    instructions::{Instruction, Registers},
    types::Types as ByteCodeTypes,
    utils::usize_to_le_bytes,
};
use ellie_core::definite::types::cloak::CloakType;

impl TypeTranspiler for CloakType {
    fn transpile(&self, options: &mut super::TypeTranspilerOptions) {
        let mut size = 0;

        for entry in &self.collective {
            entry.value.transpile(
                options
                    .copy()
                    .set_assembler(options.assembler_mut())
                    .set_target_register(Registers::A),
            );
            options
                .assembler_mut()
                .instructions
                .push(Instructions::STA(Instruction::implicit()));
            size += 1;
        }

        let arch = options.assembler().platform_attributes.architecture;

        options
            .assembler_mut()
            .instructions
            .push(Instructions::SAR(Instruction::immediate(
                ByteCodeTypes::StaticArray,
                usize_to_le_bytes(size, arch),
            )));

        let location = options.assembler().location();

        match options.target_register() {
            Registers::A => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDA(Instruction::absolute(location))),
            Registers::B => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDB(Instruction::absolute(location))),
            Registers::C => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDC(Instruction::absolute(location))),
            Registers::X => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDX(Instruction::absolute(location))),
            Registers::Y => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDY(Instruction::absolute(location))),
        }
    }
}
