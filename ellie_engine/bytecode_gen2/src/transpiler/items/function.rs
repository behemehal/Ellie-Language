use crate::{
    assembler::MainFunction,
    create_instruction,
    instructions::Instruction,
    opcode::OpCode,
    operand::{AddressingModes, Operand, Registers},
    utils::limit_platform_size,
};
use ellie_core::defs::{DebugHeader, DebugHeaderType};

impl super::Transpiler for ellie_core::definite::items::function::Function {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        _hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        // Emit function marker so the VM can build a hash→PC lookup table
        let fn_hash_raw: ellie_core::bytecode::RawType = self.hash.into();
        assembler.instructions.push(create_instruction!(
            OpCode::Fn,
            Operand {
                mode: AddressingModes::Immediate,
                register: None,
                immediate: Some(fn_hash_raw),
            }
        ));
        let fn_start = assembler.instructions.len() - 1;

        // Save caller's stack depth, start fresh for this function scope
        let saved_stack_depth = assembler.stack_depth;
        assembler.stack_depth = 0;

        // Prologue: save old FP, set new FP = SP
        assembler.instructions.push(create_instruction!(
            OpCode::Push,
            Operand {
                mode: AddressingModes::Register,
                register: Some(Registers::FP),
                immediate: None
            }
        ));
        assembler.instructions.push(create_instruction!(
            OpCode::Mov,
            Operand {
                mode: AddressingModes::Register,
                register: Some(Registers::FP),
                immediate: None
            },
            Operand {
                mode: AddressingModes::Register,
                register: Some(Registers::SP),
                immediate: None
            }
        ));

        // Register parameters as locals with negative FP offsets.
        // Caller pushes params left-to-right; after `Push FP; Mov FP, SP`:
        //   param[0] is at stack[FP - num_params - 1]
        //   param[i] is at stack[FP - (num_params - i) - 1]
        // Parameter hashes come from the inner page's FunctionParameter items.
        let inner_params: alloc::vec::Vec<_> = assembler
            .module
            .pages
            .iter()
            .find(|p| p.hash == self.inner_page_id)
            .map(|p| {
                p.items
                    .iter()
                    .filter_map(|item| {
                        if let ellie_core::definite::items::Collecting::FunctionParameter(fp) = item {
                            Some(fp.clone())
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        let num_params = inner_params.len() as isize;
        for (i, fp) in inner_params.iter().enumerate() {
            let cursor = -(num_params - i as isize) - 1;
            assembler.add_local(crate::assembler::LocalHeader {
                name: fp.name.clone(),
                cursor,
                reference: None,
                hash: Some(fp.hash),
                page_hash: processed_page.hash,
                borrowed: None,
            });
        }

        // Assemble the function body (inner page)
        assembler.assemble_dependency(&self.inner_page_id);

        // Epilogue: discard locals, restore FP, return
        assembler.instructions.push(create_instruction!(
            OpCode::Mov,
            Operand {
                mode: AddressingModes::Register,
                register: Some(Registers::SP),
                immediate: None
            },
            Operand {
                mode: AddressingModes::Register,
                register: Some(Registers::FP),
                immediate: None
            }
        ));
        assembler.instructions.push(create_instruction!(
            OpCode::Pop,
            Operand {
                mode: AddressingModes::Register,
                register: Some(Registers::FP),
                immediate: None
            }
        ));
        assembler.instructions.push(create_instruction!(OpCode::Ret));

        let fn_end = assembler.instructions.len() - 1;

        assembler.debug_headers.push(DebugHeader {
            rtype: DebugHeaderType::Function,
            hash: limit_platform_size(self.hash, assembler.platform_attributes.architecture),
            start_end: (fn_start, fn_end),
            module_name: processed_page.path.clone(),
            module_hash: processed_page.hash,
            name: self.name.clone(),
            pos: self.pos,
        });

        if self.name == "main" {
            assembler.main_function = Some(MainFunction {
                hash: self.hash,
                start: fn_start,
                end: fn_end,
            });
        }

        // Restore caller's stack depth
        assembler.stack_depth = saved_stack_depth;

        true
    }
}
