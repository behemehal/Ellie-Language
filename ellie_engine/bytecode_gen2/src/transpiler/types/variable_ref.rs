use ellie_core::{bytecode::RawType, definite::types::variable::VariableType};
use crate::{
    create_instruction,
    instructions::Instruction,
    opcode::OpCode,
    operand::{AddressingModes, Operand, Registers},
};
use super::TypeTranspiler;

impl TypeTranspiler for VariableType {
    fn transpile(&self, options: &mut super::TypeTranspilerOptions) {
        let target_reg = options.target_register();

        // Find by hash first; fall back to name when hash=0 (e.g., bde-generated refs)
        let local = options.assembler().find_local_by_hash(self.reference).cloned()
            .or_else(|| if self.reference == 0 {
                options.assembler().find_local_by_name(&self.value).cloned()
            } else {
                None
            });

        match local {
            Some(local) => {
                // Load from [FP + cursor] where cursor is the signed FP-relative stack slot index
                let offset: isize = local.cursor;
                let mut data = [0_u8; 8];
                data.copy_from_slice(&offset.to_le_bytes());
                let raw = RawType {
                    type_id: ellie_core::bytecode::TypeId::Int,
                    size: core::mem::size_of::<isize>(),
                    data,
                };
                options.assembler_mut().instructions.push(create_instruction!(
                    OpCode::Mov,
                    Operand {
                        mode: AddressingModes::Register,
                        register: Some(target_reg),
                        immediate: None
                    },
                    Operand {
                        mode: AddressingModes::IndirectOffset,
                        register: Some(Registers::FP),
                        immediate: Some(raw)
                    }
                ));
            }
            None => {
                // Hash not found as local — could be a function reference, handled by call site
                std::println!(
                    "Warning: local not found for variable '{}' (hash: {})",
                    self.value, self.reference
                );
            }
        }
    }
}
