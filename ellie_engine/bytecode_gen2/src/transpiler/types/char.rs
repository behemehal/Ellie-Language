use ellie_core::{bytecode::{RawType, TypeId}, definite::types::ellie_char::CharType};
use crate::{create_instruction, instructions::Instruction, opcode::OpCode, operand::{AddressingModes, Operand}};
use super::TypeTranspiler;

impl TypeTranspiler for CharType {
    fn transpile(&self, options: &mut super::TypeTranspilerOptions) {
        let mut data = [0_u8; 8];
        data[..4].copy_from_slice(&(self.value as u32).to_le_bytes());
        let raw = RawType { type_id: TypeId::Char, size: 4, data };
        options.assembler_mut().instructions.push(create_instruction!(
            OpCode::Mov,
            Operand { mode: AddressingModes::Immediate, register: None, immediate: Some(raw) }
        ));
    }
}
