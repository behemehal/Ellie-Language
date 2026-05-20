use ellie_core::{bytecode::{RawType, TypeId}, definite::types::string::StringType};
use crate::{
    create_instruction,
    instructions::Instruction,
    opcode::OpCode,
    operand::{AddressingModes, Operand, Registers},
};
use super::TypeTranspiler;

impl TypeTranspiler for StringType {
    fn transpile(&self, options: &mut super::TypeTranspilerOptions) {
        // Str A — allocate empty string, reference in A
        options.assembler_mut().instructions.push(create_instruction!(
            OpCode::Str,
            Operand {
                mode: AddressingModes::Register,
                register: Some(Registers::A),
                immediate: None
            }
        ));

        // Spus A, #char(c) — append each character
        for ch in self.value.chars() {
            let mut data = [0_u8; 8];
            data[..4].copy_from_slice(&(ch as u32).to_le_bytes());
            let raw = RawType { type_id: TypeId::Char, size: 4, data };
            options.assembler_mut().instructions.push(create_instruction!(
                OpCode::Spus,
                Operand {
                    mode: AddressingModes::Register,
                    register: Some(Registers::A),
                    immediate: None
                },
                Operand {
                    mode: AddressingModes::Immediate,
                    register: None,
                    immediate: Some(raw)
                }
            ));
        }
    }
}
