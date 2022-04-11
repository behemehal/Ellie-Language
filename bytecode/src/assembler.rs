use crate::{instructions::{self, AddressingModes, Instruction}, utils};
use alloc::{string::String, vec::Vec};
use ellie_parser::parser::Module;

pub struct StackMemory {
    memory: Vec<u8>,
}

pub struct Assembler {
    module: Module,
    processed: Vec<u64>,
    platform_attributes: PlatformAttributes,
    used_heap_memory: usize,
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
    pub instructions: Vec<instructions::Instructions>,
    pub debug_headers: Vec<DebugHeader>,
}

impl InstructionPage {
    pub fn assign_instruction(&mut self, instruction: instructions::Instructions) {
        self.instructions.push(instruction)
    }
}

pub enum PlatformArchitecture {
    B8,
    B16,
    B32,
    B64,
}

pub struct PlatformAttributes {
    pub architecture: PlatformArchitecture,
    pub memory_size: usize,
}

pub struct AssembleResult {
    pub pages: Vec<InstructionPage>,
}

impl Assembler {
    pub fn new(module: Module, platform_attributes: PlatformAttributes) -> Assembler {
        Assembler {
            module,
            platform_attributes,
            processed: Vec::new(),
            pages: Vec::new(),
            used_heap_memory: 0,
        }
    }

    fn assemble_dependency(&mut self, hash: &u64) {
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
                instructions: Vec::new(),
                debug_headers: Vec::new(),
            };

            for dependency in &processed_page.dependencies {
                self.assemble_dependency(&dependency.hash);
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
                        
                        //page.assign_instruction(instructions::Instructions::LDA(
                        //    Instruction::absolute(utils::convert_to_raw_type(variable.value).join("").),
                        //));
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
                    ellie_core::definite::items::Collecting::Generic(_) => todo!(),
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

    pub fn assemble(&mut self) {
        self.assemble_dependency(&self.module.initial_page.clone());
        panic!("{:#?}", self.pages);
    }
}
