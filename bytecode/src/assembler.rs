use crate::instructions;
use crate::transpiler::Transpiler;
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use ellie_core::defs::{DebugHeader, PlatformArchitecture};
use ellie_core::utils::ExportPage;
use ellie_parser::parser::Module;
use std::{io::Write, panic};

pub struct Assembler {
    pub(crate) module: Module,
    pub(crate) processed: Vec<usize>,
    pub(crate) platform_attributes: PlatformAttributes,
    pub(crate) instructions: Vec<instructions::Instructions>,
    pub(crate) locals: Vec<LocalHeader>,
    pub(crate) debug_headers: Vec<DebugHeader>,
}

#[derive(Clone, Debug)]
pub struct LocalHeader {
    pub name: String,
    pub cursor: usize,
    pub page_hash: usize,
    pub reference: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct InstructionPage {
    pub is_main: bool,
    pub hash: usize,
    pub instructions: Vec<instructions::Instructions>,
    pub locals: Vec<LocalHeader>,
    pub debug_headers: Vec<DebugHeader>,
}

impl ExportPage for InstructionPage {
    fn get_hash(&self) -> usize {
        self.hash
    }
}

impl InstructionPage {
    pub fn assign_instruction(&mut self, instruction: instructions::Instructions) {
        self.instructions.push(instruction)
    }

    pub fn extend_instructions(&mut self, instruction: Vec<instructions::Instructions>) {
        self.instructions.extend(instruction)
    }

    pub fn find_local(&self, name: &String) -> Option<&LocalHeader> {
        self.locals.iter().find(|_local| &_local.name == name)
    }
}

#[derive(Clone, Debug)]
pub struct PlatformAttributes {
    pub architecture: PlatformArchitecture,
    pub memory_size: usize,
}

#[derive(Clone, Debug)]
pub struct ModuleInfo {
    pub name: String,
    pub modue_maps: Vec<(String, Option<String>)>,
    pub is_library: bool,
    pub main_function: Option<usize>,
    pub platform_attributes: PlatformAttributes,
}

pub struct AssembleResult {
    pub module_info: ModuleInfo,
    pub debug_headers: Vec<DebugHeader>,
    pub locals: Vec<LocalHeader>,
    pub instructions: Vec<instructions::Instructions>,
}

impl AssembleResult {
    pub fn render_binary<T: Write, E: Write>(&self, writer: &mut T, dbg_w: &mut E) {
        for (module_name, path) in &self.module_info.modue_maps {
            dbg_w
                .write_all(
                    format!(
                        "{}: {}\n",
                        module_name,
                        path.clone().unwrap_or("-   ".to_string())
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
        match self.module_info.main_function {
            Some(main_fn_cursor) => {
                writer.write_all(&main_fn_cursor.to_le_bytes()).unwrap();
            }
            None => (),
        }

        for instruction in &self.instructions {
            writer
                .write(&instruction.op_code(&self.module_info.platform_attributes.architecture))
                .unwrap();
        }
    }

    pub fn alternate_render<T: Write>(&self, mut output: T) {
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
        match self.module_info.main_function {
            Some(main_fn_cursor) => {
                output
                    .write_all(format!(".main {}\n", main_fn_cursor).as_bytes())
                    .unwrap();
            }
            None => (),
        }
        output.write_all(".locals".as_bytes()).unwrap();

        for local in &self.locals {
            output
                .write_all(
                    format!(" {} {} {}", local.name, local.cursor, local.page_hash).as_bytes(),
                )
                .unwrap();
            match local.reference {
                Some(reference) => {
                    output
                        .write_all(format!(" {}", reference).as_bytes())
                        .unwrap();
                }
                None => (),
            }
            output.write_all("\n".as_bytes()).unwrap();
        }

        output.write_all(".debugHeader".as_bytes()).unwrap();

        for debug_header in &self.debug_headers {
            output
                .write_all(
                    format!(
                        "\n\t{:?} = {} : {}",
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

        output.write_all("\n".as_bytes()).unwrap();

        let mut count = 0;

        for instruction in &self.instructions {
            let code = format!("{}: {}\n", count, instruction,);
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
        self.instructions.len() - 1
    }

    pub fn find_local(&self, name: &String, page_hash: Option<Vec<usize>>) -> Option<&LocalHeader> {
        self.locals.iter().find(|local| {
            &local.name == name
                && match &page_hash {
                    Some(e) => e.contains(&local.page_hash),
                    None => true,
                }
        })
    }

    pub(crate) fn assemble_dependency(&mut self, hash: &usize) -> Option<usize> {
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

        let mut main_pos = None;

        for item in &processed_page.items {
            match item {
                ellie_core::definite::items::Collecting::Variable(variable) => {
                    variable.transpile(self, processed_page.hash as usize, &processed_page)
                }
                ellie_core::definite::items::Collecting::Function(function) => {
                    if function.name == "main" {
                        main_pos = Some(self.instructions.len());
                    }
                    function.transpile(self, processed_page.hash as usize, &processed_page)
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
                ellie_core::definite::items::Collecting::FuctionParameter(function_parameter) => {
                    function_parameter.transpile(
                        self,
                        processed_page.hash as usize,
                        &processed_page,
                    )
                }
                ellie_core::definite::items::Collecting::ConstructorParameter(_) => true,
                ellie_core::definite::items::Collecting::SelfItem(_) => true,
                ellie_core::definite::items::Collecting::Extend(_) => true,
                ellie_core::definite::items::Collecting::Loop(loop_type) => {
                    loop_type.transpile(self, processed_page.hash as usize, &processed_page)
                }
            };
        }
        main_pos
    }

    pub fn assemble(&mut self, modue_maps: Vec<(String, Option<String>)>) -> AssembleResult {
        let main_function = self.assemble_dependency(&self.module.initial_page.clone());
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
        }
    }
}
