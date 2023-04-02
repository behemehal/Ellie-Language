use alloc::vec::Vec;
use ellie_core::definite::{types::operator, types::Types as CoreTypes};

use crate::{
    assembler::Assembler,
    instruction_table,
    instructions::{self, Instruction},
    types::Types,
};

pub fn convert_type(types: &CoreTypes, _page_hash: Option<Vec<usize>>) -> (Types, [u8; 8]) {
    match types {
        CoreTypes::Byte(byte) => (
            Types::Byte,
            [byte.value.to_le_bytes()[0], 0, 0, 0, 0, 0, 0, 0],
        ),
        CoreTypes::Integer(integer) => (Types::Integer, integer.value.to_le_bytes()),
        CoreTypes::Decimal(decimal) => match decimal.value {
            ellie_core::definite::types::decimal::DecimalTypeEnum::Float(float_value) => {
                let float_bytes = float_value.to_le_bytes();
                (
                    Types::Float,
                    [
                        float_bytes[0],
                        float_bytes[1],
                        float_bytes[2],
                        float_bytes[3],
                        0,
                        0,
                        0,
                        0,
                    ],
                )
            }
            ellie_core::definite::types::decimal::DecimalTypeEnum::Double(double_value) => {
                (Types::Double, double_value.to_le_bytes())
            }
        },
        CoreTypes::Bool(bool) => (Types::Bool, [bool.value as u8, 0, 0, 0, 0, 0, 0, 0]),
        CoreTypes::String(_) => {
            todo!()
        }
        CoreTypes::Char(e) => {
            let char_bytes = (e.value as u32).to_le_bytes();
            (
                Types::Char,
                [
                    char_bytes[0],
                    char_bytes[1],
                    char_bytes[2],
                    char_bytes[3],
                    0,
                    0,
                    0,
                    0,
                ],
            )
        }
        CoreTypes::Collective(_) => todo!(),
        CoreTypes::Reference(_) => todo!(),
        CoreTypes::BraceReference(_) => todo!(),
        CoreTypes::Operator(_) => todo!(),
        CoreTypes::Cloak(_) => todo!(),
        CoreTypes::Array(_) => todo!(),
        CoreTypes::Function(_) => todo!(),
        CoreTypes::ClassCall(_) => todo!(),
        CoreTypes::FunctionCall(_) => todo!(),
        CoreTypes::SetterCall(_) => todo!(),
        CoreTypes::Void => todo!(),
        CoreTypes::NullResolver(_) => todo!(),
        CoreTypes::Negative(_) => todo!(),
        CoreTypes::VariableType(_) => todo!(),
        CoreTypes::AsKeyword(_) => todo!(),
        CoreTypes::Null => todo!(),
        CoreTypes::Dynamic => todo!(),
        CoreTypes::EnumData(_) => todo!(),
        CoreTypes::ClassInstance(_) => todo!(),
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
                /// first index's reference defined above
                //if idx == 0 {
                //    assembler
                //        .instructions
                //        .push(instruction_table::Instructions::STC(Instruction::implicit()));
                //    last_pos = assembler.location();
                //}
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
                    Instruction::immediate(Types::StaticArray(size), size.to_le_bytes()),
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
                    let _idx = class_call.params.len() - idx;
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
                                class_location.to_le_bytes(),
                            ),
                        ))
                }
                instructions::Registers::B => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDB(
                            Instruction::immediate(
                                Types::Class(class_call.params.len()),
                                class_location.to_le_bytes(),
                            ),
                        ))
                }
                instructions::Registers::C => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDC(
                            Instruction::immediate(
                                Types::Class(class_call.params.len()),
                                class_location.to_le_bytes(),
                            ),
                        ))
                }
                instructions::Registers::X => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDX(
                            Instruction::immediate(
                                Types::Class(class_call.params.len()),
                                class_location.to_le_bytes(),
                            ),
                        ))
                }
                instructions::Registers::Y => {
                    assembler
                        .instructions
                        .push(instruction_table::Instructions::LDY(
                            Instruction::immediate(
                                Types::Class(class_call.params.len()),
                                class_location.to_le_bytes(),
                            ),
                        ))
                }
            }
        }
        CoreTypes::FunctionCall(function_call) => {
            let target_local = match *function_call.target.clone() {
                CoreTypes::VariableType(e) => e.value,
                CoreTypes::Reference(e) => e.chain.last().unwrap().value.clone(),
                _ => unreachable!("{:?}", function_call.target),
            };

            let target = assembler
                .find_local(&target_local, dependencies.clone())
                .unwrap()
                .clone();

            let previous_params_location = assembler.location() + 1;
            for _ in &function_call.params {
                assembler
                    .instructions
                    .push(instruction_table::Instructions::STB(Instruction::implicit()));
            }

            if !function_call.params.is_empty() {
                for (idx, param) in function_call.params.iter().enumerate() {
                    let _idx = function_call.params.len() - idx;
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
                    Instruction::immediate(Types::Integer, previous_params_location.to_le_bytes()),
                ));

            assembler
                .instructions
                .push(instruction_table::Instructions::CALL(
                    Instruction::absolute(target.cursor),
                ));

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
                        Instruction::immediate(Types::Void, [0, 0, 0, 0, 0, 0, 0, 0]),
                    ))
            }
            instructions::Registers::B => {
                assembler
                    .instructions
                    .push(instruction_table::Instructions::LDB(
                        Instruction::immediate(Types::Void, [0, 0, 0, 0, 0, 0, 0, 0]),
                    ))
            }
            instructions::Registers::C => {
                assembler
                    .instructions
                    .push(instruction_table::Instructions::LDC(
                        Instruction::immediate(Types::Void, [0, 0, 0, 0, 0, 0, 0, 0]),
                    ))
            }
            instructions::Registers::X => {
                assembler
                    .instructions
                    .push(instruction_table::Instructions::LDX(
                        Instruction::immediate(Types::Void, [0, 0, 0, 0, 0, 0, 0, 0]),
                    ))
            }
            instructions::Registers::Y => {
                assembler
                    .instructions
                    .push(instruction_table::Instructions::LDY(
                        Instruction::immediate(Types::Void, [0, 0, 0, 0, 0, 0, 0, 0]),
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
            let pos = assembler.find_local(&e.value, dependencies).unwrap();
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
            let converted_type = convert_type(types, dependencies);
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
            let converted_type = convert_type(types, dependencies);
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
            let converted_type = convert_type(types, dependencies);

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
            let converted_type = convert_type(types, dependencies);

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
                let char_bytes = (char as u32).to_le_bytes();
                let bytes = [
                    char_bytes[0],
                    char_bytes[1],
                    char_bytes[2],
                    char_bytes[3],
                    0,
                    0,
                    0,
                    0,
                ];
                assembler
                    .instructions
                    .push(instruction_table::Instructions::STA(
                        Instruction::immediate(Types::Char, bytes),
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
            let converted_type = convert_type(types, dependencies);

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
                        Instruction::immediate(Types::Null, [0, 0, 0, 0, 0, 0, 0, 0]),
                    ))
            }
            instructions::Registers::B => {
                assembler
                    .instructions
                    .push(instruction_table::Instructions::LDB(
                        Instruction::immediate(Types::Null, [0, 0, 0, 0, 0, 0, 0, 0]),
                    ))
            }
            instructions::Registers::C => {
                assembler
                    .instructions
                    .push(instruction_table::Instructions::LDC(
                        Instruction::immediate(Types::Null, [0, 0, 0, 0, 0, 0, 0, 0]),
                    ))
            }
            instructions::Registers::X => {
                assembler
                    .instructions
                    .push(instruction_table::Instructions::LDX(
                        Instruction::immediate(Types::Null, [0, 0, 0, 0, 0, 0, 0, 0]),
                    ))
            }
            instructions::Registers::Y => {
                assembler
                    .instructions
                    .push(instruction_table::Instructions::LDY(
                        Instruction::immediate(Types::Null, [0, 0, 0, 0, 0, 0, 0, 0]),
                    ))
            }
        },
        CoreTypes::Dynamic => todo!(),
        CoreTypes::SetterCall(_) => todo!(),
        CoreTypes::EnumData(_) => todo!(),
        CoreTypes::ClassInstance(_) => todo!(),
    }
}
