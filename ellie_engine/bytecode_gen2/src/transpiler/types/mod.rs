pub mod array;
pub mod bool;
pub mod byte;
pub mod char;
pub mod class_call;
pub mod cloak;
pub mod decimal;
pub mod function_call;
pub mod integer;
pub mod null;
pub mod operator;
pub mod string;
pub mod variable_ref;

use crate::{assembler::Assembler, create_instruction, instructions::Instruction, opcode::OpCode, operand::{AddressingModes, Operand, Registers}, types::Types as ByteCodeTypes, utils::{f32_to_le_bytes, f64_to_le_bytes, isize_to_le_bytes}};
use alloc::vec::Vec;
use ellie_core::{bytecode::RawType, definite::types::{class_instance::AttributeType, Types}, defs::PlatformArchitecture};

pub struct TypeTranspilerOptions<'a> {
    assembler: Option<&'a mut Assembler>,
    target_register: Registers,
    target_page: Option<usize>,
    dependencies: Option<Vec<usize>>,
}

impl<'a> Default for TypeTranspilerOptions<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> TypeTranspilerOptions<'a> {
    pub fn new() -> Self {
        Self {
            assembler: None,
            target_register: Registers::A,
            target_page: None,
            dependencies: None,
        }
    }

    pub fn assembler(&self) -> &Assembler {
        self.assembler.as_deref().expect("Assembler is not set")
    }

    pub fn assembler_mut(&mut self) -> &mut Assembler {
        self.assembler.as_deref_mut().expect("Assembler is not set")
    }

    pub fn target_register(&self) -> Registers {
        self.target_register
    }

    pub fn set_target_page(&mut self, target_page: usize) -> &mut Self {
        self.target_page = Some(target_page);
        self
    }

    pub fn set_dependencies(&mut self, dependencies: Vec<usize>) -> &mut Self {
        self.dependencies = Some(dependencies);
        self
    }

    pub fn set_target_register(&mut self, target_register: Registers) -> &mut Self {
        self.target_register = target_register;
        self
    }

    pub fn set_assembler(&mut self, assembler: &'a mut Assembler) -> &mut Self {
        self.assembler = Some(assembler);
        self
    }

    pub fn target_page(&self) -> usize {
        self.target_page.unwrap()
    }

    pub fn dependencies(&mut self) -> Option<Vec<usize>> {
        self.dependencies.clone()
    }
}

pub trait TypeTranspiler {
    fn transpile(&self, options: &mut TypeTranspilerOptions);
}

