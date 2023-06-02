use alloc::vec;
use alloc::vec::Vec;
use ellie_core::{
    definite::{types::operator, types::Types as CoreTypes},
    defs::PlatformArchitecture,
};

use crate::{
    assembler::Assembler,
    instruction_table,
    instructions::{self, Instruction},
    types::Types,
    utils::{f32_to_le_bytes, f64_to_le_bytes, isize_to_le_bytes, usize_to_le_bytes},
};

pub fn convert_type(
    types: &CoreTypes,
    _page_hash: Option<Vec<usize>>,
    arch: PlatformArchitecture,
) -> (Types, Vec<u8>) {
    match types {
        CoreTypes::Byte(byte) => (Types::Byte, byte.value.to_le_bytes().to_vec()),
        CoreTypes::Integer(integer) => (Types::Integer, isize_to_le_bytes(integer.value, arch)),
        CoreTypes::Decimal(decimal) => match decimal.value {
            ellie_core::definite::types::decimal::DecimalTypeEnum::Float(float_value) => {
                (Types::Float, f32_to_le_bytes(float_value, arch))
            }
            ellie_core::definite::types::decimal::DecimalTypeEnum::Double(double_value) => {
                (Types::Double, f64_to_le_bytes(double_value, arch))
            }
        },
        CoreTypes::Bool(bool) => (Types::Bool, (bool.value as u8).to_le_bytes().to_vec()),
        CoreTypes::Char(e) => (Types::Char, (e.value as u32).to_le_bytes().to_vec()),
        _ => unreachable!("This type is not convertable to raw type"),
    }
}

