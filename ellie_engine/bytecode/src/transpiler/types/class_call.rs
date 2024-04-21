use ellie_core::definite::types::{class_call::ClassCall, Types as CoreTypes};

use crate::{
    assembler::LocalHeader,
    instruction_table::Instructions,
    instructions::{Instruction, Registers},
    types::Types as ByteCodeTypes,
    utils::usize_to_le_bytes,
};

use super::TypeTranspiler;

impl TypeTranspiler for ClassCall {
    fn transpile(&self, options: &mut super::TypeTranspilerOptions) {
        let dependencies = options.dependencies();
        let arch = options.assembler().platform_attributes.architecture;

        let target: LocalHeader = match *self.target.clone() {
            CoreTypes::VariableType(e) => {
                let data = options
                    .assembler_mut()
                    .find_local(&e.value, dependencies, true)
                    .unwrap()
                    .clone();
                data
            }
            _ => unreachable!("Unexpected target type"),
        };

        // Reserve class variables

        options
            .assembler_mut()
            .instructions
            .push(Instructions::ARR(Instruction::implicit()));
        let class_location = options.assembler().location();
        if !self.params.is_empty() {
            for (_idx, param) in self.params.iter().enumerate() {
                param.value.transpile(
                    options
                        .copy()
                        .set_assembler(options.assembler_mut())
                        .set_target_register(Registers::A),
                );

                options
                    .assembler_mut()
                    .instructions
                    .push(Instructions::STA(Instruction::implicit()));
                options
                    .assembler_mut()
                    .instructions
                    .push(Instructions::PUSH(Instruction::absolute(class_location)));
            }
        }

        let self_location = options.assembler().location() + 1;

        options
            .assembler_mut()
            .instructions
            .push(Instructions::CO(Instruction::absolute(class_location)));

        let previous_params_location = options.assembler().location() + 1;

        // Reserve parameters
        options
            .assembler_mut()
            .instructions
            .push(Instructions::STB(Instruction::implicit()));

        for _ in self.params.iter().enumerate() {
            options
                .assembler_mut()
                .instructions
                .push(Instructions::STB(Instruction::implicit()));
        }
        //-

        // Insert self
        options
            .assembler_mut()
            .instructions
            .push(Instructions::LDB(Instruction::absolute(self_location)));
        options
            .assembler_mut()
            .instructions
            .push(Instructions::STB(Instruction::absolute(
                previous_params_location,
            )));
        //-

        if !self.params.is_empty() {
            for (idx, param) in self.params.iter().enumerate() {
                // First position taken by self
                let idx = idx + 1;

                let prev_ = options.assembler().location();

                param.value.transpile(
                    options
                        .copy()
                        .set_assembler(options.assembler_mut())
                        .set_target_register(Registers::A),
                );

                options.assembler_mut().instructions.push(Instructions::STA(
                    Instruction::absolute(previous_params_location + idx),
                ));
            }
        }

        options
            .assembler_mut()
            .instructions
            .push(Instructions::LDX(Instruction::immediate(
                ByteCodeTypes::Integer,
                usize_to_le_bytes(previous_params_location, arch),
            )));

        options
            .assembler_mut()
            .instructions
            .push(Instructions::CALL(Instruction::absolute(target.cursor)));

        match options.target_register() {
            Registers::A => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDA(Instruction::absolute(self_location))),
            Registers::B => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDB(Instruction::absolute(self_location))),
            Registers::C => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDC(Instruction::absolute(self_location))),
            Registers::X => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDX(Instruction::absolute(self_location))),
            Registers::Y => options
                .assembler_mut()
                .instructions
                .push(Instructions::LDY(Instruction::absolute(self_location))),
        }
    }
}
