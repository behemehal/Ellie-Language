use alloc::{string::ToString, vec, vec::Vec};
use ellie_core::{
    definite::items::{class, Collecting},
    defs::{DebugHeader, DebugHeaderType},
};

use crate::{
    assembler::{ClassFieldEntry, ClassFieldMap, LocalHeader, MainFunction},
    create_instruction,
    instructions::Instruction,
    opcode::OpCode,
    operand::{AddressingModes, Operand, Registers},
    transpiler::{
        items::Transpiler,
        types::{TypeTranspiler, TypeTranspilerOptions},
    },
    utils::limit_platform_size,
};
use ellie_core::bytecode::RawType;

impl super::Transpiler for class::Class {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        _hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        // Register class itself as a local so ClassCall can find it by hash
        assembler.add_local(LocalHeader {
            name: self.name.clone(),
            cursor: 0,
            reference: None,
            hash: Some(self.hash),
            page_hash: processed_page.hash,
            borrowed: None,
        });

        // Find the class inner page
        let inner_page = match assembler.module.pages.iter().find(|p| p.hash == self.inner_page_id).cloned() {
            Some(p) => p,
            None => return true,
        };

        // Mark inner page as processed to prevent re-processing
        assembler.processed.push(self.inner_page_id);

        // Build class field map from Variable items in the inner page
        let mut fields: Vec<ClassFieldEntry> = Vec::new();
        for item in &inner_page.items {
            if let Collecting::Variable(var) = item {
                fields.push(ClassFieldEntry {
                    name: var.name.clone(),
                    hash: var.hash,
                    index: fields.len(),
                });
            }
        }
        let n_fields = fields.len();
        assembler.class_field_maps.push(ClassFieldMap {
            class_hash: self.hash,
            n_fields,
            fields,
        });

        // Process inner page dependencies
        for dep in &inner_page.dependencies {
            assembler.assemble_dependency(&dep.hash);
        }

        // Compile Constructor and Function items
        for item in &inner_page.items.clone() {
            match item {
                Collecting::Function(function) => {
                    compile_method(assembler, function, &inner_page, self.hash);
                }
                Collecting::Constructor(constructor) => {
                    compile_constructor(assembler, constructor, &inner_page, self.hash, n_fields);
                }
                // Field declarations, self declaration, params — handled above or not needed
                _ => {}
            }
        }

        true
    }
}

fn compile_constructor(
    assembler: &mut crate::assembler::Assembler,
    constructor: &ellie_core::definite::items::constructor::Constructor,
    class_page: &ellie_parser::parser::ProcessedPage,
    class_hash: usize,
    n_fields: usize,
) {
    // Emit Fn #class_hash marker (constructor is looked up by class_hash)
    let fn_hash_raw: RawType = class_hash.into();
    assembler.instructions.push(create_instruction!(
        OpCode::Fn,
        Operand { mode: AddressingModes::Immediate, register: None, immediate: Some(fn_hash_raw) }
    ));
    let fn_start = assembler.instructions.len() - 1;

    let saved_stack_depth = assembler.stack_depth;
    assembler.stack_depth = 0;

    // Prologue
    assembler.instructions.push(create_instruction!(
        OpCode::Push,
        Operand { mode: AddressingModes::Register, register: Some(Registers::FP), immediate: None }
    ));
    assembler.instructions.push(create_instruction!(
        OpCode::Mov,
        Operand { mode: AddressingModes::Register, register: Some(Registers::FP), immediate: None },
        Operand { mode: AddressingModes::Register, register: Some(Registers::SP), immediate: None }
    ));

    // Get explicit constructor params from the inner page's ConstructorParameter items.
    // These are the named params in co(...).
    let ctor_params: Vec<ellie_core::definite::items::constructor_parameter::ConstructorParameter> =
        assembler.module.pages.iter()
            .find(|p| p.hash == constructor.inner_page_id)
            .map(|p| p.items.iter().filter_map(|item| {
                if let Collecting::ConstructorParameter(cp) = item { Some(cp.clone()) } else { None }
            }).collect())
            .unwrap_or_default();

    let user_ctor_params: alloc::vec::Vec<_> = ctor_params.iter()
        .filter(|cp| cp.name != "self")
        .collect();

    // ClassCall pushes: self + one value per explicit constructor param.
    // num_params must match the actual push count so stack offsets are correct.
    let num_params = (1 + user_ctor_params.len()) as isize;

    // self is the first arg pushed (deepest on stack): cursor = -(num_params) - 1
    // Use hash=None so add_local always inserts a fresh entry (avoids corrupting
    // the class-registration local that shares the same class_hash).
    let self_cursor = -num_params - 1;
    assembler.add_local(LocalHeader {
        name: "self".to_string(),
        cursor: self_cursor,
        reference: None,
        hash: None,
        page_hash: class_page.hash,
        borrowed: None,
    });
    for (i, cp) in user_ctor_params.iter().enumerate() {
        let cursor = -(num_params - (i as isize + 1)) - 1;
        assembler.add_local(LocalHeader {
            name: cp.name.clone(),
            cursor,
            reference: None,
            hash: Some(cp.hash),
            page_hash: class_page.hash,
            borrowed: None,
        });
    }

    // Assemble constructor body from its inner page
    assemble_method_body(assembler, constructor.inner_page_id, class_page.hash);

    // Epilogue
    assembler.instructions.push(create_instruction!(
        OpCode::Mov,
        Operand { mode: AddressingModes::Register, register: Some(Registers::SP), immediate: None },
        Operand { mode: AddressingModes::Register, register: Some(Registers::FP), immediate: None }
    ));
    assembler.instructions.push(create_instruction!(
        OpCode::Pop,
        Operand { mode: AddressingModes::Register, register: Some(Registers::FP), immediate: None }
    ));
    assembler.instructions.push(create_instruction!(OpCode::Ret));

    let fn_end = assembler.instructions.len() - 1;

    assembler.debug_headers.push(DebugHeader {
        rtype: DebugHeaderType::Function,
        hash: limit_platform_size(class_hash, assembler.platform_attributes.architecture),
        start_end: (fn_start, fn_end),
        module_name: class_page.path.clone(),
        module_hash: class_page.hash,
        name: "constructor".to_string(),
        pos: constructor.pos,
    });

    assembler.stack_depth = saved_stack_depth;
}

