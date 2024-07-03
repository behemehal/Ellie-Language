use crate::{
    instruction_table::Instructions,
    instructions::{Instruction, Registers},
};

use super::TypeTranspiler;
use ellie_core::definite::types::as_keyword::AsKeyword;

impl TypeTranspiler for AsKeyword {
    fn transpile(&self, options: &mut super::TypeTranspilerOptions) {
        self.target.transpile(
            options
                .copy()
                .set_assembler(options.assembler_mut())
                .set_target_register(Registers::A),
        );

        match &self.rtype {
            ellie_core::definite::definers::DefinerCollecting::Generic(e) => {
                if e.rtype == "int" {
                    options
                        .assembler_mut()
                        .instructions
                        .push(Instructions::A2I(Instruction::implicit()));
                } else if e.rtype == "float" {
                    options
                        .assembler_mut()
                        .instructions
                        .push(Instructions::A2F(Instruction::implicit()));
                } else if e.rtype == "double" {
                    options
                        .assembler_mut()
                        .instructions
                        .push(Instructions::A2D(Instruction::implicit()));
                } else if e.rtype == "bool" {
                    options
                        .assembler_mut()
                        .instructions
                        .push(Instructions::A2O(Instruction::implicit()));
                } else if e.rtype == "string" {
                    options
                        .assembler_mut()
                        .instructions
                        .push(Instructions::A2S(Instruction::implicit()));
                } else if e.rtype == "char" {
                    options
                        .assembler_mut()
                        .instructions
                        .push(Instructions::A2C(Instruction::implicit()));
                } else if e.rtype == "byte" {
                    options
                        .assembler_mut()
                        .instructions
                        .push(Instructions::A2B(Instruction::implicit()));
                }
            }
            _ => panic!("As conv parent generic not implemented yet"),
        };

        options
            .assembler_mut()
            .instructions
            .push(Instructions::LDB(Instruction::indirect_a()));

        match options.target_register() {
            Registers::A => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDA(Instruction::indirect_b())),
            Registers::B => (),
            Registers::C => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDC(Instruction::indirect_b())),
            Registers::X => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDX(Instruction::indirect_b())),
            Registers::Y => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDY(Instruction::indirect_b())),
        }
    }
}
