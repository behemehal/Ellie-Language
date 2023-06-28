use crate::instructions::Instruction;
use crate::transpiler::Transpiler;
use crate::utils::usize_to_le_bytes;
use crate::{instruction_table, utils::limit_platform_size};
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use ellie_core::defs::{DebugHeader, ModuleMap, PlatformArchitecture};
use ellie_core::utils::ExportPage;
use ellie_parser::parser::Module;
use std::{io::Write, panic};

pub struct Assembler {
    pub(crate) module: Module,
    pub(crate) processed: Vec<usize>,
    pub(crate) platform_attributes: PlatformAttributes,
    pub(crate) instructions: Vec<instruction_table::Instructions>,
    pub(crate) locals: Vec<LocalHeader>,
    pub(crate) debug_headers: Vec<DebugHeader>,
}

#[derive(Clone, Debug)]
pub struct LocalHeader {
    pub name: String,
    pub cursor: usize,
    pub reference: Instruction,
    pub hash: Option<usize>,
    pub page_hash: usize,
    pub borrowed: Option<Vec<usize>>,
}

#[derive(Clone, Debug)]
pub struct InstructionPage {
    pub is_main: bool,
    pub hash: usize,
    pub instructions: Vec<instruction_table::Instructions>,
    pub locals: Vec<LocalHeader>,
    pub debug_headers: Vec<DebugHeader>,
}

impl ExportPage for InstructionPage {
    fn get_hash(&self) -> usize {
        self.hash
    }
}

impl InstructionPage {
    pub fn assign_instruction(&mut self, instruction: instruction_table::Instructions) {
        self.instructions.push(instruction)
    }

    pub fn extend_instructions(&mut self, instruction: Vec<instruction_table::Instructions>) {
        self.instructions.extend(instruction)
    }

    pub fn find_local(&self, name: &String) -> Option<&LocalHeader> {
        self.locals.iter().find(|_local| &_local.name == name)
    }

    pub fn find_local_by_hash(&self, hash: usize) -> Option<&LocalHeader> {
        self.locals
            .iter()
            .find(|_local| matches!(_local.hash, Some(e) if e == hash))
    }
}

#[derive(Clone, Debug)]
pub struct PlatformAttributes {
    pub architecture: PlatformArchitecture,
    pub memory_size: usize,
}

#[derive(Clone, Debug)]
pub struct MainFunction {
    pub hash: usize,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug)]
pub struct ModuleInfo {
    pub name: String,
    pub modue_maps: Vec<ModuleMap>,
    pub is_library: bool,
    pub main_function: Option<MainFunction>,
    pub platform_attributes: PlatformAttributes,
}

#[derive(Debug, Clone)]
pub struct NativeCall {
    pub name: String,
    pub hash: usize,
}

pub struct AssembleResult {
    pub module_info: ModuleInfo,
    pub debug_headers: Vec<DebugHeader>,
    pub locals: Vec<LocalHeader>,
    pub instructions: Vec<instruction_table::Instructions>,
    pub native_exports: Vec<NativeCall>,
}

impl AssembleResult {
    pub fn render_binary_to_vector(&self) -> Vec<u8> {
        let arch = self.module_info.platform_attributes.architecture;
        let mut binary = Vec::new();
        binary
            .write(&[match self.module_info.platform_attributes.architecture {
                PlatformArchitecture::B16 => 16_u8,
                PlatformArchitecture::B32 => 32_u8,
                PlatformArchitecture::B64 => 64_u8,
            }])
            .unwrap();
        binary
            .write(&[if self.module_info.main_function.is_some() {
                1
            } else {
                0
            }])
            .unwrap();
        if let Some(main_fn) = &self.module_info.main_function {
            binary
                .write_all(&usize_to_le_bytes(main_fn.start, arch))
                .unwrap();
            binary
                .write_all(&usize_to_le_bytes(main_fn.end, arch))
                .unwrap();
            binary
                .write_all(&usize_to_le_bytes(main_fn.hash, arch))
                .unwrap();
        }

        for instruction in &self.instructions {
            binary
                .write(&instruction.op_code(self.module_info.platform_attributes.architecture))
                .unwrap();
        }
        binary
    }

