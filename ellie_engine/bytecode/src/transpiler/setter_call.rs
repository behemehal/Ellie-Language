use super::type_resolver::resolve_type;
use crate::addressing_modes::AddressingModes;
use crate::instruction_table;
use crate::instructions;
use crate::utils::limit_platform_size;
use alloc::string::ToString;
use alloc::vec;
use ellie_core::definite::items::setter_call;
use ellie_core::defs::DebugHeader;
use ellie_core::defs::DebugHeaderType;

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

        //Resolve the value to be inserted
        resolve_type(
            assembler,
            &self.value,
            instructions::Registers::C,
            &hash,
            Some(dependencies.clone()),
        );

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
                //Resolve the target
                resolve_type(
                    assembler,
                    &self.target,
                    instructions::Registers::B,
                    &hash,
                    Some(dependencies.clone()),
                );
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
                resolve_type(
                    assembler,
                    &self.target,
                    instructions::Registers::B,
                    &hash,
                    Some(dependencies.clone()),
                );
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
                    hash: limit_platform_size(self.hash, assembler.platform_attributes.architecture),
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
