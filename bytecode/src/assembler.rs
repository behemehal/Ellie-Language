use std::io::{Read, Write};

use crate::{
    instructions::{self, AddressingModes, Instruction},
    utils,
};
use alloc::{format, string::String, vec::Vec};
use ellie_parser::parser::Module;

pub struct Assembler {
    module: Module,
    processed: Vec<u64>,
    platform_attributes: PlatformAttributes,
    used_stack_memory: usize,
    pages: Vec<InstructionPage>,
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
pub struct InstructionPage {
    pub is_main: bool,
    pub instructions: Vec<instructions::Instructions>,
    pub debug_headers: Vec<DebugHeader>,
}

impl InstructionPage {
    pub fn assign_instruction(&mut self, instruction: instructions::Instructions) {
        self.instructions.push(instruction)
    }
}

#[derive(Clone, Debug)]
pub enum PlatformArchitecture {
    B8,
    B16,
    B32,
    B64,
}

#[derive(Clone, Debug)]
pub struct PlatformAttributes {
    pub architecture: PlatformArchitecture,
    pub memory_size: usize,
}

pub struct AssembleResult {
    pub pages: Vec<InstructionPage>,
    pub platform_attributes: PlatformAttributes,
}

impl AssembleResult {
    pub fn render_binary<T: Write>(&self, writer: &mut T, header: Option<&mut T>) {
        writer
            .write(&[match self.platform_attributes.architecture {
                PlatformArchitecture::B8 => 8_u8,
                PlatformArchitecture::B16 => 16_u8,
                PlatformArchitecture::B32 => 32_u8,
                PlatformArchitecture::B64 => 64_u8,
            }])
            .unwrap();
        for page in &self.pages {
            for (index, page) in self.pages.iter().enumerate() {
                for instruction in &page.instructions {
                    std::println!(
                        "{:?} - {:?} - {}",
                        instruction,
                        instruction.op_code(),
                        instruction
                    );
                    writer.write(&instruction.op_code()).unwrap();
                }
            }
        }
    }

    pub fn render<T: Write>(&self, mut output: T) {
        output
            .write_all(
                format!(
                    ".arch {}\n",
                    match self.platform_attributes.architecture {
                        PlatformArchitecture::B8 => "8",
                        PlatformArchitecture::B16 => "16",
                        PlatformArchitecture::B32 => "32",
                        PlatformArchitecture::B64 => "64",
                    }
                )
                .as_bytes(),
            )
            .unwrap();
        for (index, page) in self.pages.iter().enumerate() {
            let mut page_output = String::new();
            page_output += &alloc::format!("{:?}:", index.to_le());
            page_output += &alloc::format!("\n\th:");
            for header in &page.debug_headers {
                page_output += &alloc::format!(
                    "\n\t\t{}: {} - {} - {}",
                    index,
                    header.cursor.range_start,
                    header.cursor.range_end,
                    header.name
                );
            }
            page_output += &alloc::format!("\n\ta:");
            for instruction in &page.instructions {
                page_output += &alloc::format!("\n\t\t{}", instruction);
            }
            output.write_all(page_output.as_bytes()).unwrap();
        }
    }
}

impl Assembler {
    pub fn new(module: Module, platform_attributes: PlatformAttributes) -> Assembler {
        Assembler {
            module,
            platform_attributes,
            processed: Vec::new(),
            pages: Vec::new(),
            used_stack_memory: 0,
        }
    }

    fn assemble_dependency(&mut self, hash: &u64, is_main: bool) {
        if !self.processed.contains(hash) {
            self.processed.push(hash.clone());
            let processed_page = self
                .module
                .pages
                .clone()
                .into_iter()
                .find(|x| x.hash == hash.clone())
                .unwrap_or_else(|| {
                    panic!("Unexpected assembler error, cannot find page {:?}", hash);
                });

            let mut page = InstructionPage {
                is_main,
                instructions: Vec::new(),
                debug_headers: Vec::new(),
            };

            for dependency in &processed_page.dependencies {
                self.assemble_dependency(&dependency.hash, false);
            }

            for item in &processed_page.items {
                match item {
                    ellie_core::definite::items::Collecting::Variable(variable) => {
                        page.debug_headers.push(DebugHeader {
                            id: page.instructions.len(),
                            rtype: DebugHeaderType::Variable,
                            name: variable.name.clone(),
                            cursor: variable.pos,
                        });

                        if utils::is_static_type(&variable.value) {
                            page.assign_instruction(instructions::Instructions::LDA(
                                Instruction::absolute(
                                    utils::convert_to_raw_type(variable.value.clone()).data,
                                ),
                            ));
                        } else {
                            panic!("Unimplemented: {:?}", variable);
                        }
                    }
                    ellie_core::definite::items::Collecting::Function(_) => {
                        std::println!("[Assembler,Ignore,Element] Function")
                    }
                    ellie_core::definite::items::Collecting::ForLoop(_) => {
                        std::println!("[Assembler,Ignore,Element] ForLoop")
                    }
                    ellie_core::definite::items::Collecting::Condition(_) => {
                        std::println!("[Assembler,Ignore,Element] Condition")
                    }
                    ellie_core::definite::items::Collecting::Class(_) => {
                        std::println!("[Assembler,Ignore,Element] Class")
                    }
                    ellie_core::definite::items::Collecting::Ret(_) => {
                        std::println!("[Assembler,Ignore,Element] Ret")
                    }
                    ellie_core::definite::items::Collecting::Constructor(_) => {
                        std::println!("[Assembler,Ignore,Element] NativeFunction")
                    }
                    ellie_core::definite::items::Collecting::Import(_) => {
                        std::println!("[Assembler,Ignore,Element] Import")
                    }
                    ellie_core::definite::items::Collecting::FileKey(_) => {
                        std::println!("[Assembler,Ignore,Element] FileKey")
                    }
                    ellie_core::definite::items::Collecting::Getter(_) => todo!(),
                    ellie_core::definite::items::Collecting::Setter(_) => todo!(),
                    ellie_core::definite::items::Collecting::Generic(_) => {
                        std::println!("[Assembler,Ignore,Element] Generic")
                    }
                    ellie_core::definite::items::Collecting::NativeClass => todo!(),
                    ellie_core::definite::items::Collecting::GetterCall(_) => todo!(),
                    ellie_core::definite::items::Collecting::SetterCall(_) => todo!(),
                    ellie_core::definite::items::Collecting::Enum(_) => todo!(),
                    ellie_core::definite::items::Collecting::NativeFunction(_) => {
                        std::println!("[Assembler,Ignore,Element] NativeFunction")
                    }
                    ellie_core::definite::items::Collecting::None => todo!(),
                }
            }
            self.pages.push(page);
        }
    }

    pub fn assemble(&mut self) -> AssembleResult {
        self.assemble_dependency(&self.module.initial_page.clone(), true);
        AssembleResult {
            pages: self.pages.clone(),
            platform_attributes: self.platform_attributes.clone(),
        }
    }
}
