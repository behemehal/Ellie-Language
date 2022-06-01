use crate::instructions::{self, Instruction, Instructions};
use alloc::{format, string::String, vec, vec::Vec};
use ellie_core::{
    definite::{
        items::{function, function_parameter},
        types::{operator, reference, Types},
    },
    defs::Cursor,
    utils::{ExportPage, PageExport},
};
use ellie_parser::parser::Module;
use std::{io::Write, panic, print, println};

pub struct Assembler {
    module: Module,
    processed: Vec<u64>,
    platform_attributes: PlatformAttributes,
    pages: PageExport<InstructionPage>,
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
    fn get_hash(&self) -> u64 {
        self.hash as u64
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
    pub fn render_binary<T: Write>(&self, writer: &mut T, _header: Option<&mut T>) {
        writer
            .write(&[match self.platform_attributes.architecture {
                PlatformArchitecture::B8 => 8_u8,
                PlatformArchitecture::B16 => 16_u8,
                PlatformArchitecture::B32 => 32_u8,
                PlatformArchitecture::B64 => 64_u8,
            }])
            .unwrap();
        for page in &self.pages {
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
    }
}

impl Assembler {
    pub fn new(module: Module, platform_attributes: PlatformAttributes) -> Assembler {
        Assembler {
            module,
            platform_attributes,
            processed: Vec::new(),
            pages: PageExport::new(),
        }
    }

    pub fn resolve_type(
        &mut self,
        types: &Types,
        target_register: instructions::Registers,
        target_page: &u64,
    ) -> Vec<Instructions> {
        match types {
            Types::Collective(_) => todo!(),
            Types::Reference(_) => todo!(),
            Types::BraceReference(_) => todo!(),
            Types::Operator(operator) => match &operator.operator {
                operator::Operators::ComparisonType(_) => {
                    todo!()
                }
                operator::Operators::LogicalType(_) => todo!(),
                operator::Operators::ArithmeticType(e) => {
                    let mut instructions = Vec::new();
                    instructions.extend(self.resolve_type(
                        &operator.first,
                        instructions::Registers::B,
                        target_page,
                    ));

                    instructions.extend(self.resolve_type(
                        &operator.second,
                        instructions::Registers::C,
                        target_page,
                    ));

                    instructions.push(match e {
                        operator::ArithmeticOperators::Addition => {
                            instructions::Instructions::ADD(Instruction::implict())
                        }
                        operator::ArithmeticOperators::Subtraction => {
                            instructions::Instructions::SUB(Instruction::implict())
                        }
                        operator::ArithmeticOperators::Multiplication => {
                            instructions::Instructions::MUL(Instruction::implict())
                        }
                        operator::ArithmeticOperators::Exponentiation => {
                            instructions::Instructions::EXP(Instruction::implict())
                        }
                        operator::ArithmeticOperators::Division => {
                            instructions::Instructions::DIV(Instruction::implict())
                        }
                        operator::ArithmeticOperators::Modulus => {
                            instructions::Instructions::MOD(Instruction::implict())
                        }
                        operator::ArithmeticOperators::Null => unreachable!("Wrong operator"),
                    });
                    match target_register {
                        instructions::Registers::A => (),
                        instructions::Registers::B => {
                            instructions
                                .push(instructions::Instructions::LDB(Instruction::indirect_a()));
                        }
                        instructions::Registers::C => {
                            instructions
                                .push(instructions::Instructions::LDC(Instruction::indirect_a()));
                        }
                        instructions::Registers::X => {
                            instructions
                                .push(instructions::Instructions::LDX(Instruction::indirect_a()));
                        }
                        instructions::Registers::Y => {
                            instructions
                                .push(instructions::Instructions::LDY(Instruction::indirect_a()));
                        }
                    }
                    instructions
                }
                operator::Operators::AssignmentType(_) => todo!(),
                operator::Operators::Null => todo!(),
            },
            Types::Cloak(_) => todo!(),
            Types::Array(_) => todo!(),
            Types::Vector(_) => todo!(),
            Types::Function(_) => todo!(),
            Types::ClassCall(_) => {
                vec![]
            }
            Types::FunctionCall(e) => {
                let mut instructions = Vec::new();
                instructions.extend(self.resolve_type(
                    &e.target,
                    instructions::Registers::B,
                    target_page,
                ));
                instructions
            }
            Types::Void => todo!(),
            Types::NullResolver(_) => todo!(),
            Types::Negative(_) => todo!(),
            Types::VariableType(e) => {
                let locals = self.pages.find_page(*target_page).unwrap().clone();
                let pos = locals.find_local(&e.value).unwrap_or_else(|| {
                    panic!("?? {:#?}, \n\n {:#?}", locals.locals, e);
                });

                let mut instructions = Vec::new();

                match target_register {
                    instructions::Registers::A => {
                        match pos.reference {
                            Some(target_page) => {
                                instructions.push(instructions::Instructions::AOL(
                                    Instruction::absolute(target_page),
                                ));
                                instructions.push(instructions::Instructions::LDA(
                                    Instruction::absolute(pos.cursor),
                                ))
                            }
                            None => instructions.push(instructions::Instructions::LDA(
                                Instruction::absolute(pos.cursor),
                            )),
                        }
                    }
                    instructions::Registers::B => {
                        match pos.reference {
                            Some(target_page) => {
                                instructions.push(instructions::Instructions::AOL(
                                    Instruction::absolute(target_page),
                                ));
                                instructions.push(instructions::Instructions::LDB(
                                    Instruction::absolute(pos.cursor),
                                ))
                            }
                            None => instructions.push(instructions::Instructions::LDB(
                                Instruction::absolute(pos.cursor),
                            )),
                        }
                    }
                    instructions::Registers::C => {
                        match pos.reference {
                            Some(target_page) => {
                                instructions.push(instructions::Instructions::AOL(
                                    Instruction::absolute(target_page),
                                ));
                                instructions.push(instructions::Instructions::LDC(
                                    Instruction::absolute(pos.cursor),
                                ))
                            }
                            None => instructions.push(instructions::Instructions::LDC(
                                Instruction::absolute(pos.cursor),
                            )),
                        }
                    }
                    instructions::Registers::X => {
                        match pos.reference {
                            Some(target_page) => {
                                instructions.push(instructions::Instructions::AOL(
                                    Instruction::absolute(target_page),
                                ));
                                instructions.push(instructions::Instructions::LDX(
                                    Instruction::absolute(pos.cursor),
                                ))
                            }
                            None => instructions.push(instructions::Instructions::LDX(
                                Instruction::absolute(pos.cursor),
                            )),
                        }
                    }
                    instructions::Registers::Y => {
                        match pos.reference {
                            Some(target_page) => {
                                instructions.push(instructions::Instructions::AOL(
                                    Instruction::absolute(target_page),
                                ));
                                instructions.push(instructions::Instructions::LDY(
                                    Instruction::absolute(pos.cursor),
                                ))
                            }
                            None => instructions.push(instructions::Instructions::LDY(
                                Instruction::absolute(pos.cursor),
                            )),
                        }
                    }
                };

                instructions
            }
            Types::AsKeyword(_) => todo!(),
            Types::Byte(_) => todo!(),
            Types::Integer(int) => match target_register {
                instructions::Registers::A => {
                    vec![instructions::Instructions::LDA(Instruction::immediate(
                        int.value.to_le(),
                    ))]
                }
                instructions::Registers::B => {
                    vec![instructions::Instructions::LDB(Instruction::immediate(
                        int.value.to_le(),
                    ))]
                }
                instructions::Registers::C => {
                    vec![instructions::Instructions::LDC(Instruction::immediate(
                        int.value.to_le(),
                    ))]
                }
                instructions::Registers::X => {
                    vec![instructions::Instructions::LDX(Instruction::immediate(
                        int.value.to_le(),
                    ))]
                }
                instructions::Registers::Y => {
                    vec![instructions::Instructions::LDY(Instruction::immediate(
                        int.value.to_le(),
                    ))]
                }
            },
            Types::Float(_) => todo!(),
            Types::Double(_) => todo!(),
            Types::Bool(_) => todo!(),
            Types::String(_) => todo!(),
            Types::Char(_) => todo!(),
            Types::Null => vec![],
            Types::Dynamic => todo!(),
            Types::SetterCall(_) => todo!(),
        }
    }

    fn assemble_dependency(&mut self, hash: &u64, is_main: bool, locals: Option<Vec<LocalHeader>>) {
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

            for dependency in &processed_page.dependencies {
                self.assemble_dependency(&dependency.hash, false, None);
            }

            match locals.clone() {
                Some(e) => {
                    println!("\nPage created with locals {:?} = {:?}\n", e, hash);
                }
                None => (),
            }

            self.pages.push_page(InstructionPage {
                is_main,
                hash: processed_page.hash as usize,
                instructions: Vec::new(),
                debug_headers: Vec::new(),
                locals: match locals {
                    Some(e) => e,
                    None => Vec::new(),
                },
            });

            for item in &processed_page.items {
                match item {
                    ellie_core::definite::items::Collecting::Variable(variable) => {
                        let resolved_instructions =
                            self.resolve_type(&variable.value, instructions::Registers::A, hash);
                        let current_page = self.pages.find_page(*hash).unwrap();

                        current_page.debug_headers.push(DebugHeader {
                            id: current_page.instructions.len(),
                            rtype: DebugHeaderType::Variable,
                            name: variable.name.clone(),
                            cursor: variable.pos,
                        });

                        current_page.extend_instructions(resolved_instructions);
                        current_page.assign_instruction(instructions::Instructions::STA(
                            Instruction::implict(),
                        ));
                        current_page.locals.push(LocalHeader {
                            name: variable.name.clone(),
                            cursor: current_page.instructions.len(),
                            reference: None,
                        })
                    }
                    ellie_core::definite::items::Collecting::Function(function) => {
                        for dependency in &processed_page.dependencies {
                            self.assemble_dependency(&dependency.hash, false, None);
                        }
                        let current_page = self.pages.find_page(*hash).unwrap();

                        current_page.locals.push(LocalHeader {
                            name: function.name.clone(),
                            cursor: current_page.instructions.len(),
                            reference: Some(function.inner_page_id as usize),
                        });
                        println!("Register inner page_id : {}", function.inner_page_id);
                        let current_locals = current_page.locals.clone();
                        self.assemble_dependency(
                            &function.inner_page_id,
                            false,
                            Some(
                                current_locals
                                    .clone()
                                    .iter()
                                    .map(|x| LocalHeader {
                                        name: x.name.clone(),
                                        cursor: x.cursor,
                                        reference: Some(*hash as usize),
                                    })
                                    .collect::<Vec<_>>(),
                            ),
                        );
                    }
                    ellie_core::definite::items::Collecting::ForLoop(_) => {
                        std::println!("[Assembler,Ignore,Element] ForLoop")
                    }
                    ellie_core::definite::items::Collecting::Condition(_) => {
                        std::println!("[Assembler,Ignore,Element] Condition")
                    }
                    ellie_core::definite::items::Collecting::Class(class) => {
                        for dependency in &processed_page.dependencies {
                            self.assemble_dependency(&dependency.hash, false, None);
                        }
                        let current_page = self.pages.find_page(*hash).unwrap();
                        current_page.locals.push(LocalHeader {
                            name: class.name.clone(),
                            cursor: current_page.instructions.len(),
                            reference: Some(class.inner_page_id as usize),
                        });
                        let current_locals = current_page.locals.clone();
                        self.assemble_dependency(
                            &class.inner_page_id,
                            false,
                            Some(
                                current_locals
                                    .clone()
                                    .iter()
                                    .map(|x| LocalHeader {
                                        name: x.name.clone(),
                                        cursor: x.cursor,
                                        reference: Some(*hash as usize),
                                    })
                                    .collect::<Vec<_>>(),
                            ),
                        );
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
                    ellie_core::definite::items::Collecting::GetterCall(getter) => {
                        match &getter.data {
                            Types::Collective(_) => todo!(),
                            Types::Reference(_) => todo!(),
                            Types::BraceReference(_) => todo!(),
                            Types::Operator(_) => todo!(),
                            Types::Cloak(_) => todo!(),
                            Types::Array(_) => todo!(),
                            Types::Vector(_) => todo!(),
                            Types::Function(_) => todo!(),
                            Types::ClassCall(_) => todo!(),
                            Types::FunctionCall(function_call) => {
                                let current_page = self.pages.find_page(*hash).unwrap();
                                let target_local = match *function_call.target.clone() {
                                    Types::VariableType(e) => e.value,
                                    _ => unreachable!(),
                                };
                                let target = current_page.find_local(&target_local);

                                //instructions.extend(resolved_instructions);
                                current_page.instructions.extend(vec![
                                    instructions::Instructions::CALL(Instruction::absolute(
                                        target.unwrap().reference.unwrap(),
                                    )),
                                ]);
                            }
                            Types::SetterCall(_) => todo!(),
                            Types::NullResolver(_) => todo!(),
                            Types::Negative(_) => todo!(),
                            Types::VariableType(_) => todo!(),
                            Types::AsKeyword(_) => todo!(),
                            Types::Null => todo!(),
                            Types::Dynamic => todo!(),
                            _ => {
                                let resolved_instructions = self.resolve_type(
                                    &getter.data,
                                    instructions::Registers::A,
                                    hash,
                                );
                                todo!("GetterCall: {:?}", resolved_instructions);
                            }
                            Types::Byte(_) => todo!(),
                            Types::Integer(_) => todo!(),
                            Types::Float(_) => todo!(),
                            Types::Double(_) => todo!(),
                            Types::Bool(_) => todo!(),
                            Types::String(_) => todo!(),
                            Types::Char(_) => todo!(),
                            Types::Void => todo!(),
                        }
                    }
                    ellie_core::definite::items::Collecting::SetterCall(_) => todo!(),
                    ellie_core::definite::items::Collecting::Enum(_) => todo!(),
                    ellie_core::definite::items::Collecting::NativeFunction(_) => {
                        std::println!("[Assembler,Ignore,Element] NativeFunction")
                    }
                    ellie_core::definite::items::Collecting::None => todo!(),
                    ellie_core::definite::items::Collecting::Brk(_) => todo!(),
                    ellie_core::definite::items::Collecting::Go(_) => todo!(),
                    ellie_core::definite::items::Collecting::FuctionParameter(
                        function_parameter,
                    ) => {
                        let current_page = self.pages.find_page(*hash).unwrap();
                        current_page.debug_headers.push(DebugHeader {
                            id: current_page.instructions.len(),
                            rtype: DebugHeaderType::Variable,
                            name: function_parameter.name.clone(),
                            cursor: Cursor {
                                range_start: function_parameter.name_pos.range_start,
                                range_end: function_parameter.rtype_pos.range_end,
                            },
                        });

                        current_page.assign_instruction(instructions::Instructions::STA(
                            Instruction::implict(),
                        ));

                        current_page.locals.push(LocalHeader {
                            name: function_parameter.name.clone(),
                            cursor: current_page.instructions.len(),
                            reference: None,
                        });
                    }
                    ellie_core::definite::items::Collecting::ConstructorParameter(_) => {
                        std::println!("[Assembler,Ignore,Element] ConstructorParameter")
                    }
                    ellie_core::definite::items::Collecting::SelfItem(_) => {
                        std::println!("[Assembler,Ignore,Element] SelfItem")
                    }
                }
            }
        }
    }

    pub fn assemble(&mut self) -> AssembleResult {
        self.assemble_dependency(&self.module.initial_page.clone(), true, None);
        AssembleResult {
            pages: self.pages.clone().pages,
            platform_attributes: self.platform_attributes.clone(),
        }
    }
}
