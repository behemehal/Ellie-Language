use crate::{instruction_table::Instructions, instructions::Registers};

use super::TypeTranspiler;
use ellie_core::definite::types::function::FunctionParameter;

impl TypeTranspiler for FunctionParameter {
    fn transpile(&self, options: &mut super::TypeTranspilerOptions) {
        let dependencies = options.dependencies().clone();
        let pos = match options
            .assembler_mut()
            .find_local(&self.name, dependencies, false)
        {
            Some(e) => e,
            None => panic!("Function Parameter not found: {}", self.name),
        };

        match options.target_register() {
            Registers::A => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDA(pos.reference.clone())),
            Registers::B => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDB(pos.reference.clone())),
            Registers::C => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDC(pos.reference.clone())),
            Registers::X => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDX(pos.reference.clone())),
            Registers::Y => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDY(pos.reference.clone())),
        }
    }
}