/// Resolves type to instructions
/// * `assembler` - Assembler instance
/// * `types` - Type to resolve
/// * `target_register` - Target register to store the value
/// * `target_page` - Target page to store the value
/// * `dependencies` - Dependencies of the type
pub fn resolve_type(
    assembler: &mut Assembler,
    types: &CoreTypes,
    target_register: instructions::Registers,
    target_page: &usize,
    dependencies: Option<Vec<usize>>,
) {
    match types {
        CoreTypes::Collective(_) => todo!(),
        CoreTypes::Reference(e) => {
            resolve_type(
                assembler,
                &e.reference,
                instructions::Registers::B,
                target_page,
                dependencies.clone(),
            );
            assembler
                .instructions
                .push(instruction_table::Instructions::STB(Instruction::implicit()));
            let mut last_pos = assembler.location();
            for (idx, chain) in e.index_chain.iter().enumerate() {
                match chain.rtype {
                    ellie_core::definite::types::class_instance::AttributeType::Property => {
                        match target_register {
                            instructions::Registers::A => {
                                assembler
                                    .instructions
                                    .push(instruction_table::Instructions::LDA(
                                        instructions::Instruction::absolute_property(
                                            last_pos, chain.idx,
                                        ),
                                    ));
                            }
                            instructions::Registers::B => {
                                assembler
                                    .instructions
                                    .push(instruction_table::Instructions::LDB(
                                        instructions::Instruction::absolute_property(
                                            last_pos, chain.idx,
                                        ),
                                    ));
                            }
                            instructions::Registers::C => {
                                assembler
                                    .instructions
                                    .push(instruction_table::Instructions::LDC(
                                        instructions::Instruction::absolute_property(
                                            last_pos, chain.idx,
                                        ),
                                    ));
                            }
                            instructions::Registers::X => {
                                assembler
                                    .instructions
                                    .push(instruction_table::Instructions::LDX(
                                        instructions::Instruction::absolute_property(
                                            last_pos, chain.idx,
                                        ),
                                    ));
                            }
                            instructions::Registers::Y => {
                                assembler
                                    .instructions
                                    .push(instruction_table::Instructions::LDY(
                                        instructions::Instruction::absolute_property(
                                            last_pos, chain.idx,
                                        ),
                                    ));
                            }
                        }
                    }
                    ellie_core::definite::types::class_instance::AttributeType::Method => todo!(),
                    ellie_core::definite::types::class_instance::AttributeType::Setter => todo!(),
                    ellie_core::definite::types::class_instance::AttributeType::Getter => todo!(),
                    ellie_core::definite::types::class_instance::AttributeType::EnumItemData => {
                        todo!()
                    }
                    ellie_core::definite::types::class_instance::AttributeType::EnumItemNoData => {
                        todo!()
                    }
                }
                if e.index_chain.len() - 1 != idx {
                    match chain.rtype {
                        ellie_core::definite::types::class_instance::AttributeType::Property => {
                            match target_register {
                                instructions::Registers::A => {
                                    assembler
                                        .instructions
                                        .push(instruction_table::Instructions::STA(
                                            instructions::Instruction::implicit(),
                                        ));
                                }
                                instructions::Registers::B => {
                                    assembler
                                        .instructions
                                        .push(instruction_table::Instructions::STB(
                                            instructions::Instruction::implicit(),
                                        ));
                                }
                                instructions::Registers::C => {
                                    assembler
                                        .instructions
                                        .push(instruction_table::Instructions::STC(
                                            instructions::Instruction::implicit(
                                            ),
                                        ));
                                }
                                instructions::Registers::X => {
                                    assembler
                                        .instructions
                                        .push(instruction_table::Instructions::STX(
                                            instructions::Instruction::implicit(
                                            ),
                                        ));
                                }
                                instructions::Registers::Y => {
                                    assembler
                                        .instructions
                                        .push(instruction_table::Instructions::STY(
                                            instructions::Instruction::implicit(
                                            ),
                                        ));
                                }
                            }
                            last_pos = assembler.location();
                        }
                        ellie_core::definite::types::class_instance::AttributeType::Method => todo!(),
                        ellie_core::definite::types::class_instance::AttributeType::Setter => todo!(),
                        ellie_core::definite::types::class_instance::AttributeType::Getter => todo!(),
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
        CoreTypes::BraceReference(e) => {
            resolve_type(
                assembler,
                &e.reference,
                instructions::Registers::B,
                target_page,
                dependencies.clone(),
            );
            assembler
                .instructions
                .push(instruction_table::Instructions::STB(Instruction::implicit()));

            let location_of_pointer = assembler.location();

            resolve_type(
                assembler,
                &e.value,
                instructions::Registers::C,
                target_page,
                dependencies.clone(),
            );
            assembler
                .instructions
                .push(instruction_table::Instructions::STC(Instruction::implicit()));

            match target_register {
                instructions::Registers::A => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDA(
                            instructions::Instruction::absolute_index(
                                location_of_pointer,
                                assembler.location(),
                            ),
                        ))
                }
                instructions::Registers::B => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDB(
                            instructions::Instruction::absolute_index(
                                location_of_pointer,
                                assembler.location(),
                            ),
                        ))
                }
                instructions::Registers::C => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDC(
                            instructions::Instruction::absolute_index(
                                location_of_pointer,
                                assembler.location(),
                            ),
                        ))
                }
                instructions::Registers::X => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDX(
                            instructions::Instruction::absolute_index(
                                location_of_pointer,
                                assembler.location(),
                            ),
                        ))
                }
                instructions::Registers::Y => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDY(
                            instructions::Instruction::absolute_index(
                                location_of_pointer,
                                assembler.location(),
                            ),
                        ))
                }
            }
        }
        CoreTypes::Operator(operator) => match &operator.operator {
            operator::Operators::ComparisonType(e) => {
                resolve_type(
                    assembler,
                    &operator.first,
                    instructions::Registers::B,
                    target_page,
                    dependencies.clone(),
                );

                let first_operator_pos = assembler.instructions.len();
                assembler
                    .instructions
                    .push(instruction_table::Instructions::STB(Instruction::implicit()));

                resolve_type(
                    assembler,
                    &operator.second,
                    instructions::Registers::C,
                    target_page,
                    dependencies,
                );

                let second_operator_pos = assembler.instructions.len();
                assembler
                    .instructions
                    .push(instruction_table::Instructions::STC(Instruction::implicit()));

                assembler
                    .instructions
                    .push(instruction_table::Instructions::LDB(Instruction::absolute(
                        first_operator_pos,
                    )));
                assembler
                    .instructions
                    .push(instruction_table::Instructions::LDC(Instruction::absolute(
                        second_operator_pos,
                    )));

                assembler.instructions.push(match e {
                    operator::ComparisonOperators::Equal => {
                        instruction_table::Instructions::EQ(Instruction::implicit())
                    }
                    operator::ComparisonOperators::NotEqual => {
                        instruction_table::Instructions::NE(Instruction::implicit())
                    }
                    operator::ComparisonOperators::GreaterThan => {
                        instruction_table::Instructions::GT(Instruction::implicit())
                    }
                    operator::ComparisonOperators::LessThan => {
                        instruction_table::Instructions::LT(Instruction::implicit())
                    }
                    operator::ComparisonOperators::GreaterThanOrEqual => {
                        instruction_table::Instructions::GQ(Instruction::implicit())
                    }
                    operator::ComparisonOperators::LessThanOrEqual => {
                        instruction_table::Instructions::LQ(Instruction::implicit())
                    }
                    operator::ComparisonOperators::Null => unreachable!(),
                });

                match target_register {
                    instructions::Registers::A => (),
                    instructions::Registers::B => {
                        assembler
                            .instructions
                            .push(instruction_table::Instructions::LDB(
                                Instruction::indirect_a(),
                            ));
                    }
                    instructions::Registers::C => {
                        assembler
                            .instructions
                            .push(instruction_table::Instructions::LDC(
                                Instruction::indirect_a(),
                            ));
                    }
                    instructions::Registers::X => {
                        assembler
                            .instructions
                            .push(instruction_table::Instructions::LDX(
                                Instruction::indirect_a(),
                            ));
                    }
                    instructions::Registers::Y => {
                        assembler
                            .instructions
                            .push(instruction_table::Instructions::LDY(
                                Instruction::indirect_a(),
                            ));
                    }
                }
            }
            operator::Operators::LogicalType(e) => {
                resolve_type(
                    assembler,
                    &operator.second,
                    instructions::Registers::C,
                    target_page,
                    dependencies.clone(),
                );

                let second_operator_pos = assembler.instructions.len();
                assembler
                    .instructions
                    .push(instruction_table::Instructions::STC(Instruction::implicit()));

                resolve_type(
                    assembler,
                    &operator.first,
                    instructions::Registers::B,
                    target_page,
                    dependencies,
                );

                assembler
                    .instructions
                    .push(instruction_table::Instructions::LDC(Instruction::absolute(
                        second_operator_pos,
                    )));

                assembler.instructions.push(match e {
                    operator::LogicalOperators::And => {
                        instruction_table::Instructions::AND(Instruction::implicit())
                    }
                    operator::LogicalOperators::Or => {
                        instruction_table::Instructions::OR(Instruction::implicit())
                    }
                    _ => unreachable!(),
                });
                match target_register {
                    instructions::Registers::A => (),
                    instructions::Registers::B => {
                        assembler
                            .instructions
                            .push(instruction_table::Instructions::LDB(
                                Instruction::indirect_a(),
                            ));
                    }
                    instructions::Registers::C => {
                        assembler
                            .instructions
                            .push(instruction_table::Instructions::LDC(
                                Instruction::indirect_a(),
                            ));
                    }
                    instructions::Registers::X => {
                        assembler
                            .instructions
                            .push(instruction_table::Instructions::LDX(
                                Instruction::indirect_a(),
                            ));
                    }
                    instructions::Registers::Y => {
                        assembler
                            .instructions
                            .push(instruction_table::Instructions::LDY(
                                Instruction::indirect_a(),
                            ));
                    }
                }
            }
            operator::Operators::ArithmeticType(e) => {
                resolve_type(
                    assembler,
                    &operator.first,
                    instructions::Registers::B,
                    target_page,
                    dependencies.clone(),
                );

                let first_operator_pos = assembler.instructions.len();
                assembler
                    .instructions
                    .push(instruction_table::Instructions::STB(Instruction::implicit()));

                resolve_type(
                    assembler,
                    &operator.second,
                    instructions::Registers::C,
                    target_page,
                    dependencies,
                );

                let second_operator_pos = assembler.instructions.len();
                assembler
                    .instructions
                    .push(instruction_table::Instructions::STC(Instruction::implicit()));

                assembler
                    .instructions
                    .push(instruction_table::Instructions::LDB(Instruction::absolute(
                        first_operator_pos,
                    )));
                assembler
                    .instructions
                    .push(instruction_table::Instructions::LDC(Instruction::absolute(
                        second_operator_pos,
                    )));

                assembler.instructions.push(match e {
                    operator::ArithmeticOperators::Addition => {
                        instruction_table::Instructions::ADD(Instruction::implicit())
                    }
                    operator::ArithmeticOperators::Subtraction => {
                        instruction_table::Instructions::SUB(Instruction::implicit())
                    }
                    operator::ArithmeticOperators::Multiplication => {
                        instruction_table::Instructions::MUL(Instruction::implicit())
                    }
                    operator::ArithmeticOperators::Exponentiation => {
                        instruction_table::Instructions::EXP(Instruction::implicit())
                    }
                    operator::ArithmeticOperators::Division => {
                        instruction_table::Instructions::DIV(Instruction::implicit())
                    }
                    operator::ArithmeticOperators::Modulus => {
                        instruction_table::Instructions::MOD(Instruction::implicit())
                    }
                    operator::ArithmeticOperators::Null => unreachable!("Wrong operator"),
                });
                match target_register {
                    instructions::Registers::A => (),
                    instructions::Registers::B => {
                        assembler
                            .instructions
                            .push(instruction_table::Instructions::LDB(
                                Instruction::indirect_a(),
                            ));
                    }
                    instructions::Registers::C => {
                        assembler
                            .instructions
                            .push(instruction_table::Instructions::LDC(
                                Instruction::indirect_a(),
                            ));
                    }
                    instructions::Registers::X => {
                        assembler
                            .instructions
                            .push(instruction_table::Instructions::LDX(
                                Instruction::indirect_a(),
                            ));
                    }
                    instructions::Registers::Y => {
                        assembler
                            .instructions
                            .push(instruction_table::Instructions::LDY(
                                Instruction::indirect_a(),
                            ));
                    }
                }
            }
            operator::Operators::AssignmentType(_) => todo!(),
            operator::Operators::Null => unreachable!(),
        },
        CoreTypes::Cloak(e) => {
            let mut size = 0;

            for entry in &e.collective {
                resolve_type(
                    assembler,
                    &entry.value,
                    instructions::Registers::A,
                    target_page,
                    dependencies.clone(),
                );
                assembler
                    .instructions
                    .push(instruction_table::Instructions::STA(Instruction::implicit()));
                size += 1;
            }
            assembler
                .instructions
                .push(instruction_table::Instructions::SAR(
                    Instruction::immediate(
                        Types::StaticArray(size),
                        usize_to_le_bytes(size, assembler.platform_attributes.architecture),
                    ),
                ));

            match target_register {
                instructions::Registers::A => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDA(Instruction::absolute(
                            assembler.location(),
                        )))
                }
                instructions::Registers::B => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDB(Instruction::absolute(
                            assembler.location(),
                        )))
                }
                instructions::Registers::C => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDC(Instruction::absolute(
                            assembler.location(),
                        )))
                }
                instructions::Registers::X => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDX(Instruction::absolute(
                            assembler.location(),
                        )))
                }
                instructions::Registers::Y => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDY(Instruction::absolute(
                            assembler.location(),
                        )))
                }
            }
        }
        CoreTypes::Array(e) => {
            assembler
                .instructions
                .push(instruction_table::Instructions::ARR(Instruction::implicit()));
            let array_heap_location = assembler.location();

            for entry in &e.collective {
                resolve_type(
                    assembler,
                    &entry.value,
                    instructions::Registers::A,
                    target_page,
                    dependencies.clone(),
                );
                assembler
                    .instructions
                    .push(instruction_table::Instructions::STA(Instruction::implicit()));
                assembler
                    .instructions
                    .push(instruction_table::Instructions::PUSH(
                        Instruction::absolute(array_heap_location),
                    ));
            }

            match target_register {
                instructions::Registers::A => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDA(Instruction::absolute(
                            array_heap_location,
                        )))
                }
                instructions::Registers::B => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDB(Instruction::absolute(
                            array_heap_location,
                        )))
                }
                instructions::Registers::C => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDC(Instruction::absolute(
                            array_heap_location,
                        )))
                }
                instructions::Registers::X => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDX(Instruction::absolute(
                            array_heap_location,
                        )))
                }
                instructions::Registers::Y => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDY(Instruction::absolute(
                            array_heap_location,
                        )))
                }
            }
        }
        CoreTypes::Function(_) => todo!(),
        CoreTypes::ClassCall(class_call) => {
            assembler
                .instructions
                .push(instruction_table::Instructions::ARR(Instruction::implicit()));
            let class_location = assembler.location();
            if !class_call.params.is_empty() {
                for (idx, param) in class_call.params.iter().enumerate() {
                    resolve_type(
                        assembler,
                        &param.value,
                        instructions::Registers::A,
                        &target_page,
                        dependencies.clone(),
                    );
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::STA(Instruction::implicit()));
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::PUSH(
                            Instruction::absolute(class_location),
                        ));
                }
            }
            match target_register {
                instructions::Registers::A => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDA(
                            Instruction::immediate(
                                Types::Class(class_call.params.len()),
                                usize_to_le_bytes(
                                    class_location,
                                    assembler.platform_attributes.architecture,
                                ),
                            ),
                        ))
                }
                instructions::Registers::B => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDB(
                            Instruction::immediate(
                                Types::Class(class_call.params.len()),
                                usize_to_le_bytes(
                                    class_location,
                                    assembler.platform_attributes.architecture,
                                ),
                            ),
                        ))
                }
                instructions::Registers::C => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDC(
                            Instruction::immediate(
                                Types::Class(class_call.params.len()),
                                usize_to_le_bytes(
                                    class_location,
                                    assembler.platform_attributes.architecture,
                                ),
                            ),
                        ))
                }
                instructions::Registers::X => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDX(
                            Instruction::immediate(
                                Types::Class(class_call.params.len()),
                                usize_to_le_bytes(
                                    class_location,
                                    assembler.platform_attributes.architecture,
                                ),
                            ),
                        ))
                }
                instructions::Registers::Y => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDY(
                            Instruction::immediate(
                                Types::Class(class_call.params.len()),
                                usize_to_le_bytes(
                                    class_location,
                                    assembler.platform_attributes.architecture,
                                ),
                            ),
                        ))
                }
            }
        }
        CoreTypes::FunctionCall(function_call) => {
            let mut is_reference = None;
            let target = match *function_call.target.clone() {
                CoreTypes::VariableType(e) => assembler
                    .find_local(&e.value, dependencies.clone(), true)
                    .unwrap()
                    .clone(),
                CoreTypes::Reference(e) => {
                    let reference = match *e.reference {
                        CoreTypes::VariableType(e) => assembler
                            .find_local(&e.value, dependencies.clone(), true)
                            .unwrap()
                            .clone(),
                        _ => unreachable!("Unexpected target type"),
                    };
                    let hash = e.index_chain.last().unwrap();
                    is_reference = Some(reference.clone());
                    assembler
                        .find_local_by_hash(
                            e.index_chain.last().unwrap().hash,
                            Some(vec![hash.page_hash]),
                            true,
                        )
                        .unwrap()
                        .clone()
                }
                _ => unreachable!("Unexpected target type"),
            };

            let previous_params_location = assembler.location() + 1;
            if is_reference.is_some() {
                assembler
                    .instructions
                    .push(instruction_table::Instructions::STB(Instruction::implicit()));
            }

            for _ in &function_call.params {
                assembler
                    .instructions
                    .push(instruction_table::Instructions::STB(Instruction::implicit()));
            }

            if let Some(reference) = &is_reference {
                assembler
                    .instructions
                    .push(instruction_table::Instructions::LDA(Instruction::absolute(
                        reference.cursor,
                    )));
                assembler.add_borrow_to_local(reference.hash.unwrap(), assembler.location());
                assembler
                    .instructions
                    .push(instruction_table::Instructions::STA(Instruction::absolute(
                        previous_params_location,
                    )));
            }

            if !function_call.params.is_empty() {
                for (idx, param) in function_call.params.iter().enumerate() {
                    let idx = if is_reference.is_some() { idx + 1 } else { idx };
                    resolve_type(
                        assembler,
                        &param.value,
                        instructions::Registers::A,
                        &target_page,
                        dependencies.clone(),
                    );
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::STA(Instruction::absolute(
                            previous_params_location + idx,
                        )));
                }
            }

            assembler
                .instructions
                .push(instruction_table::Instructions::LDX(
                    Instruction::immediate(
                        Types::Integer,
                        usize_to_le_bytes(
                            previous_params_location,
                            assembler.platform_attributes.architecture,
                        ),
                    ),
                ));

            assembler
                .instructions
                .push(instruction_table::Instructions::CALL(
                    Instruction::absolute(target.cursor),
                ));
            assembler.add_borrow_to_local(target.hash.unwrap(), assembler.location());

            match target_register {
                instructions::Registers::A => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDA(
                            Instruction::indirect_y(),
                        ));
                }
                instructions::Registers::B => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDB(
                            Instruction::indirect_y(),
                        ));
                }
                instructions::Registers::C => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDC(
                            Instruction::indirect_y(),
                        ));
                }
                instructions::Registers::X => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDX(
                            Instruction::indirect_y(),
                        ));
                }
                instructions::Registers::Y => (),
            }
        }
        CoreTypes::Void => match target_register {
            instructions::Registers::A => {
                assembler
                    .instructions
                    .push(instruction_table::Instructions::LDA(
                        Instruction::immediate(Types::Void, Vec::new()),
                    ))
            }
            instructions::Registers::B => {
                assembler
                    .instructions
                    .push(instruction_table::Instructions::LDB(
                        Instruction::immediate(Types::Void, Vec::new()),
                    ))
            }
            instructions::Registers::C => {
                assembler
                    .instructions
                    .push(instruction_table::Instructions::LDC(
                        Instruction::immediate(Types::Void, Vec::new()),
                    ))
            }
            instructions::Registers::X => {
                assembler
                    .instructions
                    .push(instruction_table::Instructions::LDX(
                        Instruction::immediate(Types::Void, Vec::new()),
                    ))
            }
            instructions::Registers::Y => {
                assembler
                    .instructions
                    .push(instruction_table::Instructions::LDY(
                        Instruction::immediate(Types::Void, Vec::new()),
                    ))
            }
        },
        CoreTypes::NullResolver(e) => {
            resolve_type(
                assembler,
                &e.target,
                target_register,
                target_page,
                dependencies.clone(),
            );
        }
        CoreTypes::Negative(_) => todo!(),
        CoreTypes::VariableType(e) => {
            let pos = match assembler.find_local(&e.value, dependencies, false) {
                Some(e) => e,
                None => panic!("Variable not found: {}", e.value),
            };
            let mut instructions = Vec::new();

            match target_register {
                instructions::Registers::A => {
                    instructions.push(instruction_table::Instructions::LDA(pos.reference.clone()))
                }
                instructions::Registers::B => {
                    instructions.push(instruction_table::Instructions::LDB(pos.reference.clone()))
                }
                instructions::Registers::C => {
                    instructions.push(instruction_table::Instructions::LDC(pos.reference.clone()))
                }
                instructions::Registers::X => {
                    instructions.push(instruction_table::Instructions::LDX(pos.reference.clone()))
                }
                instructions::Registers::Y => {
                    instructions.push(instruction_table::Instructions::LDY(pos.reference.clone()))
                }
            }

            assembler.instructions.extend(instructions)
        }
        CoreTypes::AsKeyword(e) => {
            resolve_type(
                assembler,
                &e.target,
                instructions::Registers::A,
                target_page,
                dependencies,
            );

            match &e.rtype {
                ellie_core::definite::definers::DefinerCollecting::Generic(e) => {
                    if e.rtype == "int" {
                        assembler
                            .instructions
                            .push(instruction_table::Instructions::A2I(Instruction::implicit()));
                    } else if e.rtype == "float" {
                        assembler
                            .instructions
                            .push(instruction_table::Instructions::A2F(Instruction::implicit()));
                    } else if e.rtype == "double" {
                        assembler
                            .instructions
                            .push(instruction_table::Instructions::A2D(Instruction::implicit()));
                    } else if e.rtype == "bool" {
                        assembler
                            .instructions
                            .push(instruction_table::Instructions::A2O(Instruction::implicit()));
                    } else if e.rtype == "string" {
                        assembler
                            .instructions
                            .push(instruction_table::Instructions::A2S(Instruction::implicit()));
                    } else if e.rtype == "char" {
                        assembler
                            .instructions
                            .push(instruction_table::Instructions::A2C(Instruction::implicit()));
                    } else if e.rtype == "byte" {
                        assembler
                            .instructions
                            .push(instruction_table::Instructions::A2B(Instruction::implicit()));
                    }
                }
                _ => panic!("As conv parent generic not implemented yet"),
            };

            assembler
                .instructions
                .push(instruction_table::Instructions::LDB(
                    Instruction::indirect_a(),
                ));

            match target_register {
                instructions::Registers::A => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDA(
                            Instruction::indirect_b(),
                        ))
                }
                instructions::Registers::B => (),
                instructions::Registers::C => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDC(
                            Instruction::indirect_b(),
                        ))
                }
                instructions::Registers::X => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDX(
                            Instruction::indirect_b(),
                        ))
                }
                instructions::Registers::Y => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDY(
                            Instruction::indirect_b(),
                        ))
                }
            }
        }
        CoreTypes::Byte(_) => {
            let converted_type = convert_type(
                types,
                dependencies,
                assembler.platform_attributes.architecture,
            );
            match target_register {
                instructions::Registers::A => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDA(
                            Instruction::immediate(converted_type.0, converted_type.1),
                        ))
                }
                instructions::Registers::B => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDB(
                            Instruction::immediate(converted_type.0, converted_type.1),
                        ))
                }
                instructions::Registers::C => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDC(
                            Instruction::immediate(converted_type.0, converted_type.1),
                        ))
                }
                instructions::Registers::X => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDX(
                            Instruction::immediate(converted_type.0, converted_type.1),
                        ))
                }
                instructions::Registers::Y => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDY(
                            Instruction::immediate(converted_type.0, converted_type.1),
                        ))
                }
            }
        }
        CoreTypes::Integer(_) => {
            let converted_type = convert_type(
                types,
                dependencies,
                assembler.platform_attributes.architecture,
            );
            match target_register {
                instructions::Registers::A => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDA(
                            Instruction::immediate(converted_type.0, converted_type.1),
                        ))
                }
                instructions::Registers::B => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDB(
                            Instruction::immediate(converted_type.0, converted_type.1),
                        ))
                }
                instructions::Registers::C => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDC(
                            Instruction::immediate(converted_type.0, converted_type.1),
                        ))
                }
                instructions::Registers::X => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDX(
                            Instruction::immediate(converted_type.0, converted_type.1),
                        ))
                }
                instructions::Registers::Y => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDY(
                            Instruction::immediate(converted_type.0, converted_type.1),
                        ))
                }
            }
        }
        CoreTypes::Decimal(_) => {
            let converted_type = convert_type(
                types,
                dependencies,
                assembler.platform_attributes.architecture,
            );

            match target_register {
                instructions::Registers::A => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDA(
                            Instruction::immediate(converted_type.0, converted_type.1),
                        ))
                }
                instructions::Registers::B => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDB(
                            Instruction::immediate(converted_type.0, converted_type.1),
                        ))
                }
                instructions::Registers::C => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDC(
                            Instruction::immediate(converted_type.0, converted_type.1),
                        ))
                }
                instructions::Registers::X => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDX(
                            Instruction::immediate(converted_type.0, converted_type.1),
                        ))
                }
                instructions::Registers::Y => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDY(
                            Instruction::immediate(converted_type.0, converted_type.1),
                        ))
                }
            }
        }
        CoreTypes::Bool(_) => {
            let converted_type = convert_type(
                types,
                dependencies,
                assembler.platform_attributes.architecture,
            );

            match target_register {
                instructions::Registers::A => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDA(
                            Instruction::immediate(converted_type.0, converted_type.1),
                        ))
                }
                instructions::Registers::B => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDB(
                            Instruction::immediate(converted_type.0, converted_type.1),
                        ))
                }
                instructions::Registers::C => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDC(
                            Instruction::immediate(converted_type.0, converted_type.1),
                        ))
                }
                instructions::Registers::X => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDX(
                            Instruction::immediate(converted_type.0, converted_type.1),
                        ))
                }
                instructions::Registers::Y => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDY(
                            Instruction::immediate(converted_type.0, converted_type.1),
                        ))
                }
            }
        }
        CoreTypes::String(e) => {
            //Create heap array
            assembler
                .instructions
                .push(instruction_table::Instructions::STR(Instruction::implicit()));
            let array_location = assembler.location();

            for char in e.value.chars() {
                assembler
                    .instructions
                    .push(instruction_table::Instructions::STA(
                        Instruction::immediate(Types::Char, (char as u32).to_le_bytes().to_vec()),
                    ));
                assembler
                    .instructions
                    .push(instruction_table::Instructions::SPUS(
                        Instruction::absolute(array_location),
                    ))
            }

            match target_register {
                instructions::Registers::A => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDA(Instruction::absolute(
                            array_location,
                        )))
                }
                instructions::Registers::B => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDB(Instruction::absolute(
                            array_location,
                        )))
                }
                instructions::Registers::C => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDC(Instruction::absolute(
                            array_location,
                        )))
                }
                instructions::Registers::X => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDX(Instruction::absolute(
                            array_location,
                        )))
                }
                instructions::Registers::Y => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDY(Instruction::absolute(
                            array_location,
                        )))
                }
            }
        }
        CoreTypes::Char(_) => {
            let converted_type = convert_type(
                types,
                dependencies,
                assembler.platform_attributes.architecture,
            );

            match target_register {
                instructions::Registers::A => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDA(
                            Instruction::immediate(converted_type.0, converted_type.1),
                        ))
                }
                instructions::Registers::B => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDB(
                            Instruction::immediate(converted_type.0, converted_type.1),
                        ))
                }
                instructions::Registers::C => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDC(
                            Instruction::immediate(converted_type.0, converted_type.1),
                        ))
                }
                instructions::Registers::X => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDX(
                            Instruction::immediate(converted_type.0, converted_type.1),
                        ))
                }
                instructions::Registers::Y => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDY(
                            Instruction::immediate(converted_type.0, converted_type.1),
                        ))
                }
            }
        }
        CoreTypes::Null => match target_register {
            instructions::Registers::A => {
                assembler
                    .instructions
                    .push(instruction_table::Instructions::LDA(
                        Instruction::immediate(Types::Null, Vec::new()),
                    ))
            }
            instructions::Registers::B => {
                assembler
                    .instructions
                    .push(instruction_table::Instructions::LDB(
                        Instruction::immediate(Types::Null, Vec::new()),
                    ))
            }
            instructions::Registers::C => {
                assembler
                    .instructions
                    .push(instruction_table::Instructions::LDC(
                        Instruction::immediate(Types::Null, Vec::new()),
                    ))
            }
            instructions::Registers::X => {
                assembler
                    .instructions
                    .push(instruction_table::Instructions::LDX(
                        Instruction::immediate(Types::Null, Vec::new()),
                    ))
            }
            instructions::Registers::Y => {
                assembler
                    .instructions
                    .push(instruction_table::Instructions::LDY(
                        Instruction::immediate(Types::Null, Vec::new()),
                    ))
            }
        },
        CoreTypes::Dynamic => todo!(),
        CoreTypes::SetterCall(_) => todo!(),
        CoreTypes::EnumData(_) => todo!(),
        CoreTypes::ClassInstance(_) => todo!(),
    }
}
