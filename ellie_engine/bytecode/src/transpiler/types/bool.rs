use ellie_core::definite::types::{bool::BoolType, Types};

use crate::{
    instruction_table::Instructions,
    instructions::{Instruction, Registers},
};

use super::{convert_type, TypeTranspiler};

impl TypeTranspiler for BoolType {
    fn transpile(&self, options: &mut super::TypeTranspilerOptions) {
        let converted_type = convert_type(
            &Types::Bool(self.clone()),
            options.dependencies(),
            options.assembler().platform_attributes.architecture,
        );
        match options.target_register() {
            Registers::A => options.assembler_mut().instructions.push(Instructions::LDA(
                Instruction::immediate(converted_type.0, converted_type.1),
            )),
            Registers::B => options.assembler_mut().instructions.push(Instructions::LDB(
                Instruction::immediate(converted_type.0, converted_type.1),
            )),
            Registers::C => options.assembler_mut().instructions.push(Instructions::LDC(
                Instruction::immediate(converted_type.0, converted_type.1),
            )),
            Registers::X => options.assembler_mut().instructions.push(Instructions::LDX(
                Instruction::immediate(converted_type.0, converted_type.1),
            )),
            Registers::Y => options.assembler_mut().instructions.push(Instructions::LDY(
                Instruction::immediate(converted_type.0, converted_type.1),
            )),
        }
    }
}
