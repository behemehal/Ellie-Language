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
        let static_array_location = options.assembler().location() + 1;
        let arch = options.assembler().platform_attributes.architecture;
        options
            .assembler_mut()
            .instructions
            .push(Instructions::SAR(Instruction::immediate(
                ByteCodeTypes::StaticArray,
                usize_to_le_bytes(static_array_location, arch),
            )));
        options
            .assembler_mut()
            .instructions
            .push(Instructions::STA(Instruction::immediate(
                ByteCodeTypes::Integer,
                usize_to_le_bytes(self.collective.len(), arch),
            )));
        let index_start = options.assembler().location();
        for _ in &self.collective {
            options
                .assembler_mut()
                .instructions
                .push(Instructions::STA(Instruction::implicit()));
        }
        for (index, entry) in self.collective.iter().enumerate() {
            entry.value.transpile(
                options
                    .copy()
                    .set_assembler(options.assembler_mut())
                    .set_target_register(Registers::A),
            );

            options
                .assembler_mut()
                .instructions
                .push(Instructions::STA(Instruction::absolute(
                    (index_start + 1) + index,
                )));
        }
        match options.target_register() {
            Registers::A => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDA(Instruction::absolute(index_start - 1))),
            Registers::B => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDB(Instruction::absolute(index_start - 1))),
            Registers::C => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDC(Instruction::absolute(index_start - 1))),
            Registers::X => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDX(Instruction::absolute(index_start - 1))),
            Registers::Y => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDY(Instruction::absolute(index_start - 1))),
        }
    }
}
