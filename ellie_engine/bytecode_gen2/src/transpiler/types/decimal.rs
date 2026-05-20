use ellie_core::{bytecode::{RawType, TypeId}, definite::types::decimal::{DecimalType, DecimalTypeEnum}};
use crate::{create_instruction, instructions::Instruction, opcode::OpCode, operand::{AddressingModes, Operand}};
use super::TypeTranspiler;

impl TypeTranspiler for DecimalType {
    fn transpile(&self, options: &mut super::TypeTranspilerOptions) {
        let raw = match self.value {
            DecimalTypeEnum::Float(f) => {
                let mut data = [0_u8; 8];
                data[..4].copy_from_slice(&f.to_le_bytes());
                RawType { type_id: TypeId::Float, size: 4, data }
            }
            DecimalTypeEnum::Double(d) => {
                let mut data = [0_u8; 8];
                data.copy_from_slice(&d.to_le_bytes());
                RawType { type_id: TypeId::Double, size: 8, data }
            }
        };
        options.assembler_mut().instructions.push(create_instruction!(
            OpCode::Mov,
            Operand { mode: AddressingModes::Immediate, register: None, immediate: Some(raw) }
        ));
    }
}
