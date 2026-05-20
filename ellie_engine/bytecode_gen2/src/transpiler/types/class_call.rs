use ellie_core::{
    bytecode::RawType,
    definite::types::{class_call::ClassCall, Types},
};
use crate::{
    create_instruction,
    instructions::Instruction,
    opcode::OpCode,
    operand::{AddressingModes, Operand, Registers},
};
use super::TypeTranspiler;

impl TypeTranspiler for ClassCall {
    fn transpile(&self, options: &mut super::TypeTranspilerOptions) {
        // Resolve the class hash from the target (e.g. Types::VariableType pointing to Library)
        let class_hash = match *self.target.clone() {
            Types::VariableType(v) => v.reference,
            _ => {
                std::println!("ClassCall: unexpected target type {:?}", self.target);
                return;
            }
        };

        // Find the class field map to know how many fields to allocate
        let n_fields = options.assembler()
            .class_field_maps
            .iter()
            .find(|m| m.class_hash == class_hash)
            .map(|m| m.n_fields)
            .unwrap_or(0);

        // Emit Co #n_fields → A = heap ref for new class instance
        let n_fields_raw: RawType = n_fields.into();
        options.assembler_mut().instructions.push(create_instruction!(
            OpCode::Co,
            Operand { mode: AddressingModes::Immediate, register: None, immediate: Some(n_fields_raw) }
        ));

        // Save the new instance ref in X (callee-saved scratch register)
        options.assembler_mut().instructions.push(create_instruction!(
            OpCode::Mov,
            Operand { mode: AddressingModes::Register, register: Some(Registers::X), immediate: None },
            Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None }
        ));

        // Push self (the heap ref) as implicit first argument to constructor
        options.assembler_mut().instructions.push(create_instruction!(
            OpCode::Push,
            Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None }
        ));

        // Push constructor parameters
        for param in &self.params {
            param.value.transpile(options);
            options.assembler_mut().instructions.push(create_instruction!(
                OpCode::Push,
                Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None }
            ));
        }

        let total_args = 1 + self.params.len();

        // Call constructor (looked up by class_hash in fn_table)
        let hash_raw: RawType = class_hash.into();
        let args_raw: RawType = total_args.into();
        options.assembler_mut().instructions.push(create_instruction!(
            OpCode::Call,
            Operand { mode: AddressingModes::Immediate, register: None, immediate: Some(hash_raw) },
            Operand { mode: AddressingModes::Immediate, register: None, immediate: Some(args_raw) }
        ));

        // Caller cleanup: Sub SP, #total_args
        options.assembler_mut().instructions.push(create_instruction!(
            OpCode::Sub,
            Operand { mode: AddressingModes::Register, register: Some(Registers::SP), immediate: None },
            Operand { mode: AddressingModes::Immediate, register: None, immediate: Some(args_raw) }
        ));

        // Restore instance ref from X to target register
        let target_reg = options.target_register();
        options.assembler_mut().instructions.push(create_instruction!(
            OpCode::Mov,
            Operand { mode: AddressingModes::Register, register: Some(target_reg), immediate: None },
            Operand { mode: AddressingModes::Register, register: Some(Registers::X), immediate: None }
        ));
    }
}
