use ellie_core::{bytecode::RawType, definite::types::array::ArrayType};
use crate::{
    create_instruction,
    instructions::Instruction,
    opcode::OpCode,
    operand::{AddressingModes, Operand, Registers},
};
use super::TypeTranspiler;

impl TypeTranspiler for ArrayType {
    fn transpile(&self, options: &mut super::TypeTranspilerOptions) {
        let n = self.collective.len();

        // Allocate ClassInst with n+1 slots: [length, elem0, elem1, ...]
        let n_plus_1: RawType = (n + 1).into();
        options.assembler_mut().instructions.push(create_instruction!(
            OpCode::Co,
            Operand { mode: AddressingModes::Immediate, register: None, immediate: Some(n_plus_1) }
        ));

        // Preserve array ref across element evaluations
        options.assembler_mut().instructions.push(create_instruction!(
            OpCode::Push,
            Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None }
        ));

        // Store length in field 0
        let len_raw: RawType = n.into();
        options.assembler_mut().instructions.push(create_instruction!(
            OpCode::Mov,
            Operand { mode: AddressingModes::Register, register: Some(Registers::B), immediate: None },
            Operand { mode: AddressingModes::Immediate, register: None, immediate: Some(len_raw) }
        ));
        let neg1: RawType = make_neg1();
        options.assembler_mut().instructions.push(create_instruction!(
            OpCode::Mov,
            Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None },
            Operand { mode: AddressingModes::IndirectOffset, register: Some(Registers::SP), immediate: Some(neg1) }
        ));
        let zero_idx: RawType = 0usize.into();
        options.assembler_mut().instructions.push(create_instruction!(
            OpCode::Sfld,
            Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None },
            Operand { mode: AddressingModes::Immediate, register: None, immediate: Some(zero_idx) }
        ));

        for (i, entry) in self.collective.iter().enumerate() {
            entry.value.transpile(options);

            let neg1: RawType = make_neg1();
            let field_idx: RawType = (i + 1).into();
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
                Operand { mode: AddressingModes::Immediate, register: None, immediate: Some(field_idx) }
            ));
        }

        let target_reg = options.target_register();
        options.assembler_mut().instructions.push(create_instruction!(
            OpCode::Pop,
            Operand { mode: AddressingModes::Register, register: Some(target_reg), immediate: None }
        ));
    }
}

fn make_neg1() -> RawType {
    let offset: isize = -1;
    let mut d = [0u8; 8];
    d.copy_from_slice(&offset.to_le_bytes());
    RawType { type_id: ellie_core::bytecode::TypeId::Int, size: 8, data: d }
}
