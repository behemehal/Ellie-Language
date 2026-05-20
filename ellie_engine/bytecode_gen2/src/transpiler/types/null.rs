use ellie_core::bytecode::{RawType, TypeId};
use crate::{create_instruction, instructions::Instruction, opcode::OpCode, operand::{AddressingModes, Operand}};
use super::TypeTranspiler;

pub struct NullTranspiler;

impl TypeTranspiler for NullTranspiler {
    fn transpile(&self, options: &mut super::TypeTranspilerOptions) {
        let raw = RawType { type_id: TypeId::Int, size: 0, data: [0_u8; 8] };
        let target_reg = options.target_register();
        options.assembler_mut().instructions.push(create_instruction!(
            OpCode::Mov,
            Operand { mode: AddressingModes::Register, register: Some(target_reg), immediate: None },
            Operand { mode: AddressingModes::Immediate, register: None, immediate: Some(raw) }
        ));
    }
}
