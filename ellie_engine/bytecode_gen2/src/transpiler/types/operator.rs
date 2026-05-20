use ellie_core::definite::types::operator::{
    ArithmeticOperators, ComparisonOperators, OperatorType, Operators,
};
use crate::{
    create_instruction,
    instructions::Instruction,
    opcode::OpCode,
    operand::{AddressingModes, Operand, Registers},
};
use super::TypeTranspiler;

impl TypeTranspiler for OperatorType {
    fn transpile(&self, options: &mut super::TypeTranspilerOptions) {
        // Evaluate left → A, save on stack
        self.first.transpile(options);
        options.assembler_mut().instructions.push(create_instruction!(
            OpCode::Push,
            Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None }
        ));

        // Evaluate right → A
        self.second.transpile(options);

        // Pop left into B  (B = left, A = right)
        options.assembler_mut().instructions.push(create_instruction!(
            OpCode::Pop,
            Operand { mode: AddressingModes::Register, register: Some(Registers::B), immediate: None }
        ));

        match &self.operator {
            Operators::ArithmeticType(arith) => {
                match arith {
                    // Commutative: A = A op B (result in A)
                    ArithmeticOperators::Addition => {
                        options.assembler_mut().instructions.push(create_instruction!(
                            OpCode::Add,
                            Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None },
                            Operand { mode: AddressingModes::Register, register: Some(Registers::B), immediate: None }
                        ));
                    }
                    ArithmeticOperators::Multiplication => {
                        options.assembler_mut().instructions.push(create_instruction!(
                            OpCode::Mul,
                            Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None },
                            Operand { mode: AddressingModes::Register, register: Some(Registers::B), immediate: None }
                        ));
                    }
                    // Non-commutative: B op A = left op right; emit OP B, A then Mov A, B
                    ArithmeticOperators::Subtraction => {
                        options.assembler_mut().instructions.push(create_instruction!(
                            OpCode::Sub,
                            Operand { mode: AddressingModes::Register, register: Some(Registers::B), immediate: None },
                            Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None }
                        ));
                        options.assembler_mut().instructions.push(create_instruction!(
                            OpCode::Mov,
                            Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None },
                            Operand { mode: AddressingModes::Register, register: Some(Registers::B), immediate: None }
                        ));
                    }
                    ArithmeticOperators::Division => {
                        options.assembler_mut().instructions.push(create_instruction!(
                            OpCode::Div,
                            Operand { mode: AddressingModes::Register, register: Some(Registers::B), immediate: None },
                            Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None }
                        ));
                        options.assembler_mut().instructions.push(create_instruction!(
                            OpCode::Mov,
                            Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None },
                            Operand { mode: AddressingModes::Register, register: Some(Registers::B), immediate: None }
                        ));
                    }
                    ArithmeticOperators::Modulus => {
                        options.assembler_mut().instructions.push(create_instruction!(
                            OpCode::Mod,
                            Operand { mode: AddressingModes::Register, register: Some(Registers::B), immediate: None },
                            Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None }
                        ));
                        options.assembler_mut().instructions.push(create_instruction!(
                            OpCode::Mov,
                            Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None },
                            Operand { mode: AddressingModes::Register, register: Some(Registers::B), immediate: None }
                        ));
                    }
                    ArithmeticOperators::Exponentiation => {
                        options.assembler_mut().instructions.push(create_instruction!(
                            OpCode::Exp,
                            Operand { mode: AddressingModes::Register, register: Some(Registers::B), immediate: None },
                            Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None }
                        ));
                        options.assembler_mut().instructions.push(create_instruction!(
                            OpCode::Mov,
                            Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None },
                            Operand { mode: AddressingModes::Register, register: Some(Registers::B), immediate: None }
                        ));
                    }
                    ArithmeticOperators::Null => {}
                }
            }
            Operators::ComparisonType(_cmp) => {
                // Cmp B, A sets flags; Je/Jne use them for conditional jumps
                // Result as bool value will be handled when conditions are implemented
                options.assembler_mut().instructions.push(create_instruction!(
                    OpCode::Cmp,
                    Operand { mode: AddressingModes::Register, register: Some(Registers::B), immediate: None },
                    Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None }
                ));
            }
            Operators::LogicalType(_) => {
                // TODO: logical AND/OR
            }
            Operators::AssignmentType(_) => {
                // TODO: assignment operators
            }
            Operators::Null => {}
        }
    }
}
