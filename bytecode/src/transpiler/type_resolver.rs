use alloc::vec::Vec;
use alloc::{boxed::Box, vec};
use ellie_core::definite::types::{operator, Types};

use crate::{
    assembler::Assembler,
    instructions::{self, Instruction, Instructions},
};

pub fn convert_type(types: &Types) -> (instructions::Types, Vec<u8>) {
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
            instructions::Types::String((string.value.len(), 1)), //UTF-16, UTF-32 is todo
            string.value.as_bytes().to_vec(),
        ),
        Types::Char(char) => (instructions::Types::Char(8), vec![]),
        Types::Collective(_) => todo!(),
        Types::Reference(_) => todo!(),
        Types::BraceReference(_) => todo!(),
        Types::Operator(_) => todo!(),
        Types::Cloak(_) => todo!(),
        Types::Array(array) => {
            let mut inner_type = instructions::Types::Null;

            let mut bytes = Vec::new();
            for collective in &array.collective {
                let converted = convert_type(&collective.value);
                if bytes.len() == 0 {
                    inner_type = converted.0;
                }
                bytes.extend(converted.1);
            }
            todo!()
            //instructions::Types::Array(array.collective.len())
        }
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
    }
}

pub fn resolve_type(
    assembler: &mut Assembler,
    types: &Types,
    target_register: instructions::Registers,
    target_page: &usize,
) -> Vec<Instructions> {
    match types {
        Types::Collective(_) => todo!(),
        Types::Reference(_) => todo!(),
        Types::BraceReference(_) => todo!(),
        Types::Operator(operator) => match &operator.operator {
            operator::Operators::ComparisonType(e) => {
                let mut instructions = Vec::new();
                instructions.extend(resolve_type(
                    assembler,
                    &operator.first,
                    instructions::Registers::B,
                    target_page,
                ));

                instructions.extend(resolve_type(
                    assembler,
                    &operator.second,
                    instructions::Registers::C,
                    target_page,
                ));

                instructions.push(match e {
                    operator::ComparisonOperators::Equal => {
                        instructions::Instructions::EQ(Instruction::implict())
                    }
                    operator::ComparisonOperators::NotEqual => {
                        instructions::Instructions::NE(Instruction::implict())
                    }
                    operator::ComparisonOperators::GreaterThan => {
                        instructions::Instructions::GT(Instruction::implict())
                    }
                    operator::ComparisonOperators::LessThan => {
                        instructions::Instructions::LT(Instruction::implict())
                    }
                    operator::ComparisonOperators::GreaterThanOrEqual => {
                        instructions::Instructions::GQ(Instruction::implict())
                    }
                    operator::ComparisonOperators::LessThanOrEqual => {
                        instructions::Instructions::LQ(Instruction::implict())
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
                    &operator.first,
                    instructions::Registers::B,
                    target_page,
                ));

                instructions.extend(resolve_type(
                    assembler,
                    &operator.second,
                    instructions::Registers::C,
                    target_page,
                ));

                instructions.push(match e {
                    operator::ArithmeticOperators::Addition => {
                        instructions::Instructions::ADD(Instruction::implict())
                    }
                    operator::ArithmeticOperators::Subtraction => {
                        instructions::Instructions::SUB(Instruction::implict())
                    }
                    operator::ArithmeticOperators::Multiplication => {
                        instructions::Instructions::MUL(Instruction::implict())
                    }
                    operator::ArithmeticOperators::Exponentiation => {
                        instructions::Instructions::EXP(Instruction::implict())
                    }
                    operator::ArithmeticOperators::Division => {
                        instructions::Instructions::DIV(Instruction::implict())
                    }
                    operator::ArithmeticOperators::Modulus => {
                        instructions::Instructions::MOD(Instruction::implict())
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
            let converted_type = convert_type(types);

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
        Types::FunctionCall(e) => {
            let mut instructions = Vec::new();
            instructions.extend(resolve_type(
                assembler,
                &e.target,
                instructions::Registers::B,
                target_page,
            ));
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
            let pos = assembler.find_local(&e.value).unwrap();

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
        Types::AsKeyword(_) => todo!(),
        Types::Byte(_) => {
            let converted_type = convert_type(types);
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
            let converted_type = convert_type(types);

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
            let converted_type = convert_type(types);

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
            let converted_type = convert_type(types);

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
            let converted_type = convert_type(types);

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
            let converted_type = convert_type(types);

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
            let converted_type = convert_type(types);

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
    }
}
