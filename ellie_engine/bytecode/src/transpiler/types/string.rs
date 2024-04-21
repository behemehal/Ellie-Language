use ellie_core::definite::types::string::StringType;

use super::TypeTranspiler;
use crate::{
    instruction_table::Instructions,
    instructions::{Instruction, Registers},
    types::Types as ByteCodeTypes,
};

impl TypeTranspiler for StringType {
    fn transpile(&self, options: &mut super::TypeTranspilerOptions) {
        //Create heap array
        options
            .assembler_mut()
            .instructions
            .push(Instructions::STR(Instruction::implicit()));
        let array_location = options.assembler().location();

        for char in self.value.chars() {
            options
                .assembler_mut()
                .instructions
                .push(Instructions::STA(Instruction::immediate(
                    ByteCodeTypes::Char,
                    (char as u32).to_le_bytes().to_vec(),
                )));
            options
                .assembler_mut()
                .instructions
                .push(Instructions::SPUS(Instruction::absolute(array_location)))
        }

        match options.target_register() {
            Registers::A => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDA(Instruction::absolute(array_location))),
            Registers::B => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDB(Instruction::absolute(array_location))),
            Registers::C => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDC(Instruction::absolute(array_location))),
            Registers::X => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDX(Instruction::absolute(array_location))),
            Registers::Y => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDY(Instruction::absolute(array_location))),
        }
    }
}
