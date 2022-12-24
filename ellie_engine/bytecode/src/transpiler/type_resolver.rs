use alloc::vec;
use alloc::vec::Vec;
use ellie_core::definite::types::{operator, Types};

use crate::{
    assembler::Assembler,
    instructions::{self, Instruction},
};

pub fn convert_type(
    types: &Types,
    _page_hash: Option<Vec<usize>>,
) -> (instructions::Types, Vec<u8>) {
    match types {
        Types::Byte(byte) => (instructions::Types::Byte, byte.value.to_le_bytes().to_vec()),
        Types::Integer(integer) => (
            instructions::Types::Integer,
            integer.value.to_le_bytes().to_vec(),
        ),
        Types::Float(float) => (
            instructions::Types::Float,
            float.value.to_le_bytes().to_vec(),
        ),
        Types::Double(double) => (
            instructions::Types::Double,
            double.value.to_le_bytes().to_vec(),
        ),
        Types::Bool(bool) => (instructions::Types::Bool, vec![bool.value as u8]),
        Types::String(string) => (
            instructions::Types::String(string.value.len()),
            string.value.as_bytes().to_vec(),
        ),
        Types::Char(_) => (instructions::Types::Char, vec![]),
        Types::Collective(_) => todo!(),
        Types::Reference(_) => todo!(),
        Types::BraceReference(_) => todo!(),
        Types::Operator(_) => todo!(),
        Types::Cloak(_) => todo!(),
        Types::Array(array) => (instructions::Types::Array(array.collective.len()), vec![]),
        Types::Function(_) => todo!(),
        Types::ClassCall(_) => todo!(),
        Types::FunctionCall(_) => todo!(),
        Types::SetterCall(_) => todo!(),
        Types::Void => todo!(),
        Types::NullResolver(_) => todo!(),
        Types::Negative(_) => todo!(),
        Types::VariableType(_) => todo!(),
        Types::AsKeyword(_) => todo!(),
        Types::Null => todo!(),
        Types::Dynamic => todo!(),
        Types::EnumData(_) => todo!(),
        Types::ClassInstance(_) => todo!(),
    }
}

