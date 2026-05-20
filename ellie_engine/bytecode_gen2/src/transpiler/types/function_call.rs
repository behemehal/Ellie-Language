use ellie_core::{
    bytecode::RawType,
    definite::types::{class_instance::AttributeType, function_call::FunctionCall, variable::VariableType, Types},
    defs::DebugHeaderType,
};
use crate::{
    create_instruction,
    instructions::Instruction,
    opcode::OpCode,
    operand::{AddressingModes, Operand, Registers},
};
use super::TypeTranspiler;

impl TypeTranspiler for FunctionCall {
    fn transpile(&self, options: &mut super::TypeTranspilerOptions) {
        // For method calls (target is Reference with Method in index_chain),
        // push the receiver object first as the implicit `self` argument.
        let has_receiver = if let Types::Reference(ref_type) = &*self.target {
            ref_type.index_chain.iter().any(|a| a.rtype == AttributeType::Method)
        } else {
            false
        };

        if has_receiver {
            if let Types::Reference(ref_type) = &*self.target {
                ref_type.reference.transpile(options);
                options.assembler_mut().instructions.push(create_instruction!(
                    OpCode::Push,
                    Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None }
                ));
            }
        }

        // Push each user argument: evaluate into A, then Push A
        for param in &self.params {
            param.value.transpile(options);
            options.assembler_mut().instructions.push(create_instruction!(
                OpCode::Push,
                Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None }
            ));
        }

        let arg_count = self.params.len() + if has_receiver { 1 } else { 0 };

        // Resolve the call target
        let (target_hash, is_native) = resolve_target(&self.target, options);

        if let Some(hash) = target_hash {
            let raw: RawType = hash.into();
            let arg_count_raw: RawType = arg_count.into();
            if is_native {
                options.assembler_mut().instructions.push(create_instruction!(
                    OpCode::CallN,
                    Operand { mode: AddressingModes::Immediate, register: None, immediate: Some(raw) },
                    Operand { mode: AddressingModes::Immediate, register: None, immediate: Some(arg_count_raw) }
                ));
            } else {
                options.assembler_mut().instructions.push(create_instruction!(
                    OpCode::Call,
                    Operand { mode: AddressingModes::Immediate, register: None, immediate: Some(raw) },
                    Operand { mode: AddressingModes::Immediate, register: None, immediate: Some(arg_count_raw) }
                ));
            }

            // Caller cleanup: Sub SP, #arg_count (discard pushed args after return)
            if arg_count > 0 {
                let count_raw: RawType = arg_count.into();
                options.assembler_mut().instructions.push(create_instruction!(
                    OpCode::Sub,
                    Operand { mode: AddressingModes::Register, register: Some(Registers::SP), immediate: None },
                    Operand { mode: AddressingModes::Immediate, register: None, immediate: Some(count_raw) }
                ));
            }
        }
    }
}

fn resolve_target(
    target: &Types,
    options: &mut super::TypeTranspilerOptions,
) -> (Option<usize>, bool) {
    match target {
        Types::VariableType(var) => {
            let hash = var.reference;
            let is_native = options
                .assembler()
                .debug_headers
                .iter()
                .any(|h| h.hash == hash && h.rtype == DebugHeaderType::NativeFunction);
            (Some(hash), is_native)
        }
        Types::Reference(ref_type) => {
            // Method call like obj.method(...) — use method hash from index_chain if present
            if let Some(method_attr) = ref_type.index_chain.iter()
                .find(|a| a.rtype == AttributeType::Method)
            {
                let hash = method_attr.hash;
                let is_native = options.assembler().debug_headers.iter()
                    .any(|h| h.hash == hash && h.rtype == DebugHeaderType::NativeFunction);
                return (Some(hash), is_native);
            }
            resolve_target(&ref_type.reference, options)
        }
        _ => (None, false),
    }
}
