use ellie_core::{bytecode::{RawType, TypeId}, definite::types::byte::ByteType};
use crate::{create_instruction, instructions::Instruction, opcode::OpCode, operand::{AddressingModes, Operand}};
use super::TypeTranspiler;

impl TypeTranspiler for ByteType {
    fn transpile(&self, options: &mut super::TypeTranspilerOptions) {
        let mut data = [0_u8; 8];
        data[0] = self.value as u8;
        let raw = RawType { type_id: TypeId::Byte, size: 1, data };
        options.assembler_mut().instructions.push(create_instruction!(
            OpCode::Mov,
            Operand { mode: AddressingModes::Immediate, register: None, immediate: Some(raw) }
        ));
    }
}
