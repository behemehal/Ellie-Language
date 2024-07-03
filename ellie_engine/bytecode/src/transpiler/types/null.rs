use alloc::vec::Vec;

use crate::{
    instruction_table::Instructions,
    instructions::{Instruction, Registers},
    types::Types as ByteCodeTypes,
};

use super::TypeTranspiler;

pub struct Null;

impl TypeTranspiler for Null {
    fn transpile(&self, options: &mut super::TypeTranspilerOptions) {
        match options.target_register() {
            Registers::A => options.assembler_mut().instructions.push(Instructions::LDA(
                Instruction::immediate(ByteCodeTypes::Null, Vec::new()),
            )),
            Registers::B => options.assembler_mut().instructions.push(Instructions::LDB(
                Instruction::immediate(ByteCodeTypes::Null, Vec::new()),
            )),
            Registers::C => options.assembler_mut().instructions.push(Instructions::LDC(
                Instruction::immediate(ByteCodeTypes::Null, Vec::new()),
            )),
            Registers::X => options.assembler_mut().instructions.push(Instructions::LDX(
                Instruction::immediate(ByteCodeTypes::Null, Vec::new()),
            )),
            Registers::Y => options.assembler_mut().instructions.push(Instructions::LDY(
                Instruction::immediate(ByteCodeTypes::Null, Vec::new()),
            )),
        }
    }
}