    pub fn render_binary<T: Write, E: Write>(&self, writer: &mut T, dbg_w: &mut E) {
        let arch = self.module_info.platform_attributes.architecture;
        for module_map in &self.module_info.modue_maps {
            dbg_w
                .write_all(
                    format!(
                        "{}: {}\n",
                        module_map.module_name,
                        module_map.module_path.clone().unwrap_or("-   ".to_string())
                    )
                    .as_bytes(),
                )
                .unwrap();
        }
        dbg_w.write_all(b"---\n").unwrap();
        for (idx, header) in self.debug_headers.iter().enumerate() {
            dbg_w
                .write_all(
                    format!(
                        "{}:{}:{}:{}:{}:{}:{}:{}:{}{}",
                        header.start_end.0,
                        header.start_end.1,
                        header.module,
                        header.name,
                        header.pos.range_start.0,
                        header.pos.range_start.1,
                        header.pos.range_end.0,
                        header.pos.range_end.1,
                        header.hash,
                        if idx != self.debug_headers.len() - 1 {
                            "\n"
                        } else {
                            ""
                        },
                    )
                    .as_bytes(),
                )
                .unwrap();
        }
        writer
            .write(&[match self.module_info.platform_attributes.architecture {
                PlatformArchitecture::B16 => 16_u8,
                PlatformArchitecture::B32 => 32_u8,
                PlatformArchitecture::B64 => 64_u8,
            }])
            .unwrap();
        writer
            .write(&[if self.module_info.main_function.is_some() {
                1
            } else {
                0
            }])
            .unwrap();
        if let Some(main_fn) = &self.module_info.main_function {
            writer
                .write_all(&usize_to_le_bytes(main_fn.start, arch))
                .unwrap();
            writer
                .write_all(&usize_to_le_bytes(main_fn.end, arch))
                .unwrap();
            writer
                .write_all(&usize_to_le_bytes(main_fn.hash, arch))
                .unwrap();
        }

        for instruction in &self.instructions {
            writer
                .write(&instruction.op_code(self.module_info.platform_attributes.architecture))
                .unwrap();
        }
    }

    pub fn alternate_render<T: Write>(&self, output: &mut T) {
        output
            .write_all(
                format!(
                    ".arch {}\n",
                    match self.module_info.platform_attributes.architecture {
                        PlatformArchitecture::B16 => "16",
                        PlatformArchitecture::B32 => "32",
                        PlatformArchitecture::B64 => "64",
                    }
                )
                .as_bytes(),
            )
            .unwrap();
        match &self.module_info.main_function {
            Some(main_fn) => {
                output
                    .write_all(
                        format!(
                            ".main {}: {} @ {}\n",
                            limit_platform_size(
                                main_fn.start,
                                self.module_info.platform_attributes.architecture
                            ),
                            limit_platform_size(
                                main_fn.end,
                                self.module_info.platform_attributes.architecture
                            ),
                            limit_platform_size(
                                main_fn.hash,
                                self.module_info.platform_attributes.architecture
                            )
                        )
                        .as_bytes(),
                    )
                    .unwrap();
            }
            None => (),
        }
        output.write_all(".locals".as_bytes()).unwrap();

        for local in &self.locals {
            output
                .write_all(
                    format!(
                        "\n{}: {} = {}{}",
                        local.cursor,
                        local.name,
                        local.reference.addressing_mode.to_string(),
                        match local.hash {
                            Some(reference) => {
                                format!("({})", reference)
                            }
                            None => {
                                String::new()
                            }
                        }
                    )
                    .as_bytes(),
                )
                .unwrap();
        }

        output.write_all("\n.debugHeader".as_bytes()).unwrap();

        for debug_header in &self.debug_headers {
            output
                .write_all(
                    format!(
                        "\n{:?} = {} : {}",
                        debug_header.rtype,
                        {
                            if debug_header.start_end.1 == (debug_header.start_end.0 + 1) {
                                debug_header.start_end.0.to_string()
                            } else {
                                format!("{}~{}", debug_header.start_end.0, debug_header.start_end.1)
                            }
                        },
                        debug_header.hash
                    )
                    .as_bytes(),
                )
                .unwrap()
        }

        output.write_all("\n.instructions".as_bytes()).unwrap();

        let mut count = 0;

        for instruction in &self.instructions {
            let code = format!(
                "\n{}: {} = {} : {:?}",
                count,
                instruction,
                instruction.op_code(self.module_info.platform_attributes.architecture)[0],
                instruction.op_code(self.module_info.platform_attributes.architecture)[1..]
                    .to_vec(),
            );
            output.write_all(&code.as_bytes()).unwrap();
            count += 1;
        }
    }
}