impl TypeTranspiler for Types {
    fn transpile(&self, options: &mut TypeTranspilerOptions) {
        match self {
            Types::Integer(i) => i.transpile(options),
            Types::Bool(b) => b.transpile(options),
            Types::Byte(b) => b.transpile(options),
            Types::Decimal(d) => d.transpile(options),
            Types::Char(c) => c.transpile(options),
            Types::String(s) => s.transpile(options),
            Types::Null => null::NullTranspiler.transpile(options),
            Types::Void => null::NullTranspiler.transpile(options),
            Types::Operator(op) => op.transpile(options),
            Types::VariableType(v) => v.transpile(options),
            Types::FunctionCall(fc) => fc.transpile(options),
            Types::FunctionParameter(fp) => {
                // Function parameters are locals with negative FP-relative offsets.
                let target_reg = options.target_register();
                let local = options.assembler().find_local_by_name(&fp.name).cloned();
                match local {
                    Some(local) => {
                        use crate::{
                            create_instruction,
                            instructions::Instruction,
                            opcode::OpCode,
                            operand::{AddressingModes, Operand, Registers},
                        };
                        use ellie_core::bytecode::{RawType, TypeId};
                        let offset: isize = local.cursor;
                        let mut data = [0_u8; 8];
                        data.copy_from_slice(&offset.to_le_bytes());
                        let raw = RawType {
                            type_id: TypeId::Int,
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
                        std::println!("Warning: parameter '{}' not found in locals", fp.name);
                    }
                }
            }
            Types::ConstructorParameter(cp) => {
                // Same as FunctionParameter — load from [FP + cursor] by name lookup
                let target_reg = options.target_register();
                let local = options.assembler().find_local_by_name(&cp.name).cloned();
                match local {
                    Some(local) => {
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
                            Operand { mode: AddressingModes::Register, register: Some(target_reg), immediate: None },
                            Operand { mode: AddressingModes::IndirectOffset, register: Some(Registers::FP), immediate: Some(raw) }
                        ));
                    }
                    None => {
                        std::println!("Warning: constructor parameter '{}' not found in locals", cp.name);
                    }
                }
            }
            Types::ClassCall(cc) => cc.transpile(options),
            Types::Array(arr) => arr.transpile(options),
            Types::Cloak(c) => c.transpile(options),
            Types::AsKeyword(ak) => ak.target.transpile(options),
            Types::Reference(ref_type) => {
                // self.field read: Reference with Property in index_chain
                if let Some(prop) = ref_type.index_chain.iter()
                    .find(|a| a.rtype == AttributeType::Property)
                {
                    let field_idx = options.assembler().get_field_idx_by_hash(prop.hash);
                    let self_local = options.assembler().find_local_by_name("self").cloned();
                    if let (Some(field_idx), Some(self_local)) = (field_idx, self_local) {
                        let target_reg = options.target_register();
                        // Load self (heap ref) into target register
                        let offset: isize = self_local.cursor;
                        let mut data = [0_u8; 8];
                        data.copy_from_slice(&offset.to_le_bytes());
                        let raw = RawType { type_id: ellie_core::bytecode::TypeId::Int, size: 8, data };
                        options.assembler_mut().instructions.push(create_instruction!(
                            OpCode::Mov,
                            Operand { mode: AddressingModes::Register, register: Some(target_reg), immediate: None },
                            Operand { mode: AddressingModes::IndirectOffset, register: Some(Registers::FP), immediate: Some(raw) }
                        ));
                        // Gfld target_reg, #field_idx → target_reg = heap[target_reg].fields[field_idx]
                        let idx_raw: RawType = field_idx.into();
                        options.assembler_mut().instructions.push(create_instruction!(
                            OpCode::Gfld,
                            Operand { mode: AddressingModes::Register, register: Some(target_reg), immediate: None },
                            Operand { mode: AddressingModes::Immediate, register: None, immediate: Some(idx_raw) }
                        ));
                    } else {
                        std::println!("Warning: could not resolve self.field read (field_idx={:?})", field_idx);
                    }
                } else {
                    std::println!("Skipping unimplemented reference type: {:?}", ref_type);
                }
            }
            Types::Negative(neg) => {
                // Transpile inner value then negate: Sub A, A; Sub A, inner  (0 - inner)
                // Simpler: inner → A, then Sub B, A where B=0... just use: Mov B, #0; Sub B, A; Mov A, B
                // For now: transpile inner, the VM will handle negation via the negative opcode path
                neg.value.transpile(options);
                // Emit Neg-style: 0 - A → using Sub with zero
                use crate::{create_instruction, instructions::Instruction, opcode::OpCode, operand::{AddressingModes, Operand, Registers}};
                use ellie_core::bytecode::RawType;
                let zero: RawType = 0usize.into();
                // Mov B, #0; Sub B, A; Mov A, B
                options.assembler_mut().instructions.push(create_instruction!(
                    OpCode::Mov,
                    Operand { mode: AddressingModes::Register, register: Some(Registers::B), immediate: None },
                    Operand { mode: AddressingModes::Immediate, register: None, immediate: Some(zero) }
                ));
                options.assembler_mut().instructions.push(create_instruction!(
                    OpCode::Sub,
                    Operand { mode: AddressingModes::Register, register: Some(Registers::B), immediate: None },
                    Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None }
                ));
                options.assembler_mut().instructions.push(create_instruction!(
                    OpCode::Mov,
                    Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None },
                    Operand { mode: AddressingModes::Register, register: Some(Registers::B), immediate: None }
                ));
            }
            _ => {
                panic!("Unimplemented type in gen2 transpiler: {:?}", self);
            }
        }
    }
}

pub fn convert_type(
    types: &Types,
    _page_hash: Option<Vec<usize>>,
    arch: PlatformArchitecture,
) -> (ByteCodeTypes, Vec<u8>) {
    match types {
        Types::Integer(integer) => (
            ByteCodeTypes::Integer,
            isize_to_le_bytes(integer.value, arch),
        ),
        _ => unreachable!("This type is not convertable to raw type"),
    }
}
