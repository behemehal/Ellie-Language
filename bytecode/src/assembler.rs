use crate::instructions::{self, Instruction, Instructions};
use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use ellie_core::{
    definite::types::{operator, Types},
    defs::Cursor,
    utils::{ExportPage, PageExport},
};
use ellie_parser::parser::Module;
use std::{io::Write, panic, println};

pub struct Assembler {
    module: Module,
    processed: Vec<u64>,
    platform_attributes: PlatformAttributes,
    instructions: Vec<instructions::Instructions>,
    locals: Vec<LocalHeader>,
    debug_headers: Vec<DebugHeader>,
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
    B16,
    B32,
    B64,
}

impl PlatformArchitecture {
    pub fn get_code(&self) -> u8 {
        match self {
            PlatformArchitecture::B16 => 16,
            PlatformArchitecture::B32 => 32,
            PlatformArchitecture::B64 => 64,
        }
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
                operator::Operators::ComparisonType(e) => {
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
                        operator::ComparisonOperators::Equal => {
                            instructions::Instructions::EQ(Instruction::implict())
                        }
                        operator::ComparisonOperators::NotEqual => {
                            instructions::Instructions::NE(Instruction::implict())
                        }
                        operator::ComparisonOperators::GreaterThan => {
                            instructions::Instructions::GT(Instruction::implict())
                        }
                        operator::ComparisonOperators::LessThan => {
                            instructions::Instructions::LT(Instruction::implict())
                        }
                        operator::ComparisonOperators::GreaterThanOrEqual => {
                            instructions::Instructions::GQ(Instruction::implict())
                        }
                        operator::ComparisonOperators::LessThanOrEqual => {
                            instructions::Instructions::LQ(Instruction::implict())
                        }
                        operator::ComparisonOperators::Null => unreachable!(),
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
                let pos = self.find_local(&e.value).unwrap();

                let mut instructions = Vec::new();

                match target_register {
                    instructions::Registers::A => match pos.reference {
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
                    },
                    instructions::Registers::B => match pos.reference {
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
                    },
                    instructions::Registers::C => match pos.reference {
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
                    },
                    instructions::Registers::X => match pos.reference {
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
                    },
                    instructions::Registers::Y => match pos.reference {
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
                    },
                };

                instructions
            }
            Types::AsKeyword(_) => todo!(),
            Types::Byte(_) => todo!(),
            Types::Integer(int) => match target_register {
                instructions::Registers::A => {
                    vec![instructions::Instructions::LDA(Instruction::immediate(
                        crate::instructions::Types::Integer,
                        int.value.to_le(),
                    ))]
                }
                instructions::Registers::B => {
                    vec![instructions::Instructions::LDB(Instruction::immediate(
                        crate::instructions::Types::Integer,
                        int.value.to_le(),
                    ))]
                }
                instructions::Registers::C => {
                    vec![instructions::Instructions::LDC(Instruction::immediate(
                        crate::instructions::Types::Integer,
                        int.value.to_le(),
                    ))]
                }
                instructions::Registers::X => {
                    vec![instructions::Instructions::LDX(Instruction::immediate(
                        crate::instructions::Types::Integer,
                        int.value.to_le(),
                    ))]
                }
                instructions::Registers::Y => {
                    vec![instructions::Instructions::LDY(Instruction::immediate(
                        crate::instructions::Types::Integer,
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

    fn assemble_dependency(&mut self, hash: &u64, is_main: bool) -> Option<usize> {
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
            self.assemble_dependency(&dependency.hash, false);
        }

        let mut main_pos = None;

        for item in &processed_page.items {
            match item {
                ellie_core::definite::items::Collecting::Variable(variable) => {
                    let resolved_instructions =
                        self.resolve_type(&variable.value, instructions::Registers::A, hash);

                    //current_page.debug_headers.push(DebugHeader {
                    //    id: current_page.instructions.len() - 1,
                    //    rtype: DebugHeaderType::Variable,
                    //    name: variable.name.clone(),
                    //    cursor: variable.pos,
                    //});

                    self.instructions.extend(resolved_instructions);
                    self.instructions
                        .push(instructions::Instructions::STA(Instruction::implict()));
                    self.locals.push(LocalHeader {
                        name: variable.name.clone(),
                        cursor: self.instructions.len() - 1,
                        reference: None,
                    })
                }
                ellie_core::definite::items::Collecting::Function(function) => {
                    for dependency in &processed_page.dependencies {
                        self.assemble_dependency(&dependency.hash, false);
                    }

                    self.locals.push(LocalHeader {
                        name: function.name.clone(),
                        cursor: self.instructions.len(),
                        reference: Some(function.inner_page_id as usize),
                    });

                    if function.name == "main" {
                        main_pos = Some(self.instructions.len());
                    }

                    self.assemble_dependency(&function.inner_page_id, false);

                    self.instructions
                        .push(instructions::Instructions::RET(Instruction::implict()));
                }
                ellie_core::definite::items::Collecting::ForLoop(_) => {
                    std::println!("[Assembler,Ignore,Element] ForLoop")
                }
                ellie_core::definite::items::Collecting::Condition(condition) => {
                    let has_ret = condition.returns.is_some();
                    let mut data_cursor = 0;
                    {
                        self.instructions.push(instructions::Instructions::LDA(
                            Instruction::immediate(crate::instructions::Types::Void, 0),
                        ));
                        self.instructions
                            .push(instructions::Instructions::STA(Instruction::implict()));
                        //Register a ret point
                        self.locals.push(LocalHeader {
                            name: "$0".to_string(),
                            cursor: self.instructions.len() - 1,
                            reference: None,
                        });
                        data_cursor = self.instructions.len() - 1;
                    }

                    for chain in &condition.chains {
                        if chain.rtype
                            != ellie_core::definite::items::condition::ConditionType::Else
                        {
                            let resolved_instructions = self.resolve_type(
                                &chain.condition,
                                instructions::Registers::A,
                                hash,
                            );
                            self.instructions.extend(resolved_instructions);
                        }
                        self.instructions.push(instructions::Instructions::JMPA(
                            Instruction::absolute(01234),
                        ));
                        self.assemble_dependency(&chain.inner_page_id, false);
                    }
                    let mut ret_location = 0;
                    {
                        self.instructions.push(instructions::Instructions::RET(
                            Instruction::absolute(data_cursor),
                        ));
                        ret_location = self.instructions.len() - 1;
                    }
                    for chain in &condition.chains {
                        self.instructions.push(instructions::Instructions::ACP(
                            Instruction::absolute(ret_location),
                        ));
                        self.instructions.push(instructions::Instructions::JMP(
                            Instruction::absolute(*hash as usize),
                        ));
                    }
                }
                ellie_core::definite::items::Collecting::Class(class) => {
                    for dependency in &processed_page.dependencies {
                        self.assemble_dependency(&dependency.hash, false);
                    }
                    self.assemble_dependency(&class.inner_page_id, false);
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
                ellie_core::definite::items::Collecting::GetterCall(getter) => match &getter.data {
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
                        let target_local = match *function_call.target.clone() {
                            Types::VariableType(e) => e.value,
                            _ => unreachable!(),
                        };

                        let target = self.find_local(&target_local).unwrap().clone();

                        self.instructions.push(instructions::Instructions::CALL(
                            Instruction::absolute(target.cursor),
                        ));
                    }
                    Types::SetterCall(_) => todo!(),
                    Types::NullResolver(_) => todo!(),
                    Types::Negative(_) => todo!(),
                    Types::VariableType(_) => todo!(),
                    Types::AsKeyword(_) => todo!(),
                    Types::Null => todo!(),
                    Types::Dynamic => todo!(),
                    _ => {
                        let resolved_instructions =
                            self.resolve_type(&getter.data, instructions::Registers::A, hash);
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
                },
                ellie_core::definite::items::Collecting::SetterCall(setter) => {
                    match &setter.target {
                        Types::Reference(_) => todo!(),
                        Types::BraceReference(_) => todo!(),
                        Types::VariableType(e) => {
                            let mut instructions = Vec::new();
                            let resolved_instructions =
                                self.resolve_type(&setter.value, instructions::Registers::A, hash);

                            let target = self.find_local(&e.value).unwrap();

                            instructions.extend(resolved_instructions);
                            if let Some(reference) = target.reference {
                                instructions.push(instructions::Instructions::AOL(
                                    Instruction::absolute(reference),
                                ));
                            }

                            instructions.push(instructions::Instructions::STA(
                                Instruction::absolute(target.cursor),
                            ));
                            self.instructions.extend(instructions)
                        }
                        _ => unreachable!("Invalid left-side of assignment"),
                    }
                }
                ellie_core::definite::items::Collecting::Enum(_) => todo!(),
                ellie_core::definite::items::Collecting::NativeFunction(e) => {
                    std::println!("[Assembler,Ignore,Element] NativeFunction: {:?}", e)
                }
                ellie_core::definite::items::Collecting::None => todo!(),
                ellie_core::definite::items::Collecting::Brk(_) => todo!(),
                ellie_core::definite::items::Collecting::Go(_) => todo!(),
                ellie_core::definite::items::Collecting::FuctionParameter(function_parameter) => {
                    self.debug_headers.push(DebugHeader {
                        id: if self.instructions.len() == 0 {
                            0
                        } else {
                            self.instructions.len() - 1
                        },
                        rtype: DebugHeaderType::Variable,
                        name: function_parameter.name.clone(),
                        cursor: Cursor {
                            range_start: function_parameter.name_pos.range_start,
                            range_end: function_parameter.rtype_pos.range_end,
                        },
                    });

                    self.instructions
                        .push(instructions::Instructions::STA(Instruction::implict()));

                    self.locals.push(LocalHeader {
                        name: function_parameter.name.clone(),
                        cursor: self.instructions.len() - 1,
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
        let main_function = self.assemble_dependency(&self.module.initial_page.clone(), true);
        AssembleResult {
            instructions: self.instructions.clone(),
            platform_attributes: self.platform_attributes.clone(),
            main_function,
        }
    }
}
