use ellie_core::definite::types::cloak::CloakType;
use crate::{
    create_instruction,
    instructions::Instruction,
    opcode::OpCode,
    operand::{AddressingModes, Operand, Registers},
};
use ellie_core::bytecode::RawType;
use super::TypeTranspiler;

impl TypeTranspiler for CloakType {
    fn transpile(&self, options: &mut super::TypeTranspilerOptions) {
        let n = self.collective.len();

        // Allocate a ClassInst with n fields
        let n_raw: RawType = n.into();
        options.assembler_mut().instructions.push(create_instruction!(
            OpCode::Co,
            Operand { mode: AddressingModes::Immediate, register: None, immediate: Some(n_raw) }
        ));

        // Push cloak_ref onto data stack to preserve it across field evaluations
        options.assembler_mut().instructions.push(create_instruction!(
            OpCode::Push,
            Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None }
        ));

        for (i, entry) in self.collective.iter().enumerate() {
            // Evaluate field value → A
            entry.value.transpile(options);

            // B = value; A = cloak_ref (peek stack top); Sfld
            let idx_raw: RawType = i.into();
            let neg1: RawType = {
                let offset: isize = -1;
                let mut d = [0u8; 8];
                d.copy_from_slice(&offset.to_le_bytes());
                RawType { type_id: ellie_core::bytecode::TypeId::Int, size: 8, data: d }
            };
            options.assembler_mut().instructions.push(create_instruction!(
                OpCode::Mov,
                Operand { mode: AddressingModes::Register, register: Some(Registers::B), immediate: None },
                Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None }
            ));
            options.assembler_mut().instructions.push(create_instruction!(
                OpCode::Mov,
                Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None },
                Operand { mode: AddressingModes::IndirectOffset, register: Some(Registers::SP), immediate: Some(neg1) }
            ));
            options.assembler_mut().instructions.push(create_instruction!(
                OpCode::Sfld,
                Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None },
                Operand { mode: AddressingModes::Immediate, register: None, immediate: Some(idx_raw) }
            ));
        }

        // Pop cloak_ref into target register
        let target_reg = options.target_register();
        options.assembler_mut().instructions.push(create_instruction!(
            OpCode::Pop,
            Operand { mode: AddressingModes::Register, register: Some(target_reg), immediate: None }
        ));
    }
}
