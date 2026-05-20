use alloc::vec::Vec;
use ellie_core::{
    definite::{
        items::condition::{Condition, ConditionType},
        types::{
            operator::{ComparisonOperators, Operators},
            Types,
        },
    },
};
use crate::{
    create_instruction,
    instructions::Instruction,
    opcode::OpCode,
    operand::{AddressingModes, Operand, Registers},
    transpiler::types::{TypeTranspiler, TypeTranspilerOptions},
};

impl super::Transpiler for Condition {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        let mut deps = alloc::vec![processed_page.hash];
        deps.extend(processed_page.dependencies.iter().map(|d| d.hash));

        // Collect PCs of end-jumps so we can patch them all to point past the condition.
        let mut end_jumps: Vec<usize> = Vec::new();

        for chain in &self.chains {
            if chain.rtype != ConditionType::Else {
                // Evaluate the condition; for comparison operators this emits Cmp B, A.
                {
                    let mut opts = TypeTranspilerOptions::new();
                    opts.set_assembler(assembler)
                        .set_dependencies(deps.clone())
                        .set_target_page(hash);
                    chain.condition.transpile(&mut opts);
                }

                // Emit a conditional jump that skips this block when the condition is false.
                let skip_opcode = inverse_jump_opcode(&chain.condition);
                let skip_pc = assembler.instructions.len();
                assembler.instructions.push(create_instruction!(
                    skip_opcode,
                    Operand {
                        mode: AddressingModes::Immediate,
                        register: None,
                        immediate: Some(0usize.into())
                    }
                ));

                // Assemble the condition body inline.
                let saved_depth = assembler.stack_depth;
                assembler.assemble_dependency(&chain.inner_page_id);
                let local_delta = assembler.stack_depth.saturating_sub(saved_depth);
                assembler.stack_depth = saved_depth;

                // Pop any locals declared inside the block (for the fall-through path).
                if local_delta > 0 {
                    assembler.instructions.push(create_instruction!(
                        OpCode::Sub,
                        Operand {
                            mode: AddressingModes::Register,
                            register: Some(Registers::SP),
                            immediate: None
                        },
                        Operand {
                            mode: AddressingModes::Immediate,
                            register: None,
                            immediate: Some(local_delta.into())
                        }
                    ));
                }

                // Emit end-jump to after the entire if/else chain (patched below).
                let end_jmp_pc = assembler.instructions.len();
                assembler.instructions.push(create_instruction!(
                    OpCode::Jmp,
                    Operand {
                        mode: AddressingModes::Immediate,
                        register: None,
                        immediate: Some(0usize.into())
                    }
                ));
                end_jumps.push(end_jmp_pc);

                // Patch the skip-jump: condition false → jump to just after this chain.
                let next_chain_start = assembler.instructions.len();
                patch_jump(assembler, skip_pc, next_chain_start);
            } else {
                // Else block: no condition check, just assemble the body.
                let saved_depth = assembler.stack_depth;
                assembler.assemble_dependency(&chain.inner_page_id);
                let local_delta = assembler.stack_depth.saturating_sub(saved_depth);
                assembler.stack_depth = saved_depth;

                if local_delta > 0 {
                    assembler.instructions.push(create_instruction!(
                        OpCode::Sub,
                        Operand {
                            mode: AddressingModes::Register,
                            register: Some(Registers::SP),
                            immediate: None
                        },
                        Operand {
                            mode: AddressingModes::Immediate,
                            register: None,
                            immediate: Some(local_delta.into())
                        }
                    ));
                }

                let end_jmp_pc = assembler.instructions.len();
                assembler.instructions.push(create_instruction!(
                    OpCode::Jmp,
                    Operand {
                        mode: AddressingModes::Immediate,
                        register: None,
                        immediate: Some(0usize.into())
                    }
                ));
                end_jumps.push(end_jmp_pc);
            }
        }

        // Patch all end-jumps to point to the first instruction after the condition.
        let end_pc = assembler.instructions.len();
        for jmp_pc in end_jumps {
            patch_jump(assembler, jmp_pc, end_pc);
        }

        true
    }
}

fn patch_jump(assembler: &mut crate::assembler::Assembler, pc: usize, target: usize) {
    if let Some(instr) = assembler.instructions.get_mut(pc) {
        if let Some(ref mut op) = instr.operand_0 {
            op.immediate = Some(target.into());
        }
    }
}

/// Returns the jump opcode that skips the then-block when the condition is FALSE.
/// The operator transpiler emits `Cmp B, A` where B=left, A=right, so flags are:
///   flag_lt = (left < right), flag_eq = (left == right), flag_gt = (left > right).
fn inverse_jump_opcode(condition: &Types) -> OpCode {
    if let Types::Operator(op) = condition {
        if let Operators::ComparisonType(cmp) = &op.operator {
            return match cmp {
                ComparisonOperators::Equal => OpCode::Jne,
                ComparisonOperators::NotEqual => OpCode::Je,
                ComparisonOperators::LessThan => OpCode::Jge,
                ComparisonOperators::LessThanOrEqual => OpCode::Jg,
                ComparisonOperators::GreaterThan => OpCode::Jle,
                ComparisonOperators::GreaterThanOrEqual => OpCode::Jl,
                ComparisonOperators::Null => OpCode::Je,
            };
        }
    }
    OpCode::Je
}