pub fn resolve_type(
    assembler: &mut Assembler,
    types: &Types,
    target_register: instructions::Registers,
    target_page: &usize,
    dependencies: Option<Vec<usize>>,
) {
    match types {
        Types::Collective(_) => todo!(),
        Types::Reference(_) => todo!(),
        Types::BraceReference(e) => {
            resolve_type(
                assembler,
                &e.reference,
                instructions::Registers::B,
                target_page,
                dependencies.clone(),
            );
            assembler
                .instructions
                .push(instructions::Instructions::STB(Instruction::implicit()));

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
                .push(instructions::Instructions::STC(Instruction::implicit()));

            match target_register {
                instructions::Registers::A => assembler.instructions.push(
                    instructions::Instructions::LDA(instructions::Instruction::absolute_index(
                        location_of_pointer,
                        assembler.location(),
                    )),
                ),
                instructions::Registers::B => assembler.instructions.push(
                    instructions::Instructions::LDB(instructions::Instruction::absolute_index(
                        location_of_pointer,
                        assembler.location(),
                    )),
                ),
                instructions::Registers::C => assembler.instructions.push(
                    instructions::Instructions::LDC(instructions::Instruction::absolute_index(
                        location_of_pointer,
                        assembler.location(),
                    )),
                ),
                instructions::Registers::X => assembler.instructions.push(
                    instructions::Instructions::LDX(instructions::Instruction::absolute_index(
                        location_of_pointer,
                        assembler.location(),
                    )),
                ),
                instructions::Registers::Y => assembler.instructions.push(
                    instructions::Instructions::LDY(instructions::Instruction::absolute_index(
                        location_of_pointer,
                        assembler.location(),
                    )),
                ),
            }
        }
        Types::Operator(operator) => match &operator.operator {
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
                    .push(instructions::Instructions::STB(Instruction::implicit()));

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
                    .push(instructions::Instructions::STC(Instruction::implicit()));

                assembler.instructions.push(instructions::Instructions::LDB(
                    Instruction::absolute(first_operator_pos),
                ));
                assembler.instructions.push(instructions::Instructions::LDC(
                    Instruction::absolute(second_operator_pos),
                ));

                assembler.instructions.push(match e {
                    operator::ComparisonOperators::Equal => {
                        instructions::Instructions::EQ(Instruction::implicit())
                    }
                    operator::ComparisonOperators::NotEqual => {
                        instructions::Instructions::NE(Instruction::implicit())
                    }
                    operator::ComparisonOperators::GreaterThan => {
                        instructions::Instructions::GT(Instruction::implicit())
                    }
                    operator::ComparisonOperators::LessThan => {
                        instructions::Instructions::LT(Instruction::implicit())
                    }
                    operator::ComparisonOperators::GreaterThanOrEqual => {
                        instructions::Instructions::GQ(Instruction::implicit())
                    }
                    operator::ComparisonOperators::LessThanOrEqual => {
                        instructions::Instructions::LQ(Instruction::implicit())
                    }
                    operator::ComparisonOperators::Null => unreachable!(),
                });

                match target_register {
                    instructions::Registers::A => (),
                    instructions::Registers::B => {
                        assembler
                            .instructions
                            .push(instructions::Instructions::LDB(Instruction::indirect_a()));
                    }
                    instructions::Registers::C => {
                        assembler
                            .instructions
                            .push(instructions::Instructions::LDC(Instruction::indirect_a()));
                    }
                    instructions::Registers::X => {
                        assembler
                            .instructions
                            .push(instructions::Instructions::LDX(Instruction::indirect_a()));
                    }
                    instructions::Registers::Y => {
                        assembler
                            .instructions
                            .push(instructions::Instructions::LDY(Instruction::indirect_a()));
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
                    .push(instructions::Instructions::STC(Instruction::implicit()));

                resolve_type(
                    assembler,
                    &operator.first,
                    instructions::Registers::B,
                    target_page,
                    dependencies,
                );

                assembler.instructions.push(instructions::Instructions::LDC(
                    Instruction::absolute(second_operator_pos),
                ));

                assembler.instructions.push(match e {
                    operator::LogicalOperators::And => {
                        instructions::Instructions::AND(Instruction::implicit())
                    }
                    operator::LogicalOperators::Or => {
                        instructions::Instructions::OR(Instruction::implicit())
                    }
                    _ => unreachable!(),
                });
                match target_register {
                    instructions::Registers::A => (),
                    instructions::Registers::B => {
                        assembler
                            .instructions
                            .push(instructions::Instructions::LDB(Instruction::indirect_a()));
                    }
                    instructions::Registers::C => {
                        assembler
                            .instructions
                            .push(instructions::Instructions::LDC(Instruction::indirect_a()));
                    }
                    instructions::Registers::X => {
                        assembler
                            .instructions
                            .push(instructions::Instructions::LDX(Instruction::indirect_a()));
                    }
                    instructions::Registers::Y => {
                        assembler
                            .instructions
                            .push(instructions::Instructions::LDY(Instruction::indirect_a()));
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
                    .push(instructions::Instructions::STB(Instruction::implicit()));

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
                    .push(instructions::Instructions::STC(Instruction::implicit()));

                assembler.instructions.push(instructions::Instructions::LDB(
                    Instruction::absolute(first_operator_pos),
                ));
                assembler.instructions.push(instructions::Instructions::LDC(
                    Instruction::absolute(second_operator_pos),
                ));

                assembler.instructions.push(match e {
                    operator::ArithmeticOperators::Addition => {
                        instructions::Instructions::ADD(Instruction::implicit())
                    }
                    operator::ArithmeticOperators::Subtraction => {
                        instructions::Instructions::SUB(Instruction::implicit())
                    }
                    operator::ArithmeticOperators::Multiplication => {
                        instructions::Instructions::MUL(Instruction::implicit())
                    }
                    operator::ArithmeticOperators::Exponentiation => {
                        instructions::Instructions::EXP(Instruction::implicit())
                    }
                    operator::ArithmeticOperators::Division => {
                        instructions::Instructions::DIV(Instruction::implicit())
                    }
                    operator::ArithmeticOperators::Modulus => {
                        instructions::Instructions::MOD(Instruction::implicit())
                    }
                    operator::ArithmeticOperators::Null => unreachable!("Wrong operator"),
                });
                match target_register {
                    instructions::Registers::A => (),
                    instructions::Registers::B => {
                        assembler
                            .instructions
                            .push(instructions::Instructions::LDB(Instruction::indirect_a()));
                    }
                    instructions::Registers::C => {
                        assembler
                            .instructions
                            .push(instructions::Instructions::LDC(Instruction::indirect_a()));
                    }
                    instructions::Registers::X => {
                        assembler
                            .instructions
                            .push(instructions::Instructions::LDX(Instruction::indirect_a()));
                    }
                    instructions::Registers::Y => {
                        assembler
                            .instructions
                            .push(instructions::Instructions::LDY(Instruction::indirect_a()));
                    }
                }
            }
            operator::Operators::AssignmentType(_) => todo!(),
            operator::Operators::Null => unreachable!(),
        },
        Types::Cloak(e) => {
            let mut entries = vec![];

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
                    .push(instructions::Instructions::STA(Instruction::implicit()));
                let location_of_data = assembler.instructions.len() - 1;
                entries.extend(location_of_data.to_le_bytes());
            }

            let array_rtype = instructions::Types::Array(entries.len());

            match target_register {
                instructions::Registers::A => assembler.instructions.push(
                    instructions::Instructions::LDA(Instruction::immediate(array_rtype, entries)),
                ),
                instructions::Registers::B => assembler.instructions.push(
                    instructions::Instructions::LDB(Instruction::immediate(array_rtype, entries)),
                ),
                instructions::Registers::C => assembler.instructions.push(
                    instructions::Instructions::LDC(Instruction::immediate(array_rtype, entries)),
                ),
                instructions::Registers::X => assembler.instructions.push(
                    instructions::Instructions::LDX(Instruction::immediate(array_rtype, entries)),
                ),
                instructions::Registers::Y => assembler.instructions.push(
                    instructions::Instructions::LDY(Instruction::immediate(array_rtype, entries)),
                ),
            }
        }
        Types::Array(e) => {
            let mut entries = vec![];

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
                    .push(instructions::Instructions::STA(Instruction::implicit()));
                let location_of_data = assembler.instructions.len() - 1;
                entries.extend(location_of_data.to_le_bytes());
            }

            let array_rtype = instructions::Types::Array(entries.len());

            match target_register {
                instructions::Registers::A => assembler.instructions.push(
                    instructions::Instructions::LDA(Instruction::immediate(array_rtype, entries)),
                ),
                instructions::Registers::B => assembler.instructions.push(
                    instructions::Instructions::LDB(Instruction::immediate(array_rtype, entries)),
                ),
                instructions::Registers::C => assembler.instructions.push(
                    instructions::Instructions::LDC(Instruction::immediate(array_rtype, entries)),
                ),
                instructions::Registers::X => assembler.instructions.push(
                    instructions::Instructions::LDX(Instruction::immediate(array_rtype, entries)),
                ),
                instructions::Registers::Y => assembler.instructions.push(
                    instructions::Instructions::LDY(Instruction::immediate(array_rtype, entries)),
                ),
            }
        }
        Types::Function(_) => todo!(),
        Types::ClassCall(_) => {
            todo!()
        }
        Types::FunctionCall(function_call) => {
            let target_local = match *function_call.target.clone() {
                Types::VariableType(e) => e.value,
                _ => unreachable!(),
            };

            let target = assembler
                .find_local(&target_local, dependencies.clone())
                .unwrap()
                .clone();

            assembler
                .instructions
                .push(instructions::Instructions::STX(Instruction::implicit()));

            let previous_params_location = assembler.location();

            if !function_call.params.is_empty() {
                assembler.instructions.push(instructions::Instructions::LDA(
                    Instruction::immediate(instructions::Types::Array(0), vec![]),
                ));
                assembler
                    .instructions
                    .push(instructions::Instructions::STA(Instruction::implicit()));
                let params_location = assembler.location();

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
                        .push(instructions::Instructions::STA(Instruction::implicit()));
                    assembler
                        .instructions
                        .push(instructions::Instructions::PUSH(Instruction::absolute(
                            params_location,
                        )));

                    /*
                    //Functions always reserve parameter spaces we're writing upper locations of function
                    assembler.instructions.push(instructions::Instructions::STA(
                        Instruction::immediate(
                            instructions::Types::Integer,
                            idx.to_le_bytes().to_vec(),
                        ),
                    ));
                    assembler.instructions.push(instructions::Instructions::STA(
                        Instruction::absolute_index(target.cursor, assembler.location()),
                    ));
                    */
                }
                assembler.instructions.push(instructions::Instructions::LDX(
                    Instruction::absolute(params_location),
                ));
            }

            /*
            assembler.instructions.push(instructions::Instructions::LDA(
                Instruction::immediate(
                    instructions::Types::Array(array_pointers.len()),
                    array_pointers,
                ),
            ));
            assembler
                .instructions
                .push(instructions::Instructions::STA(Instruction::absolute(
                    target.cursor,
                )));
                */

            assembler
                .instructions
                .push(instructions::Instructions::CALL(Instruction::absolute(
                    target.cursor,
                )));
            assembler
                .instructions
                .push(instructions::Instructions::LDX(Instruction::absolute(
                    previous_params_location,
                )));

            match target_register {
                instructions::Registers::A => {
                    assembler
                        .instructions
                        .push(instructions::Instructions::LDA(Instruction::indirect_y()));
                }
                instructions::Registers::B => {
                    assembler
                        .instructions
                        .push(instructions::Instructions::LDB(Instruction::indirect_y()));
                }
                instructions::Registers::C => {
                    assembler
                        .instructions
                        .push(instructions::Instructions::LDC(Instruction::indirect_y()));
                }
                instructions::Registers::X => {
                    assembler
                        .instructions
                        .push(instructions::Instructions::LDX(Instruction::indirect_y()));
                }
                instructions::Registers::Y => (),
            }
        }
        Types::Void => match target_register {
            instructions::Registers::A => {
                assembler.instructions.push(instructions::Instructions::LDA(
                    Instruction::immediate(crate::instructions::Types::Void, vec![]),
                ))
            }
            instructions::Registers::B => {
                assembler.instructions.push(instructions::Instructions::LDB(
                    Instruction::immediate(crate::instructions::Types::Void, vec![]),
                ))
            }
            instructions::Registers::C => {
                assembler.instructions.push(instructions::Instructions::LDC(
                    Instruction::immediate(crate::instructions::Types::Void, vec![]),
                ))
            }
            instructions::Registers::X => {
                assembler.instructions.push(instructions::Instructions::LDX(
                    Instruction::immediate(crate::instructions::Types::Void, vec![]),
                ))
            }
            instructions::Registers::Y => {
                assembler.instructions.push(instructions::Instructions::LDY(
                    Instruction::immediate(crate::instructions::Types::Void, vec![]),
                ))
            }
        },
        Types::NullResolver(e) => {
            resolve_type(
                assembler,
                &e.target,
                target_register,
                target_page,
                dependencies.clone(),
            );
        }
        Types::Negative(_) => todo!(),
        Types::VariableType(e) => {
            let pos = assembler.find_local(&e.value, dependencies).unwrap();

            let mut instructions = Vec::new();

            match target_register {
                instructions::Registers::A => {
                    instructions.push(instructions::Instructions::LDA(pos.reference.clone()))
                }
                instructions::Registers::B => {
                    instructions.push(instructions::Instructions::LDB(pos.reference.clone()))
                }
                instructions::Registers::C => {
                    instructions.push(instructions::Instructions::LDC(pos.reference.clone()))
                }
                instructions::Registers::X => {
                    instructions.push(instructions::Instructions::LDX(pos.reference.clone()))
                }
                instructions::Registers::Y => {
                    instructions.push(instructions::Instructions::LDY(pos.reference.clone()))
                }
            }

            assembler.instructions.extend(instructions)
        }
        Types::AsKeyword(e) => {
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
                            .push(instructions::Instructions::A2I(Instruction::implicit()));
                    } else if e.rtype == "float" {
                        assembler
                            .instructions
                            .push(instructions::Instructions::A2F(Instruction::implicit()));
                    } else if e.rtype == "double" {
                        assembler
                            .instructions
                            .push(instructions::Instructions::A2D(Instruction::implicit()));
                    } else if e.rtype == "bool" {
                        assembler
                            .instructions
                            .push(instructions::Instructions::A2O(Instruction::implicit()));
                    } else if e.rtype == "string" {
                        assembler
                            .instructions
                            .push(instructions::Instructions::A2S(Instruction::implicit()));
                    } else if e.rtype == "char" {
                        assembler
                            .instructions
                            .push(instructions::Instructions::A2C(Instruction::implicit()));
                    } else if e.rtype == "byte" {
                        assembler
                            .instructions
                            .push(instructions::Instructions::A2B(Instruction::implicit()));
                    }
                }
                _ => panic!("As conv parent generic not implemented yet"),
            };

            assembler
                .instructions
                .push(instructions::Instructions::LDB(Instruction::indirect_a()));

            match target_register {
                instructions::Registers::A => assembler
                    .instructions
                    .push(instructions::Instructions::LDA(Instruction::indirect_b())),
                instructions::Registers::B => (),
                instructions::Registers::C => assembler
                    .instructions
                    .push(instructions::Instructions::LDC(Instruction::indirect_b())),
                instructions::Registers::X => assembler
                    .instructions
                    .push(instructions::Instructions::LDX(Instruction::indirect_b())),
                instructions::Registers::Y => assembler
                    .instructions
                    .push(instructions::Instructions::LDY(Instruction::indirect_b())),
            }
        }
        Types::Byte(_) => {
            let converted_type = convert_type(types, dependencies);
            match target_register {
                instructions::Registers::A => {
                    assembler.instructions.push(instructions::Instructions::LDA(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::B => {
                    assembler.instructions.push(instructions::Instructions::LDB(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::C => {
                    assembler.instructions.push(instructions::Instructions::LDC(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::X => {
                    assembler.instructions.push(instructions::Instructions::LDX(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::Y => {
                    assembler.instructions.push(instructions::Instructions::LDY(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
            }
        }
        Types::Integer(_) => {
            let converted_type = convert_type(types, dependencies);

            match target_register {
                instructions::Registers::A => {
                    assembler.instructions.push(instructions::Instructions::LDA(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::B => {
                    assembler.instructions.push(instructions::Instructions::LDB(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::C => {
                    assembler.instructions.push(instructions::Instructions::LDC(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::X => {
                    assembler.instructions.push(instructions::Instructions::LDX(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::Y => {
                    assembler.instructions.push(instructions::Instructions::LDY(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
            }
        }
        Types::Float(_) => {
            let converted_type = convert_type(types, dependencies);

            match target_register {
                instructions::Registers::A => {
                    assembler.instructions.push(instructions::Instructions::LDA(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::B => {
                    assembler.instructions.push(instructions::Instructions::LDB(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::C => {
                    assembler.instructions.push(instructions::Instructions::LDC(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::X => {
                    assembler.instructions.push(instructions::Instructions::LDX(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::Y => {
                    assembler.instructions.push(instructions::Instructions::LDY(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
            }
        }
        Types::Double(_) => {
            let converted_type = convert_type(types, dependencies);

            match target_register {
                instructions::Registers::A => {
                    assembler.instructions.push(instructions::Instructions::LDA(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::B => {
                    assembler.instructions.push(instructions::Instructions::LDB(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::C => {
                    assembler.instructions.push(instructions::Instructions::LDC(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::X => {
                    assembler.instructions.push(instructions::Instructions::LDX(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::Y => {
                    assembler.instructions.push(instructions::Instructions::LDY(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
            }
        }
        Types::Bool(_) => {
            let converted_type = convert_type(types, dependencies);

            match target_register {
                instructions::Registers::A => {
                    assembler.instructions.push(instructions::Instructions::LDA(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::B => {
                    assembler.instructions.push(instructions::Instructions::LDB(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::C => {
                    assembler.instructions.push(instructions::Instructions::LDC(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::X => {
                    assembler.instructions.push(instructions::Instructions::LDX(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::Y => {
                    assembler.instructions.push(instructions::Instructions::LDY(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
            }
        }
        Types::String(_) => {
            let converted_type = convert_type(types, dependencies);

            match target_register {
                instructions::Registers::A => {
                    assembler.instructions.push(instructions::Instructions::LDA(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::B => {
                    assembler.instructions.push(instructions::Instructions::LDB(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::C => {
                    assembler.instructions.push(instructions::Instructions::LDC(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::X => {
                    assembler.instructions.push(instructions::Instructions::LDX(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::Y => {
                    assembler.instructions.push(instructions::Instructions::LDY(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
            }
        }
        Types::Char(_) => {
            let converted_type = convert_type(types, dependencies);

            match target_register {
                instructions::Registers::A => {
                    assembler.instructions.push(instructions::Instructions::LDA(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::B => {
                    assembler.instructions.push(instructions::Instructions::LDB(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::C => {
                    assembler.instructions.push(instructions::Instructions::LDC(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::X => {
                    assembler.instructions.push(instructions::Instructions::LDX(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
                instructions::Registers::Y => {
                    assembler.instructions.push(instructions::Instructions::LDY(
                        Instruction::immediate(converted_type.0, converted_type.1),
                    ))
                }
            }
        }
        Types::Null => match target_register {
            instructions::Registers::A => {
                assembler.instructions.push(instructions::Instructions::LDA(
                    Instruction::immediate(crate::instructions::Types::Null, vec![]),
                ))
            }
            instructions::Registers::B => {
                assembler.instructions.push(instructions::Instructions::LDB(
                    Instruction::immediate(crate::instructions::Types::Null, vec![]),
                ))
            }
            instructions::Registers::C => {
                assembler.instructions.push(instructions::Instructions::LDC(
                    Instruction::immediate(crate::instructions::Types::Null, vec![]),
                ))
            }
            instructions::Registers::X => {
                assembler.instructions.push(instructions::Instructions::LDX(
                    Instruction::immediate(crate::instructions::Types::Null, vec![]),
                ))
            }
            instructions::Registers::Y => {
                assembler.instructions.push(instructions::Instructions::LDY(
                    Instruction::immediate(crate::instructions::Types::Null, vec![]),
                ))
            }
        },
        Types::Dynamic => todo!(),
        Types::SetterCall(_) => todo!(),
        Types::EnumData(_) => todo!(),
        Types::ClassInstance(_) => todo!(),
    }
}
