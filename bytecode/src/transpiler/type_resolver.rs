use alloc::vec;
use alloc::vec::Vec;
use ellie_core::definite::types::{operator, Types};

use crate::{
    assembler::Assembler,
    instructions::{self, Instruction, Instructions},
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
        Types::Vector(_) => todo!(),
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
            let target = resolve_type(
                assembler,
                &e.reference,
                instructions::Registers::B,
                target_page,
                dependencies.clone(),
            );

            let val = resolve_type(
                assembler,
                &e.value,
                instructions::Registers::C,
                target_page,
                dependencies.clone(),
            );
            assembler.instructions.push(instructions::Instructions::LDA(
                instructions::Instruction::absolute_index(),
            ));
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
        Types::Operator(operator) => match &operator.operator {
            operator::Operators::ComparisonType(e) => {
                resolve_type(
                    assembler,
                    &operator.second,
                    instructions::Registers::C,
                    target_page,
                    dependencies.clone(),
                );

                resolve_type(
                    assembler,
                    &operator.first,
                    instructions::Registers::B,
                    target_page,
                    dependencies,
                );

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
            operator::Operators::LogicalType(_) => todo!(),
            operator::Operators::ArithmeticType(e) => {
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
            operator::Operators::Null => todo!(),
        },
        Types::Cloak(_) => todo!(),
        Types::Array(_) => {
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
        Types::Vector(_) => todo!(),
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

            for (idx, param) in function_call.params.iter().enumerate() {
                resolve_type(
                    assembler,
                    &param.value,
                    instructions::Registers::A,
                    &target_page,
                    dependencies.clone(),
                );

                //Functions always reserve parameter spaces we're writing upper locations of function
                assembler.instructions.push(instructions::Instructions::STA(
                    Instruction::absolute(target.cursor - (idx + 1)),
                ));
            }

            assembler
                .instructions
                .push(instructions::Instructions::CALL(Instruction::absolute(
                    target.cursor,
                )));
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
        Types::NullResolver(_) => todo!(),
        Types::Negative(_) => todo!(),
        Types::VariableType(e) => {
            let pos = assembler.find_local(&e.value, dependencies).unwrap();

            let mut instructions = Vec::new();

            match target_register {
                instructions::Registers::A => match pos.reference {
                    Some(target_page) => {
                        instructions.push(instructions::Instructions::AOL(Instruction::absolute(
                            target_page,
                        )));
                        instructions.push(instructions::Instructions::LDA(Instruction::absolute(
                            pos.cursor,
                        )))
                    }
                    None => instructions.push(instructions::Instructions::LDA(
                        Instruction::absolute(pos.cursor),
                    )),
                },
                instructions::Registers::B => match pos.reference {
                    Some(target_page) => {
                        instructions.push(instructions::Instructions::AOL(Instruction::absolute(
                            target_page,
                        )));
                        instructions.push(instructions::Instructions::LDB(Instruction::absolute(
                            pos.cursor,
                        )))
                    }
                    None => instructions.push(instructions::Instructions::LDB(
                        Instruction::absolute(pos.cursor),
                    )),
                },
                instructions::Registers::C => match pos.reference {
                    Some(target_page) => {
                        instructions.push(instructions::Instructions::AOL(Instruction::absolute(
                            target_page,
                        )));
                        instructions.push(instructions::Instructions::LDC(Instruction::absolute(
                            pos.cursor,
                        )))
                    }
                    None => instructions.push(instructions::Instructions::LDC(
                        Instruction::absolute(pos.cursor),
                    )),
                },
                instructions::Registers::X => match pos.reference {
                    Some(target_page) => {
                        instructions.push(instructions::Instructions::AOL(Instruction::absolute(
                            target_page,
                        )));
                        instructions.push(instructions::Instructions::LDX(Instruction::absolute(
                            pos.cursor,
                        )))
                    }
                    None => instructions.push(instructions::Instructions::LDX(
                        Instruction::absolute(pos.cursor),
                    )),
                },
                instructions::Registers::Y => match pos.reference {
                    Some(target_page) => {
                        instructions.push(instructions::Instructions::AOL(Instruction::absolute(
                            target_page,
                        )));
                        instructions.push(instructions::Instructions::LDY(Instruction::absolute(
                            pos.cursor,
                        )))
                    }
                    None => instructions.push(instructions::Instructions::LDY(
                        Instruction::absolute(pos.cursor),
                    )),
                },
            };

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
    }
}

/*
pub fn _resolve_type(
    assembler: &mut Assembler,
    types: &Types,
    target_register: instructions::Registers,
    target_page: &usize,
    dependencies: Option<Vec<usize>>,
) -> Vec<Instructions> {
    match types {
        Types::Collective(_) => todo!(),
        Types::Reference(_) => todo!(),
        Types::BraceReference(e) => {
            let mut instructions = vec![];
            let target = resolve_type(
                assembler,
                &e.reference,
                instructions::Registers::B,
                target_page,
                dependencies.clone(),
            );
            instructions.extend(target);

            let val = resolve_type(
                assembler,
                &e.value,
                instructions::Registers::C,
                target_page,
                dependencies.clone(),
            );
            instructions.extend(val);
            instructions.push(instructions::Instructions::LDA(
                instructions::Instruction::absolute_index(),
            ));
            match target_register {
                instructions::Registers::A => (),
                instructions::Registers::B => {
                    instructions.push(instructions::Instructions::LDB(Instruction::indirect_a()));
                }
                instructions::Registers::C => {
                    instructions.push(instructions::Instructions::LDC(Instruction::indirect_a()));
                }
                instructions::Registers::X => {
                    instructions.push(instructions::Instructions::LDX(Instruction::indirect_a()));
                }
                instructions::Registers::Y => {
                    instructions.push(instructions::Instructions::LDY(Instruction::indirect_a()));
                }
            }
            instructions
        }
        Types::Operator(operator) => match &operator.operator {
            operator::Operators::ComparisonType(e) => {
                let mut instructions = Vec::new();

                instructions.extend(resolve_type(
                    assembler,
                    &operator.second,
                    instructions::Registers::C,
                    target_page,
                    dependencies.clone(),
                ));

                instructions.extend(resolve_type(
                    assembler,
                    &operator.first,
                    instructions::Registers::B,
                    target_page,
                    dependencies,
                ));

                instructions.push(match e {
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
                        instructions
                            .push(instructions::Instructions::LDB(Instruction::indirect_a()));
                    }
                    instructions::Registers::C => {
                        instructions
                            .push(instructions::Instructions::LDC(Instruction::indirect_a()));
                    }
                    instructions::Registers::X => {
                        instructions
                            .push(instructions::Instructions::LDX(Instruction::indirect_a()));
                    }
                    instructions::Registers::Y => {
                        instructions
                            .push(instructions::Instructions::LDY(Instruction::indirect_a()));
                    }
                }
                instructions
            }
            operator::Operators::LogicalType(_) => todo!(),
            operator::Operators::ArithmeticType(e) => {
                let mut instructions = Vec::new();
                instructions.extend(resolve_type(
                    assembler,
                    &operator.second,
                    instructions::Registers::C,
                    target_page,
                    dependencies.clone(),
                ));

                instructions.extend(resolve_type(
                    assembler,
                    &operator.first,
                    instructions::Registers::B,
                    target_page,
                    dependencies,
                ));

                instructions.push(match e {
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
                        instructions
                            .push(instructions::Instructions::LDB(Instruction::indirect_a()));
                    }
                    instructions::Registers::C => {
                        instructions
                            .push(instructions::Instructions::LDC(Instruction::indirect_a()));
                    }
                    instructions::Registers::X => {
                        instructions
                            .push(instructions::Instructions::LDX(Instruction::indirect_a()));
                    }
                    instructions::Registers::Y => {
                        instructions
                            .push(instructions::Instructions::LDY(Instruction::indirect_a()));
                    }
                }
                instructions
            }
            operator::Operators::AssignmentType(_) => todo!(),
            operator::Operators::Null => todo!(),
        },
        Types::Cloak(_) => todo!(),
        Types::Array(_) => {
            let converted_type = convert_type(types, dependencies);
            match target_register {
                instructions::Registers::A => vec![instructions::Instructions::LDA(
                    Instruction::immediate(converted_type.0, converted_type.1),
                )],
                instructions::Registers::B => vec![instructions::Instructions::LDB(
                    Instruction::immediate(converted_type.0, converted_type.1),
                )],
                instructions::Registers::C => vec![instructions::Instructions::LDC(
                    Instruction::immediate(converted_type.0, converted_type.1),
                )],
                instructions::Registers::X => vec![instructions::Instructions::LDX(
                    Instruction::immediate(converted_type.0, converted_type.1),
                )],
                instructions::Registers::Y => vec![instructions::Instructions::LDY(
                    Instruction::immediate(converted_type.0, converted_type.1),
                )],
            }
        }
        Types::Vector(_) => todo!(),
        Types::Function(_) => todo!(),
        Types::ClassCall(_) => {
            vec![]
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

            let mut instructions = Vec::new();

            for (idx, param) in function_call.params.iter().enumerate() {
                let resolved_instructions = resolve_type(
                    assembler,
                    &param.value,
                    instructions::Registers::A,
                    &target_page,
                    dependencies.clone(),
                );
                instructions.extend(resolved_instructions);
                //Functions always reserve parameter spaces we're writing upper locations of function
                instructions.push(instructions::Instructions::STA(Instruction::absolute(
                    target.cursor - (idx + 1),
                )));
            }

            instructions.push(instructions::Instructions::CALL(Instruction::absolute(
                target.cursor,
            )));
            match target_register {
                instructions::Registers::A => (),
                instructions::Registers::B => {
                    instructions.push(instructions::Instructions::LDB(Instruction::indirect_a()));
                }
                instructions::Registers::C => {
                    instructions.push(instructions::Instructions::LDC(Instruction::indirect_a()));
                }
                instructions::Registers::X => {
                    instructions.push(instructions::Instructions::LDX(Instruction::indirect_a()));
                }
                instructions::Registers::Y => {
                    instructions.push(instructions::Instructions::LDY(Instruction::indirect_a()));
                }
            }
            instructions
        }
        Types::Void => match target_register {
            instructions::Registers::A => {
                vec![instructions::Instructions::LDA(Instruction::immediate(
                    crate::instructions::Types::Void,
                    vec![],
                ))]
            }
            instructions::Registers::B => {
                vec![instructions::Instructions::LDB(Instruction::immediate(
                    crate::instructions::Types::Void,
                    vec![],
                ))]
            }
            instructions::Registers::C => {
                vec![instructions::Instructions::LDC(Instruction::immediate(
                    crate::instructions::Types::Void,
                    vec![],
                ))]
            }
            instructions::Registers::X => {
                vec![instructions::Instructions::LDX(Instruction::immediate(
                    crate::instructions::Types::Void,
                    vec![],
                ))]
            }
            instructions::Registers::Y => {
                vec![instructions::Instructions::LDY(Instruction::immediate(
                    crate::instructions::Types::Void,
                    vec![],
                ))]
            }
        },
        Types::NullResolver(_) => todo!(),
        Types::Negative(_) => todo!(),
        Types::VariableType(e) => {
            let pos = assembler.find_local(&e.value, dependencies).unwrap();

            let mut instructions = Vec::new();

            match target_register {
                instructions::Registers::A => match pos.reference {
                    Some(target_page) => {
                        instructions.push(instructions::Instructions::AOL(Instruction::absolute(
                            target_page,
                        )));
                        instructions.push(instructions::Instructions::LDA(Instruction::absolute(
                            pos.cursor,
                        )))
                    }
                    None => instructions.push(instructions::Instructions::LDA(
                        Instruction::absolute(pos.cursor),
                    )),
                },
                instructions::Registers::B => match pos.reference {
                    Some(target_page) => {
                        instructions.push(instructions::Instructions::AOL(Instruction::absolute(
                            target_page,
                        )));
                        instructions.push(instructions::Instructions::LDB(Instruction::absolute(
                            pos.cursor,
                        )))
                    }
                    None => instructions.push(instructions::Instructions::LDB(
                        Instruction::absolute(pos.cursor),
                    )),
                },
                instructions::Registers::C => match pos.reference {
                    Some(target_page) => {
                        instructions.push(instructions::Instructions::AOL(Instruction::absolute(
                            target_page,
                        )));
                        instructions.push(instructions::Instructions::LDC(Instruction::absolute(
                            pos.cursor,
                        )))
                    }
                    None => instructions.push(instructions::Instructions::LDC(
                        Instruction::absolute(pos.cursor),
                    )),
                },
                instructions::Registers::X => match pos.reference {
                    Some(target_page) => {
                        instructions.push(instructions::Instructions::AOL(Instruction::absolute(
                            target_page,
                        )));
                        instructions.push(instructions::Instructions::LDX(Instruction::absolute(
                            pos.cursor,
                        )))
                    }
                    None => instructions.push(instructions::Instructions::LDX(
                        Instruction::absolute(pos.cursor),
                    )),
                },
                instructions::Registers::Y => match pos.reference {
                    Some(target_page) => {
                        instructions.push(instructions::Instructions::AOL(Instruction::absolute(
                            target_page,
                        )));
                        instructions.push(instructions::Instructions::LDY(Instruction::absolute(
                            pos.cursor,
                        )))
                    }
                    None => instructions.push(instructions::Instructions::LDY(
                        Instruction::absolute(pos.cursor),
                    )),
                },
            };

            instructions
        }
        Types::AsKeyword(e) => {
            let mut instructions = resolve_type(
                assembler,
                &e.target,
                instructions::Registers::A,
                target_page,
                dependencies,
            );

            match &e.rtype {
                ellie_core::definite::definers::DefinerCollecting::Generic(e) => {
                    if e.rtype == "int" {
                        instructions.push(instructions::Instructions::A2I(Instruction::implicit()));
                    } else if e.rtype == "float" {
                        instructions.push(instructions::Instructions::A2F(Instruction::implicit()));
                    } else if e.rtype == "double" {
                        instructions.push(instructions::Instructions::A2D(Instruction::implicit()));
                    } else if e.rtype == "bool" {
                        instructions.push(instructions::Instructions::A2O(Instruction::implicit()));
                    } else if e.rtype == "string" {
                        instructions.push(instructions::Instructions::A2S(Instruction::implicit()));
                    } else if e.rtype == "char" {
                        instructions.push(instructions::Instructions::A2C(Instruction::implicit()));
                    } else if e.rtype == "byte" {
                        instructions.push(instructions::Instructions::A2B(Instruction::implicit()));
                    }
                }
                _ => panic!("As conv parent generic not implemented yet"),
            };

            instructions.push(instructions::Instructions::LDB(Instruction::indirect_a()));

            match target_register {
                instructions::Registers::A => {
                    instructions.push(instructions::Instructions::LDA(Instruction::indirect_b()))
                }
                instructions::Registers::B => (),
                instructions::Registers::C => {
                    instructions.push(instructions::Instructions::LDC(Instruction::indirect_b()))
                }
                instructions::Registers::X => {
                    instructions.push(instructions::Instructions::LDX(Instruction::indirect_b()))
                }
                instructions::Registers::Y => {
                    instructions.push(instructions::Instructions::LDY(Instruction::indirect_b()))
                }
            }
            instructions
        }
        Types::Byte(_) => {
            let converted_type = convert_type(types, dependencies);
            match target_register {
                instructions::Registers::A => {
                    vec![instructions::Instructions::LDA(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::B => {
                    vec![instructions::Instructions::LDB(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::C => {
                    vec![instructions::Instructions::LDC(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::X => {
                    vec![instructions::Instructions::LDX(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::Y => {
                    vec![instructions::Instructions::LDY(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
            }
        }
        Types::Integer(_) => {
            let converted_type = convert_type(types, dependencies);

            match target_register {
                instructions::Registers::A => {
                    vec![instructions::Instructions::LDA(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::B => {
                    vec![instructions::Instructions::LDB(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::C => {
                    vec![instructions::Instructions::LDC(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::X => {
                    vec![instructions::Instructions::LDX(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::Y => {
                    vec![instructions::Instructions::LDY(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
            }
        }
        Types::Float(_) => {
            let converted_type = convert_type(types, dependencies);

            match target_register {
                instructions::Registers::A => {
                    vec![instructions::Instructions::LDA(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::B => {
                    vec![instructions::Instructions::LDB(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::C => {
                    vec![instructions::Instructions::LDC(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::X => {
                    vec![instructions::Instructions::LDX(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::Y => {
                    vec![instructions::Instructions::LDY(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
            }
        }
        Types::Double(_) => {
            let converted_type = convert_type(types, dependencies);

            match target_register {
                instructions::Registers::A => {
                    vec![instructions::Instructions::LDA(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::B => {
                    vec![instructions::Instructions::LDB(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::C => {
                    vec![instructions::Instructions::LDC(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::X => {
                    vec![instructions::Instructions::LDX(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::Y => {
                    vec![instructions::Instructions::LDY(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
            }
        }
        Types::Bool(_) => {
            let converted_type = convert_type(types, dependencies);

            match target_register {
                instructions::Registers::A => {
                    vec![instructions::Instructions::LDA(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::B => {
                    vec![instructions::Instructions::LDB(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::C => {
                    vec![instructions::Instructions::LDC(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::X => {
                    vec![instructions::Instructions::LDX(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::Y => {
                    vec![instructions::Instructions::LDY(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
            }
        }
        Types::String(_) => {
            let converted_type = convert_type(types, dependencies);

            match target_register {
                instructions::Registers::A => {
                    vec![instructions::Instructions::LDA(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::B => {
                    vec![instructions::Instructions::LDB(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::C => {
                    vec![instructions::Instructions::LDC(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::X => {
                    vec![instructions::Instructions::LDX(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::Y => {
                    vec![instructions::Instructions::LDY(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
            }
        }
        Types::Char(_) => {
            let converted_type = convert_type(types, dependencies);

            match target_register {
                instructions::Registers::A => {
                    vec![instructions::Instructions::LDA(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::B => {
                    vec![instructions::Instructions::LDB(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::C => {
                    vec![instructions::Instructions::LDC(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::X => {
                    vec![instructions::Instructions::LDX(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
                instructions::Registers::Y => {
                    vec![instructions::Instructions::LDY(Instruction::immediate(
                        converted_type.0,
                        converted_type.1,
                    ))]
                }
            }
        }
        Types::Null => match target_register {
            instructions::Registers::A => {
                vec![instructions::Instructions::LDA(Instruction::immediate(
                    crate::instructions::Types::Null,
                    vec![],
                ))]
            }
            instructions::Registers::B => {
                vec![instructions::Instructions::LDB(Instruction::immediate(
                    crate::instructions::Types::Null,
                    vec![],
                ))]
            }
            instructions::Registers::C => {
                vec![instructions::Instructions::LDC(Instruction::immediate(
                    crate::instructions::Types::Null,
                    vec![],
                ))]
            }
            instructions::Registers::X => {
                vec![instructions::Instructions::LDX(Instruction::immediate(
                    crate::instructions::Types::Null,
                    vec![],
                ))]
            }
            instructions::Registers::Y => {
                vec![instructions::Instructions::LDY(Instruction::immediate(
                    crate::instructions::Types::Null,
                    vec![],
                ))]
            }
        },
        Types::Dynamic => todo!(),
        Types::SetterCall(_) => todo!(),
        Types::EnumData(_) => todo!(),
    }
}
*/
