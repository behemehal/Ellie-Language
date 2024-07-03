use crate::{
    addressing_modes::AddressingModes,
    instruction_table, instructions,
    transpiler::types::{TypeTranspiler, TypeTranspilerOptions},
    utils::limit_platform_size,
};
use alloc::{string::ToString, vec};
use ellie_core::{
    definite::items::setter_call,
    defs::{DebugHeader, DebugHeaderType},
};

impl super::Transpiler for setter_call::SetterCall {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        let mut dependencies = vec![processed_page.hash];
        dependencies.extend(processed_page.dependencies.iter().map(|d| d.hash));

        let location = assembler.location();

        let mut binding = TypeTranspilerOptions::new();
        let type_transpiler_options = binding
            .set_assembler(assembler)
            .set_target_register(instructions::Registers::C)
            .set_dependencies(dependencies.clone())
            .set_target_page(hash);

        self.value.transpile(type_transpiler_options);

        //Store it in the stack
        assembler
            .instructions
            .push(instruction_table::Instructions::STC(
                instructions::Instruction::implicit(),
            ));
        //Reserve value location
        let value_pos = assembler.location();

        match &self.operator {
            ellie_core::definite::types::operator::AssignmentOperators::Assignment => {
                let mut binding = TypeTranspilerOptions::new();
                let type_transpiler_options = binding
                    .set_assembler(assembler)
                    .set_target_register(instructions::Registers::B)
                    .set_dependencies(dependencies.clone())
                    .set_target_page(hash);

                self.target.transpile(type_transpiler_options);

                let target_last_instruction = assembler.instructions.last().unwrap().clone();
                match target_last_instruction {
                    instruction_table::Instructions::LDB(ldb_in) => {
                        match ldb_in.addressing_mode {
                            AddressingModes::Absolute(e) => {
                                //Load the value from `value_pos` to `b`
                                assembler.instructions.last_mut().unwrap().clone_from(
                                    &instruction_table::Instructions::LDB(
                                        instructions::Instruction::absolute(value_pos),
                                    ),
                                );
                                //Store the value from `b` to `e`
                                assembler.instructions.push(
                                    instruction_table::Instructions::STB(
                                        instructions::Instruction::absolute(e),
                                    ),
                                );
                            }
                            AddressingModes::AbsoluteIndex(pointer, index) => {
                                //Load the value from `value_pos` to `b`
                                assembler.instructions.last_mut().unwrap().clone_from(
                                    &instruction_table::Instructions::LDB(
                                        instructions::Instruction::absolute(value_pos),
                                    ),
                                );
                                //Load the value from `value_pos` to `b`
                                assembler.instructions.push(
                                    instruction_table::Instructions::STB(
                                        instructions::Instruction::absolute_index(pointer, index),
                                    ),
                                );
                            }
                            AddressingModes::AbsoluteProperty(pointer, index) => {
                                //Load the value from `value_pos` to `b`
                                assembler.instructions.last_mut().unwrap().clone_from(
                                    &instruction_table::Instructions::LDB(
                                        instructions::Instruction::absolute(value_pos),
                                    ),
                                );
                                //Load the value from `value_pos` to `b`
                                assembler.instructions.push(
                                    instruction_table::Instructions::STB(
                                        instructions::Instruction::absolute_property(pointer, index),
                                    ),
                                );

                            }
                            _ => unreachable!(
                                "Since this is setter its impossible to get a no absolute addressing mode"
                            ),
                        }
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
            e => {
                let operation_instruction = {
                    match e {
                            ellie_core::definite::types::operator::AssignmentOperators::AdditionAssignment => {
                                instruction_table::Instructions::ADD(
                                    instructions::Instruction::implicit(),
                                )
                            }
                            ellie_core::definite::types::operator::AssignmentOperators::SubtractionAssignment => {
                                instruction_table::Instructions::SUB(
                                    instructions::Instruction::implicit(),
                                )
                            },
                            ellie_core::definite::types::operator::AssignmentOperators::MultiplicationAssignment => {
                                instruction_table::Instructions::MUL(
                                    instructions::Instruction::implicit(),
                                )
                            },
                            ellie_core::definite::types::operator::AssignmentOperators::DivisionAssignment => {
                                instruction_table::Instructions::DIV(
                                    instructions::Instruction::implicit(),
                                )
                            },
                            ellie_core::definite::types::operator::AssignmentOperators::ModulusAssignment => {
                                instruction_table::Instructions::MOD(
                                    instructions::Instruction::implicit(),
                                )
                            },
                            ellie_core::definite::types::operator::AssignmentOperators::ExponentiationAssignment => {
                                instruction_table::Instructions::EXP(
                                    instructions::Instruction::implicit(),
                                )
                            },
                            _ => unreachable!(),
                        }
                };

                let mut binding = TypeTranspilerOptions::new();
                let type_transpiler_options = binding
                    .set_assembler(assembler)
                    .set_target_register(instructions::Registers::B)
                    .set_dependencies(dependencies)
                    .set_target_page(hash);

                self.target.transpile(type_transpiler_options);

                let left_last_instruction = assembler.instructions.last().unwrap().clone();
                assembler
                    .instructions
                    .push(instruction_table::Instructions::LDC(
                        instructions::Instruction::absolute(value_pos),
                    ));
                assembler.instructions.push(operation_instruction);

                match left_last_instruction {
                    instruction_table::Instructions::LDB(ldb_in) => {
                        match ldb_in.addressing_mode {
                            AddressingModes::Absolute(e) => {
                                assembler.instructions.push(
                                    instruction_table::Instructions::STA(
                                        instructions::Instruction::absolute(e),
                                    ),
                                );
                            }
                            AddressingModes::AbsoluteIndex(pointer, index) => {
                                assembler.instructions.push(
                                    instruction_table::Instructions::STA(
                                        instructions::Instruction::absolute_index(pointer, index),
                                    ),
                                );
                            }
                            AddressingModes::AbsoluteProperty(pointer, index) => {
                                assembler.instructions.push(
                                    instruction_table::Instructions::STA(
                                        instructions::Instruction::absolute_property(pointer, index),
                                    ),
                                );
                            }
                            _ => unreachable!(
                                "Since this is setter its impossible to get a no absolute addressing mode"
                            ),
                        }
                    }
                    _ => {
                        unreachable!()
                    }
                }

                assembler.debug_headers.push(DebugHeader {
                    rtype: DebugHeaderType::Variable,
                    hash: limit_platform_size(
                        self.hash,
                        assembler.platform_attributes.architecture,
                    ),
                    start_end: (location, assembler.location()),
                    module_name: processed_page.path.clone(),
                    module_hash: processed_page.hash,
                    name: "".to_string(),
                    pos: self.target_pos,
                });
            }
        }
        true
    }
}