fn compile_method(
    assembler: &mut crate::assembler::Assembler,
    function: &ellie_core::definite::items::function::Function,
    class_page: &ellie_parser::parser::ProcessedPage,
    class_hash: usize,
) {
    // Emit Fn #method_hash marker
    let fn_hash_raw: RawType = function.hash.into();
    assembler.instructions.push(create_instruction!(
        OpCode::Fn,
        Operand { mode: AddressingModes::Immediate, register: None, immediate: Some(fn_hash_raw) }
    ));
    let fn_start = assembler.instructions.len() - 1;

    let saved_stack_depth = assembler.stack_depth;
    assembler.stack_depth = 0;

    // Prologue
    assembler.instructions.push(create_instruction!(
        OpCode::Push,
        Operand { mode: AddressingModes::Register, register: Some(Registers::FP), immediate: None }
    ));
    assembler.instructions.push(create_instruction!(
        OpCode::Mov,
        Operand { mode: AddressingModes::Register, register: Some(Registers::FP), immediate: None },
        Operand { mode: AddressingModes::Register, register: Some(Registers::SP), immediate: None }
    ));

    // Get user params from method's inner page
    let user_params: Vec<ellie_core::definite::items::function_parameter::FunctionParameter> =
        assembler.module.pages.iter()
            .find(|p| p.hash == function.inner_page_id)
            .map(|p| {
                p.items.iter().filter_map(|item| {
                    if let Collecting::FunctionParameter(fp) = item { Some(fp.clone()) } else { None }
                }).collect()
            })
            .unwrap_or_default();

    // Total params = 1 (self) + user params
    let num_params = (1 + user_params.len()) as isize;

    // Register self as implicit first param (hash=None avoids colliding with class registration)
    let self_cursor = -num_params - 1;
    assembler.add_local(LocalHeader {
        name: "self".to_string(),
        cursor: self_cursor,
        reference: None,
        hash: None,
        page_hash: class_page.hash,
        borrowed: None,
    });

    // Register user params
    for (i, fp) in user_params.iter().enumerate() {
        let cursor = -(num_params - (i as isize + 1)) - 1;
        assembler.add_local(LocalHeader {
            name: fp.name.clone(),
            cursor,
            reference: None,
            hash: Some(fp.hash),
            page_hash: class_page.hash,
            borrowed: None,
        });
    }

    // Assemble method body from its inner page
    assemble_method_body(assembler, function.inner_page_id, class_page.hash);

    // Epilogue
    assembler.instructions.push(create_instruction!(
        OpCode::Mov,
        Operand { mode: AddressingModes::Register, register: Some(Registers::SP), immediate: None },
        Operand { mode: AddressingModes::Register, register: Some(Registers::FP), immediate: None }
    ));
    assembler.instructions.push(create_instruction!(
        OpCode::Pop,
        Operand { mode: AddressingModes::Register, register: Some(Registers::FP), immediate: None }
    ));
    assembler.instructions.push(create_instruction!(OpCode::Ret));

    let fn_end = assembler.instructions.len() - 1;

    assembler.debug_headers.push(DebugHeader {
        rtype: DebugHeaderType::Function,
        hash: limit_platform_size(function.hash, assembler.platform_attributes.architecture),
        start_end: (fn_start, fn_end),
        module_name: class_page.path.clone(),
        module_hash: class_page.hash,
        name: function.name.clone(),
        pos: function.pos,
    });

    if function.name == "main" {
        assembler.main_function = Some(MainFunction {
            hash: function.hash,
            start: fn_start,
            end: fn_end,
        });
    }

    assembler.stack_depth = saved_stack_depth;
}

