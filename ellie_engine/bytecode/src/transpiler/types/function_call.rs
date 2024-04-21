use alloc::vec;
use ellie_core::definite::types::{
    class_instance::AttributeType, function_call::FunctionCall, Types as CoreTypes,
};


use crate::{
    assembler::LocalHeader,
    instruction_table::Instructions,
    instructions::{Instruction, Registers},
    types::Types as ByteCodeTypes,
    utils::usize_to_le_bytes,
};

use super::TypeTranspiler;

impl TypeTranspiler for FunctionCall {
    fn transpile(&self, options: &mut super::TypeTranspilerOptions) {
        let dependencies = options.dependencies();
        let arch = options.assembler().platform_attributes.architecture;

        let mut is_reference: Option<usize> = None;
        let target: LocalHeader = match *self.target.clone() {
            CoreTypes::VariableType(e) => options
                .assembler_mut()
                .find_local(&e.value, dependencies.clone(), true)
                .unwrap_or_else(|| {
                    panic!("Variable not found: {}: {:#?} {:#?}, {:#?}", e.value, dependencies, options.target_page, self);
                })
                .clone(),
            CoreTypes::Reference(e) => {
                let mut _pos = options.assembler().location();

                e.reference.transpile(
                    options
                        .copy()
                        .set_assembler(options.assembler_mut())
                        .set_target_register(Registers::B),
                );

                options
                    .assembler_mut()
                    .instructions
                    .push(Instructions::STB(Instruction::implicit()));
                let mut last_pos = options.assembler().location();
                let mut found = None;

                for (idx, chain) in e.index_chain.iter().enumerate() {
                    if e.index_chain.len() - 1 != idx {
                        match chain.rtype {
                            AttributeType::Property => {
                                options.assembler_mut().instructions.push(Instructions::LDA(
                                    Instruction::absolute_property(
                                        last_pos,
                                        chain.class_attribute_idx,
                                    ),
                                ));

                                options
                                    .assembler_mut()
                                    .instructions
                                    .push(Instructions::STA(Instruction::implicit()));
                                last_pos = options.assembler().location();
                            }
                            AttributeType::Method => unimplemented!(),
                            AttributeType::Setter => unimplemented!(),
                            AttributeType::Getter => unimplemented!(),
                            AttributeType::EnumItemData => unimplemented!(),
                            AttributeType::EnumItemNoData => unimplemented!(),
                        }
                    }
                }
                let last_chain = e.index_chain.last().unwrap();
                found = options.assembler_mut().find_local_by_hash(
                    last_chain.hash,
                    Some(vec![last_chain.page_hash]),
                    true,
                );
                is_reference = Some(last_pos);
                found.unwrap()
            }
            _ => unreachable!("Unexpected target type"),
        };

        let previous_params_location = options.assembler().location() + 1;
        if is_reference.is_some() {
            options
                .assembler_mut()
                .instructions
                .push(Instructions::STB(Instruction::implicit()));
        }

        for _ in &self.params {
            options
                .assembler_mut()
                .instructions
                .push(Instructions::STB(Instruction::implicit()));
        }

        if let Some(reference) = &is_reference {
            options
                .assembler_mut()
                .instructions
                .push(Instructions::LDB(Instruction::absolute(*reference)));
            options
                .assembler_mut()
                .instructions
                .push(Instructions::STB(Instruction::absolute(
                    previous_params_location,
                )));
        }

        if !self.params.is_empty() {
            for (idx, param) in self.params.iter().enumerate() {
                let idx = if is_reference.is_some() { idx + 1 } else { idx };

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

        let borrow_location = options.assembler().location() + 1;

        options
            .assembler_mut()
            .add_borrow_to_local(target.hash.unwrap(), borrow_location);

        match options.target_register() {
            Registers::A => {
                options
                    .assembler_mut()
                    .instructions
                    .push(Instructions::LDA(Instruction::indirect_y()));
            }
            Registers::B => {
                options
                    .assembler_mut()
                    .instructions
                    .push(Instructions::LDB(Instruction::indirect_y()));
            }
            Registers::C => {
                options
                    .assembler_mut()
                    .instructions
                    .push(Instructions::LDC(Instruction::indirect_y()));
            }
            Registers::X => {
                options
                    .assembler_mut()
                    .instructions
                    .push(Instructions::LDX(Instruction::indirect_y()));
            }
            Registers::Y => (),
        }
    }
}
