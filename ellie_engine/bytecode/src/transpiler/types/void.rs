use alloc::vec::Vec;

use crate::{
    instruction_table::Instructions,
    instructions::{Instruction, Registers},
    types::Types as ByteCodeTypes,
};

use super::TypeTranspiler;

pub struct Void;

impl TypeTranspiler for Void {
    fn transpile(&self, options: &mut super::TypeTranspilerOptions) {
        match options.target_register() {
            Registers::A => options.assembler_mut().instructions.push(Instructions::LDA(
                Instruction::immediate(ByteCodeTypes::Void, Vec::new()),
            )),
            Registers::B => options.assembler_mut().instructions.push(Instructions::LDB(
                Instruction::immediate(ByteCodeTypes::Void, Vec::new()),
            )),
            Registers::C => options.assembler_mut().instructions.push(Instructions::LDC(
                Instruction::immediate(ByteCodeTypes::Void, Vec::new()),
            )),
            Registers::X => options.assembler_mut().instructions.push(Instructions::LDX(
                Instruction::immediate(ByteCodeTypes::Void, Vec::new()),
            )),
            Registers::Y => options.assembler_mut().instructions.push(Instructions::LDY(
                Instruction::immediate(ByteCodeTypes::Void, Vec::new()),
            )),
        }
    }
}
