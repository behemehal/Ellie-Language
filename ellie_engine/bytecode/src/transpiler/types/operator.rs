use ellie_core::definite::types::operator::{
    ArithmeticOperators, ComparisonOperators, LogicalOperators, OperatorType, Operators,
};

use super::TypeTranspiler;
use crate::{
    instruction_table::Instructions,
    instructions::{Instruction, Registers},
};

impl TypeTranspiler for OperatorType {
    fn transpile(&self, options: &mut super::TypeTranspilerOptions) {
        match &self.operator {
            Operators::ComparisonType(comparison_operator) => {
                self.first.transpile(
                    options
                        .copy()
                        .set_assembler(options.assembler_mut())
                        .set_target_register(Registers::B),
                );

                let first_operator_pos = options.assembler().location();

                options
                    .assembler_mut()
                    .instructions
                    .push(Instructions::STB(Instruction::implicit()));

                self.second.transpile(
                    options
                        .copy()
                        .set_assembler(options.assembler_mut())
                        .set_target_register(Registers::C),
                );

                let second_operator_pos = options.assembler().location();

                options
                    .assembler_mut()
                    .instructions
                    .push(Instructions::STC(Instruction::implicit()));

                options
                    .assembler_mut()
                    .instructions
                    .push(Instructions::LDB(Instruction::absolute(first_operator_pos)));

                options.assembler_mut().instructions.push(Instructions::LDC(
                    Instruction::absolute(second_operator_pos),
                ));

                options
                    .assembler_mut()
                    .instructions
                    .push(match comparison_operator {
                        ComparisonOperators::Equal => Instructions::EQ(Instruction::implicit()),
                        ComparisonOperators::NotEqual => Instructions::NE(Instruction::implicit()),
                        ComparisonOperators::GreaterThan => {
                            Instructions::GT(Instruction::implicit())
                        }
                        ComparisonOperators::LessThan => Instructions::LT(Instruction::implicit()),
                        ComparisonOperators::GreaterThanOrEqual => {
                            Instructions::GQ(Instruction::implicit())
                        }
                        ComparisonOperators::LessThanOrEqual => {
                            Instructions::LQ(Instruction::implicit())
                        }
                        ComparisonOperators::Null => unreachable!(),
                    });

                match options.target_register() {
                    Registers::A => (),
                    Registers::B => {
                        options
                            .assembler_mut()
                            .instructions
                            .push(Instructions::LDB(Instruction::indirect_a()));
                    }
                    Registers::C => {
                        options
                            .assembler_mut()
                            .instructions
                            .push(Instructions::LDC(Instruction::indirect_a()));
                    }
                    Registers::X => {
                        options
                            .assembler_mut()
                            .instructions
                            .push(Instructions::LDX(Instruction::indirect_a()));
                    }
                    Registers::Y => {
                        options
                            .assembler_mut()
                            .instructions
                            .push(Instructions::LDY(Instruction::indirect_a()));
                    }
                }
            }
            Operators::LogicalType(logical_operators) => {
                self.second.transpile(
                    options
                        .copy()
                        .set_assembler(options.assembler_mut())
                        .set_target_register(Registers::C),
                );

                let second_operator_pos = options.assembler().location();
                options
                    .assembler_mut()
                    .instructions
                    .push(Instructions::STC(Instruction::implicit()));

                self.first.transpile(
                    options
                        .copy()
                        .set_assembler(options.assembler_mut())
                        .set_target_register(Registers::B),
                );

                options.assembler_mut().instructions.push(Instructions::LDC(
                    Instruction::absolute(second_operator_pos),
                ));

                options
                    .assembler_mut()
                    .instructions
                    .push(match logical_operators {
                        LogicalOperators::And => Instructions::AND(Instruction::implicit()),
                        LogicalOperators::Or => Instructions::OR(Instruction::implicit()),
                        _ => unreachable!(),
                    });

                match options.target_register() {
                    Registers::A => (),
                    Registers::B => {
                        options
                            .assembler_mut()
                            .instructions
                            .push(Instructions::LDB(Instruction::indirect_a()));
                    }
                    Registers::C => {
                        options
                            .assembler_mut()
                            .instructions
                            .push(Instructions::LDC(Instruction::indirect_a()));
                    }
                    Registers::X => {
                        options
                            .assembler_mut()
                            .instructions
                            .push(Instructions::LDX(Instruction::indirect_a()));
                    }
                    Registers::Y => {
                        options
                            .assembler_mut()
                            .instructions
                            .push(Instructions::LDY(Instruction::indirect_a()));
                    }
                }
            }
            Operators::ArithmeticType(arithmetic_operators) => {
                self.first.transpile(
                    options
                        .copy()
                        .set_assembler(options.assembler_mut())
                        .set_target_register(Registers::B),
                );

                let first_operator_pos = options.assembler().instructions.len();
                options
                    .assembler_mut()
                    .instructions
                    .push(Instructions::STB(Instruction::implicit()));

                self.second.transpile(
                    options
                        .copy()
                        .set_assembler(options.assembler_mut())
                        .set_target_register(Registers::C),
                );

                let second_operator_pos = options.assembler().instructions.len();

                options
                    .assembler_mut()
                    .instructions
                    .push(Instructions::STC(Instruction::implicit()));

                options
                    .assembler_mut()
                    .instructions
                    .push(Instructions::LDB(Instruction::absolute(first_operator_pos)));
                options.assembler_mut().instructions.push(Instructions::LDC(
                    Instruction::absolute(second_operator_pos),
                ));

                options
                    .assembler_mut()
                    .instructions
                    .push(match arithmetic_operators {
                        ArithmeticOperators::Addition => Instructions::ADD(Instruction::implicit()),
                        ArithmeticOperators::Subtraction => {
                            Instructions::SUB(Instruction::implicit())
                        }
                        ArithmeticOperators::Multiplication => {
                            Instructions::MUL(Instruction::implicit())
                        }
                        ArithmeticOperators::Exponentiation => {
                            Instructions::EXP(Instruction::implicit())
                        }
                        ArithmeticOperators::Division => Instructions::DIV(Instruction::implicit()),
                        ArithmeticOperators::Modulus => Instructions::MOD(Instruction::implicit()),
                        ArithmeticOperators::Null => unreachable!("Wrong operator"),
                    });
                match options.target_register() {
                    Registers::A => (),
                    Registers::B => {
                        options
                            .assembler_mut()
                            .instructions
                            .push(Instructions::LDB(Instruction::indirect_a()));
                    }
                    Registers::C => {
                        options
                            .assembler_mut()
                            .instructions
                            .push(Instructions::LDC(Instruction::indirect_a()));
                    }
                    Registers::X => {
                        options
                            .assembler_mut()
                            .instructions
                            .push(Instructions::LDX(Instruction::indirect_a()));
                    }
                    Registers::Y => {
                        options
                            .assembler_mut()
                            .instructions
                            .push(Instructions::LDY(Instruction::indirect_a()));
                    }
                }
            }
            Operators::AssignmentType(_) => todo!(),
            Operators::Null => unreachable!(),
        }
    }
}