#[derive(Clone)]
pub enum PageType {
    Raw,
    Fn,
    Condition,
    Loop,
    Class,
}

impl Assembler {
    pub fn new(module: Module, platform_attributes: PlatformAttributes) -> Assembler {
        Assembler {
            module,
            platform_attributes,
            processed: Vec::new(),
            instructions: Vec::new(),
            locals: Vec::new(),
            debug_headers: Vec::new(),
        }
    }

    pub fn location(&self) -> usize {
        if self.instructions.len() == 0 {
            0
        } else {
            self.instructions.len() - 1
        }
    }

    pub fn find_local_by_hash(
        &mut self,
        hash: usize,
        page_hash: Option<Vec<usize>>,
        borrow: bool,
    ) -> Option<LocalHeader> {
        let mut locals: Vec<&LocalHeader> = self
            .locals
            .iter()
            .filter(|filter| match &page_hash {
                Some(page_hash) => page_hash.contains(&filter.page_hash),
                None => true,
            })
            .collect();
        locals.sort_by(|a, b| a.cursor.cmp(&b.cursor));
        locals.reverse();
        match locals
            .iter_mut()
            .find(|local| matches!(local.hash, Some(local_hash) if local_hash == hash))
        {
            Some(local) => Some(local.clone()),
            None => {
                if page_hash.is_none() || !borrow {
                    return None;
                }
                match self.module.pages.clone().into_iter().find_map(|x| {
                    if page_hash.clone().unwrap().contains(&x.hash) {
                        x.items.clone().into_iter().find_map(|e| match e {
                            ellie_core::definite::items::Collecting::Function(function) => {
                                if function.hash == hash {
                                    Some(LocalHeader {
                                        name: function.name.clone(),
                                        cursor: 0,
                                        reference: Instruction::absolute_static(0),
                                        hash: Some(function.hash),
                                        page_hash: x.hash,
                                        borrowed: Some(Vec::new()),
                                    })
                                } else {
                                    None
                                }
                            }
                            ellie_core::definite::items::Collecting::Variable(variable) => {
                                if variable.hash == hash {
                                    Some(LocalHeader {
                                        name: variable.name.clone(),
                                        cursor: 0,
                                        reference: Instruction::absolute_static(0),
                                        hash: Some(variable.hash),
                                        page_hash: x.hash,
                                        borrowed: Some(Vec::new()),
                                    })
                                } else {
                                    None
                                }
                            }
                            ellie_core::definite::items::Collecting::NativeFunction(nfunction) => {
                                if nfunction.hash == hash {
                                    Some(LocalHeader {
                                        name: nfunction.name.clone(),
                                        cursor: 0,
                                        reference: Instruction::absolute_static(0),
                                        hash: Some(nfunction.hash),
                                        page_hash: x.hash,
                                        borrowed: Some(Vec::new()),
                                    })
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        })
                    } else {
                        None
                    }
                }) {
                    Some(local) => {
                        self.locals.push(local.clone());
                        Some(local)
                    }
                    None => None,
                }
            }
        }
    }

    pub fn add_borrow_to_local(&mut self, hash: usize, cursor: usize) {
        let local = self.locals.iter_mut().find(|local| {
            matches!(local.hash, Some(local_hash) if local_hash == hash) && local.borrowed.is_some()
        });
        match local {
            Some(local) => match local.borrowed {
                Some(ref mut borrowed) => {
                    borrowed.push(cursor);
                }
                None => (),
            },
            None => (),
        }
    }

    pub fn add_local(&mut self, local: LocalHeader) {
        let location = self.location();
        match self.locals.iter_mut().find(|x| x.hash == local.hash) {
            Some(local) => {
                //If the function already preserved with find we will just update the reference
                local.reference = Instruction::absolute_static(location);
                local.cursor = location;
                if let Some(e) = &local.borrowed {
                    for cursor in e {
                        self.instructions[*cursor]
                            .get_addressing_mode_mut()
                            .addressing_mode =
                            crate::addressing_modes::AddressingModes::Absolute(location);
                    }
                }
            }
            None => {
                self.locals.push(local);
            }
        }
    }

    pub fn find_local(
        &mut self,
        name: &String,
        page_hash: Option<Vec<usize>>,
        borrow: bool,
    ) -> Option<LocalHeader> {
        let mut locals: Vec<&LocalHeader> = self
            .locals
            .iter()
            .filter(|filter| match &page_hash {
                Some(page_hash) => page_hash.contains(&filter.page_hash),
                None => true,
            })
            .collect();

        locals.sort_by(|a, b| a.cursor.cmp(&b.cursor));
        locals.reverse();
        match locals.iter_mut().find(|local| &local.name == name) {
            Some(local) => Some(local.clone()),
            None => {
                if page_hash.is_none() || !borrow {
                    return None;
                }
                match self.module.pages.clone().into_iter().find_map(|x| {
                    if page_hash.clone().unwrap().contains(&x.hash) {
                        x.items.clone().into_iter().find_map(|e| match e {
                            ellie_core::definite::items::Collecting::Function(function) => {
                                if &function.name == name {
                                    Some(LocalHeader {
                                        name: function.name.clone(),
                                        cursor: 0,
                                        reference: Instruction::absolute_static(0),
                                        hash: Some(function.hash),
                                        page_hash: x.hash,
                                        borrowed: Some(Vec::new()),
                                    })
                                } else {
                                    None
                                }
                            }
                            ellie_core::definite::items::Collecting::Variable(variable) => {
                                if &variable.name == name {
                                    Some(LocalHeader {
                                        name: variable.name.clone(),
                                        cursor: 0,
                                        reference: Instruction::absolute_static(0),
                                        hash: Some(variable.hash),
                                        page_hash: x.hash,
                                        borrowed: Some(Vec::new()),
                                    })
                                } else {
                                    None
                                }
                            }
                            ellie_core::definite::items::Collecting::NativeFunction(nfunction) => {
                                if &nfunction.name == name {
                                    Some(LocalHeader {
                                        name: nfunction.name.clone(),
                                        cursor: 0,
                                        reference: Instruction::absolute_static(0),
                                        hash: Some(nfunction.hash),
                                        page_hash: x.hash,
                                        borrowed: Some(Vec::new()),
                                    })
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        })
                    } else {
                        None
                    }
                }) {
                    Some(local) => {
                        self.locals.push(local.clone());
                        Some(local)
                    }
                    None => None,
                }
            }
        }
    }

    pub(crate) fn assemble_dependency(&mut self, hash: &usize) -> Option<MainFunction> {
        if self.processed.contains(hash) {
            return None;
        }
        self.processed.push(*hash);

        let processed_page = self
            .module
            .pages
            .clone()
            .into_iter()
            .find(|x| x.hash == hash.clone())
            .unwrap_or_else(|| {
                panic!("Unexpected assembler error, cannot find page {:?}", hash);
            });

        for dependency in &processed_page.dependencies {
            self.assemble_dependency(&dependency.hash);
        }

        let mut main_function = None;

        for item in &processed_page.items {
            match item {
                ellie_core::definite::items::Collecting::Variable(variable) => {
                    variable.transpile(self, processed_page.hash as usize, &processed_page)
                }
                ellie_core::definite::items::Collecting::Function(function) => {
                    let start = self.instructions.len();
                    let transpile_res =
                        function.transpile(self, processed_page.hash as usize, &processed_page);
                    if function.name == "main" {
                        main_function = Some(MainFunction {
                            hash: function.hash,
                            start,
                            end: self.location(),
                        });
                    }
                    transpile_res
                }
                ellie_core::definite::items::Collecting::ForLoop(for_loop) => {
                    for_loop.transpile(self, processed_page.hash as usize, &processed_page)
                }
                ellie_core::definite::items::Collecting::Condition(condition) => {
                    condition.transpile(self, processed_page.hash as usize, &processed_page)
                }
                ellie_core::definite::items::Collecting::Class(class) => {
                    class.transpile(self, processed_page.hash as usize, &processed_page)
                }
                ellie_core::definite::items::Collecting::Ret(ret) => {
                    ret.transpile(self, processed_page.hash as usize, &processed_page)
                }
                ellie_core::definite::items::Collecting::Constructor(constructor) => {
                    constructor.transpile(self, processed_page.hash as usize, &processed_page)
                }
                ellie_core::definite::items::Collecting::Import(_) => true,
                ellie_core::definite::items::Collecting::FileKey(_) => true,
                ellie_core::definite::items::Collecting::Getter(_) => todo!(),
                ellie_core::definite::items::Collecting::Setter(_) => todo!(),
                ellie_core::definite::items::Collecting::Generic(_) => true,
                ellie_core::definite::items::Collecting::GetterCall(getter_call) => {
                    getter_call.transpile(self, processed_page.hash as usize, &processed_page)
                }
                ellie_core::definite::items::Collecting::SetterCall(setter_call) => {
                    setter_call.transpile(self, processed_page.hash as usize, &processed_page)
                }
                ellie_core::definite::items::Collecting::Enum(_) => todo!(),
                ellie_core::definite::items::Collecting::NativeFunction(native_function) => {
                    native_function.transpile(self, processed_page.hash as usize, &processed_page)
                }
                ellie_core::definite::items::Collecting::None => todo!(),
                ellie_core::definite::items::Collecting::Brk(_) => todo!(),
                ellie_core::definite::items::Collecting::Go(_) => todo!(),
                ellie_core::definite::items::Collecting::FunctionParameter(function_parameter) => {
                    function_parameter.transpile(
                        self,
                        processed_page.hash as usize,
                        &processed_page,
                    )
                }
                ellie_core::definite::items::Collecting::ConstructorParameter(_) => true,
                ellie_core::definite::items::Collecting::SelfItem(self_item) => {
                    self_item.transpile(self, processed_page.hash as usize, &processed_page)
                }
                ellie_core::definite::items::Collecting::Extend(_) => true,
                ellie_core::definite::items::Collecting::Loop(loop_type) => {
                    loop_type.transpile(self, processed_page.hash as usize, &processed_page)
                }
                ellie_core::definite::items::Collecting::ClassInstance(class_instance) => {
                    class_instance.transpile(self, processed_page.hash as usize, &processed_page)
                }
            };
        }
        main_function
    }

    pub fn assemble(&mut self, modue_maps: Vec<ModuleMap>) -> AssembleResult {
        let main_function = self.assemble_dependency(&self.module.initial_page.clone());
        //self.instructions.push(instruction_table::Instructions::RET(Instruction { addressing_mode: AddressingModes::Implicit }));
        let mut native_exports = Vec::new();

        for native_header in &self.debug_headers {
            match native_header.rtype {
                ellie_core::defs::DebugHeaderType::NativeFunction => {
                    native_exports.push(NativeCall {
                        name: native_header.name.clone(),
                        hash: native_header.hash,
                    });
                }
                _ => (),
            }
        }

        AssembleResult {
            module_info: ModuleInfo {
                name: self.module.name.clone(),
                is_library: self.module.is_library,
                platform_attributes: self.platform_attributes.clone(),
                modue_maps,
                main_function,
            },
            locals: self.locals.clone(),
            debug_headers: self.debug_headers.clone(),
            instructions: self.instructions.clone(),
            native_exports,
        }
    }
}
