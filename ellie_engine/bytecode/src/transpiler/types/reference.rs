use ellie_core::definite::types::reference::ReferenceType;

use crate::{
    instruction_table::Instructions,
    instructions::{Instruction, Registers},
};

use super::TypeTranspiler;

impl TypeTranspiler for ReferenceType {
    fn transpile(&self, options: &mut super::TypeTranspilerOptions) {
        self.reference.transpile(
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
        for (idx, chain) in self.index_chain.iter().enumerate() {
            match chain.rtype {
                ellie_core::definite::types::class_instance::AttributeType::Property => {
                    match options.target_register() {
                        Registers::A => {
                            options.assembler_mut().instructions.push(Instructions::LDA(
                                Instruction::absolute_property(last_pos, chain.idx),
                            ));
                        }
                        Registers::B => {
                            options.assembler_mut().instructions.push(Instructions::LDB(
                                Instruction::absolute_property(last_pos, chain.idx),
                            ));
                        }
                        Registers::C => {
                            options.assembler_mut().instructions.push(Instructions::LDC(
                                Instruction::absolute_property(last_pos, chain.idx),
                            ));
                        }
                        Registers::X => {
                            options.assembler_mut().instructions.push(Instructions::LDX(
                                Instruction::absolute_property(last_pos, chain.idx),
                            ));
                        }
                        Registers::Y => {
                            options.assembler_mut().instructions.push(Instructions::LDY(
                                Instruction::absolute_property(last_pos, chain.idx),
                            ));
                        }
                    }
                }
                ellie_core::definite::types::class_instance::AttributeType::Method => {
                    unreachable!("Method is handled by the function call")
                }
                ellie_core::definite::types::class_instance::AttributeType::Getter => {
                    unreachable!("Getter is handled by the getter call")
                }
                ellie_core::definite::types::class_instance::AttributeType::Setter => todo!(),
                ellie_core::definite::types::class_instance::AttributeType::EnumItemData => {
                    todo!()
                }
                ellie_core::definite::types::class_instance::AttributeType::EnumItemNoData => {
                    todo!()
                }
            }
            if self.index_chain.len() - 1 != idx {
                match chain.rtype {
                    ellie_core::definite::types::class_instance::AttributeType::Property => {
                        match options.target_register() {
                            Registers::A => {
                                options
                                    .assembler_mut()
                                    .instructions
                                    .push(Instructions::STA(Instruction::implicit()));
                            }
                            Registers::B => {
                                options
                                    .assembler_mut()
                                    .instructions
                                    .push(Instructions::STB(Instruction::implicit()));
                            }
                            Registers::C => {
                                options
                                    .assembler_mut()
                                    .instructions
                                    .push(Instructions::STC(Instruction::implicit()));
                            }
                            Registers::X => {
                                options
                                    .assembler_mut()
                                    .instructions
                                    .push(Instructions::STX(Instruction::implicit()));
                            }
                            Registers::Y => {
                                options
                                    .assembler_mut()
                                    .instructions
                                    .push(Instructions::STY(Instruction::implicit()));
                            }
                        }
                        last_pos = options.assembler().location();
                    }
                    ellie_core::definite::types::class_instance::AttributeType::Method => {
                        unreachable!("Method is handled by the function call")
                    }
                    ellie_core::definite::types::class_instance::AttributeType::Getter => {
                        unreachable!("Getter is handled by the getter call")
                    }
                    ellie_core::definite::types::class_instance::AttributeType::Setter => todo!(),
                    ellie_core::definite::types::class_instance::AttributeType::EnumItemData => {
                        todo!()
                    }
                    ellie_core::definite::types::class_instance::AttributeType::EnumItemNoData => {
                        todo!()
                    }
                }
            }
        }
    }
}
