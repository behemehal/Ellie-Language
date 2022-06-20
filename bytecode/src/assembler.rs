use crate::instructions::{self};
use crate::transpiler::Transpiler;
use alloc::{format, string::String, vec::Vec};
use ellie_core::defs::PlatformArchitecture;
use ellie_core::utils::ExportPage;
use ellie_parser::parser::Module;
use std::{io::Write, panic, println};

pub struct Assembler {
    pub(crate) module: Module,
    pub(crate) processed: Vec<usize>,
    pub(crate) platform_attributes: PlatformAttributes,
    pub(crate) instructions: Vec<instructions::Instructions>,
    pub(crate) locals: Vec<LocalHeader>,
    pub(crate) debug_headers: Vec<DebugHeader>,
}

#[derive(Clone, Debug)]
pub enum DebugHeaderType {
    Variable,
    Class,
    Parameter,
}

#[derive(Clone, Debug)]
pub struct DebugHeader {
    pub id: usize,
    pub rtype: DebugHeaderType,
    pub name: String,
    pub cursor: ellie_core::defs::Cursor,
}

#[derive(Clone, Debug)]
pub struct LocalHeader {
    pub name: String,
    pub cursor: usize,
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

pub struct AssembleResult {
    pub instructions: Vec<instructions::Instructions>,
    pub platform_attributes: PlatformAttributes,
    pub main_function: Option<usize>,
}

impl AssembleResult {
    pub fn render_binary<T: Write>(&self, writer: &mut T, _header: Option<&mut T>) {
        writer
            .write(&[match self.platform_attributes.architecture {
                PlatformArchitecture::B16 => 16_u8,
                PlatformArchitecture::B32 => 32_u8,
                PlatformArchitecture::B64 => 64_u8,
            }])
            .unwrap();
        writer
            .write(&[if self.main_function.is_some() { 1 } else { 0 }])
            .unwrap();
        match self.main_function {
            Some(main_fn_cursor) => {
                writer.write_all(&main_fn_cursor.to_le_bytes()).unwrap();
            }
            None => (),
        }

        for instruction in &self.instructions {
            let op_code = instruction.op_code();
            println!("{:?}", op_code);
            writer.write(&instruction.op_code()).unwrap();
        }
    }

    pub fn alternate_render<T: Write>(&self, mut output: T) {
        output
            .write_all(
                format!(
                    ".arch {}\n",
                    match self.platform_attributes.architecture {
                        PlatformArchitecture::B16 => "16",
                        PlatformArchitecture::B32 => "32",
                        PlatformArchitecture::B64 => "64",
                    }
                )
                .as_bytes(),
            )
            .unwrap();
        match self.main_function {
            Some(main_fn_cursor) => {
                output
                    .write_all(format!(".main {}\n", main_fn_cursor).as_bytes())
                    .unwrap();
            }
            None => (),
        }

        let mut count = 0;

        for instruction in &self.instructions {
            let code = format!("{}: {}\n", count, instruction,);
            output.write_all(&code.as_bytes()).unwrap();
            count += 1;
        }
    }

    pub fn render<T: Write>(&self, mut output: T) {
        output
            .write_all(
                format!(
                    ".arch {}\n",
                    match self.platform_attributes.architecture {
                        PlatformArchitecture::B16 => "16",
                        PlatformArchitecture::B32 => "32",
                        PlatformArchitecture::B64 => "64",
                    }
                )
                .as_bytes(),
            )
            .unwrap();
        match self.main_function {
            Some(main_fn_cursor) => {
                output
                    .write_all(format!(".main {}\n", main_fn_cursor).as_bytes())
                    .unwrap();
            }
            None => (),
        }

        /*
        for (index, page) in self.pages.iter().enumerate() {
            let mut page_output = String::new();
            page_output += &alloc::format!("{:?}- {:?}:", index.to_le(), page.hash);
            page_output += &alloc::format!("\n\th:");
            for (index, header) in page.debug_headers.iter().enumerate() {
                page_output += &alloc::format!(
                    "\n\t\t{}: {} - {} - {}",
                    index,
                    header.cursor.range_start,
                    header.cursor.range_end,
                    header.name
                );
            }
            page_output += &alloc::format!("\n\tl:");
            for (index, header) in page.locals.iter().enumerate() {
                page_output += &alloc::format!(
                    "\n\t\t{}: {} - {}{}",
                    index,
                    header.name,
                    header.cursor,
                    match header.reference {
                        Some(e) => format!("~{}", e),
                        None => String::new(),
                    }
                );
            }
            page_output += &alloc::format!("\n\ta:");
            for instruction in &page.instructions {
                page_output += &alloc::format!("\n\t\t{}", instruction);
            }
            output.write_all(page_output.as_bytes()).unwrap();
            output.write_all("\n".as_bytes()).unwrap();
        }
        */
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

    pub fn find_local(&self, name: &String) -> Option<&LocalHeader> {
        self.locals.iter().find(|_local| &_local.name == name)
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
                ellie_core::definite::items::Collecting::Import(_) => {
                    std::println!("[Assembler,Ignore,Element] Import");
                    true
                }
                ellie_core::definite::items::Collecting::FileKey(_) => {
                    std::println!("[Assembler,Ignore,Element] FileKey");
                    true
                }
                ellie_core::definite::items::Collecting::Getter(_) => todo!(),
                ellie_core::definite::items::Collecting::Setter(_) => todo!(),
                ellie_core::definite::items::Collecting::Generic(_) => {
                    std::println!("[Assembler,Ignore,Element] Generic");
                    true
                }
                ellie_core::definite::items::Collecting::GetterCall(getter_call) => {
                    getter_call.transpile(self, processed_page.hash as usize, &processed_page)
                }
                ellie_core::definite::items::Collecting::SetterCall(setter_call) => {
                    setter_call.transpile(self, processed_page.hash as usize, &processed_page)
                }
                ellie_core::definite::items::Collecting::Enum(_) => todo!(),
                ellie_core::definite::items::Collecting::NativeFunction(function) => {
                    self.locals.push(LocalHeader {
                        name: function.name.clone(),
                        cursor: self.instructions.len(),
                        reference: None,
                    });
                    true
                    //std::println!("[Assembler,Ignore,Element] NativeFunction: {:?}", function)
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
                ellie_core::definite::items::Collecting::ConstructorParameter(_) => {
                    std::println!("[Assembler,Ignore,Element] ConstructorParameter");
                    true
                }
                ellie_core::definite::items::Collecting::SelfItem(_) => {
                    std::println!("[Assembler,Ignore,Element] SelfItem");
                    true
                }
            };
        }
        main_pos
    }

    pub fn assemble(&mut self) -> AssembleResult {
        /*
        if !self.module.is_library {

            let main_fn_inner_page_id = self.module.pages.find_page(self.module.initial_page).unwrap().items.iter().find_map(|x| match x {
                ellie_core::definite::items::Collecting::Function(e) => Some(e),
                _ => None
            }).unwrap().inner_page_id;
            self.pages.push_page(InstructionPage {
                is_main: true,
                hash: 0,
                instructions: vec![instructions::Instructions::CALL(Instruction::absolute(
                    main_fn_inner_page_id as usize,
                ))],
                locals: Vec::new(),
                debug_headers: Vec::new(),
            });
        }
        */
        let main_function = self.assemble_dependency(&self.module.initial_page.clone());
        AssembleResult {
            instructions: self.instructions.clone(),
            platform_attributes: self.platform_attributes.clone(),
            main_function,
        }
    }
}