fn assemble_method_body(
    assembler: &mut crate::assembler::Assembler,
    inner_page_id: usize,
    _parent_page_hash: usize,
) {
    // Mark as processed to avoid re-entry
    if assembler.processed.contains(&inner_page_id) {
        return;
    }
    assembler.processed.push(inner_page_id);

    let inner_page = match assembler.module.pages.iter().find(|p| p.hash == inner_page_id).cloned() {
        Some(p) => p,
        None => return,
    };

    // Process dependencies
    for dep in &inner_page.dependencies {
        assembler.assemble_dependency(&dep.hash);
    }

    // Dispatch items — same as assemble_dependency but also handles SetterCall
    for item in &inner_page.items.clone() {
        match item {
            Collecting::Variable(variable) => {
                variable.transpile(assembler, inner_page.hash, &inner_page);
            }
            Collecting::Function(function) => {
                function.transpile(assembler, inner_page.hash, &inner_page);
            }
            Collecting::NativeFunction(nfn) => {
                nfn.transpile(assembler, inner_page.hash, &inner_page);
            }
            Collecting::GetterCall(getter_call) => {
                let mut deps = vec![inner_page.hash];
                deps.extend(inner_page.dependencies.iter().map(|d| d.hash));
                let mut opts = TypeTranspilerOptions::new();
                opts.set_assembler(assembler)
                    .set_dependencies(deps)
                    .set_target_page(inner_page.hash);
                getter_call.data.transpile(&mut opts);
            }
            Collecting::Ret(ret) => {
                ret.transpile(assembler, inner_page.hash, &inner_page);
            }
            Collecting::Condition(condition) => {
                condition.transpile(assembler, inner_page.hash, &inner_page);
            }
            Collecting::SetterCall(setter_call) => {
                // Delegate to the assembler's SetterCall handling
                let mut deps = vec![inner_page.hash];
                deps.extend(inner_page.dependencies.iter().map(|d| d.hash));
                dispatch_setter_call(assembler, setter_call, &inner_page, deps);
            }
            Collecting::FunctionParameter(_) | Collecting::ConstructorParameter(_) | Collecting::SelfItem(_) => {}
            Collecting::Import(_) | Collecting::FileKey(_) | Collecting::Generic(_) => {}
            _ => {
                std::println!("Skipping method body item: {:?}", item);
            }
        }
    }
}

fn dispatch_setter_call(
    assembler: &mut crate::assembler::Assembler,
    setter_call: &ellie_core::definite::items::setter_call::SetterCall,
    inner_page: &ellie_parser::parser::ProcessedPage,
    deps: Vec<usize>,
) {
    use ellie_core::definite::types::{class_instance::AttributeType, Types};

    // Evaluate RHS value → A
    {
        let mut opts = TypeTranspilerOptions::new();
        opts.set_assembler(assembler)
            .set_dependencies(deps)
            .set_target_page(inner_page.hash);
        setter_call.value.transpile(&mut opts);
    }

    match &setter_call.target {
        Types::Reference(ref_type) => {
            if let Some(prop) = ref_type.index_chain.iter()
                .find(|a| a.rtype == AttributeType::Property)
            {
                let field_idx = assembler.get_field_idx_by_hash(prop.hash);
                let self_local = assembler.find_local_by_name("self").cloned();
                if let (Some(field_idx), Some(self_local)) = (field_idx, self_local) {
                    // Mov B, A (save value)
                    assembler.instructions.push(create_instruction!(
                        OpCode::Mov,
                        Operand { mode: AddressingModes::Register, register: Some(Registers::B), immediate: None },
                        Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None }
                    ));
                    // Load self (heap ref) to A
                    let offset: isize = self_local.cursor;
                    let mut data = [0_u8; 8];
                    data.copy_from_slice(&offset.to_le_bytes());
                    let raw = RawType { type_id: ellie_core::bytecode::TypeId::Int, size: 8, data };
                    assembler.instructions.push(create_instruction!(
                        OpCode::Mov,
                        Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None },
                        Operand { mode: AddressingModes::IndirectOffset, register: Some(Registers::FP), immediate: Some(raw) }
                    ));
                    // Sfld A, #field_idx (heap[A].fields[idx] = B)
                    let idx_raw: RawType = field_idx.into();
                    assembler.instructions.push(create_instruction!(
                        OpCode::Sfld,
                        Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None },
                        Operand { mode: AddressingModes::Immediate, register: None, immediate: Some(idx_raw) }
                    ));
                } else {
                    std::println!("Warning: could not resolve self.field assignment (field_idx={:?})", field_idx);
                }
            }
        }
        Types::VariableType(var) => {
            let local = assembler.find_local_by_hash(var.reference).cloned();
            if let Some(local) = local {
                let offset: isize = local.cursor;
                let mut data = [0_u8; 8];
                data.copy_from_slice(&offset.to_le_bytes());
                let raw = RawType { type_id: ellie_core::bytecode::TypeId::Int, size: 8, data };
                assembler.instructions.push(create_instruction!(
                    OpCode::Mov,
                    Operand { mode: AddressingModes::IndirectOffset, register: Some(Registers::FP), immediate: Some(raw) },
                    Operand { mode: AddressingModes::Register, register: Some(Registers::A), immediate: None }
                ));
            }
        }
        _ => {
            std::println!("Skipping unimplemented setter target type in method body");
        }
    }
}
