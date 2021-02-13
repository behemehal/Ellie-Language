//TODO: no-std use crate::alloc::string::{String, ToString};
//TODO: no-std use crate::alloc::vec::Vec;

#![allow(warnings)] //TODO Remove this

pub mod defs;

use crate::error;
use crate::mapper;
use crate::processors;
use crate::syntax::{function, types, variable};
use crate::utils;

#[derive(Debug, Clone, PartialEq)]
pub struct Mapped {
    pub items: Vec<Collecting>,
    pub syntax_errors: Vec<error::Error>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Collecting {
    Variable(variable::VariableCollector),
    Function(function::FunctionCollector),
    None,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Mapper {
    pub code: String,
    pub options: defs::MapperOptions,
    pub collected: Vec<Collecting>,
    pub pos: defs::CursorPosition,
    pub ignore_line: bool,
    pub current: Collecting,
}

impl Mapper {
    pub fn new(code: String, options: defs::MapperOptions) -> Self {
        Mapper {
            code,
            options,
            collected: Vec::new(),
            pos: defs::CursorPosition(0, 0),
            ignore_line: false,
            current: Collecting::None,
        }
    }
    pub fn Map(&mut self) -> Mapped {
        let mut errors: Vec<error::Error> = Vec::new();

        pub fn cursorWriteDebug(letter_char: String, row: i64, column: i64) {
            //println!(
            //    "Scaned | char: {:?}, {}:{} |",
            //    letter_char,
            //    row,
            //    column
            //);
        }

        'charLoop: for (index, char) in self.code.chars().enumerate() {
            let letter_char = &char.to_string();
            let last_char = &utils::get_letter(self.code.to_string(), index, false).to_owned();
            let next_char = &utils::get_letter(self.code.to_string(), index, true).to_owned();

            let next_next_char =
                &utils::get_letter(self.code.to_string(), index + 1, true).to_owned();
            if char != '\n' && char != '\r' && char != '\t' {
                if letter_char == "\t" {
                    println!("TAB");
                } else if (letter_char == "d" && next_char == " ")
                    && self.current == Collecting::None
                {
                    println!("Capture Dynamic Variable");
                    self.current = Collecting::Variable(variable::VariableCollector {
                        initialized: true,
                        dynamic: true,
                        pos: defs::Cursor {
                            range_start: self.pos,
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                    continue 'charLoop;
                } else if (letter_char == "v" && next_char == " ")
                    && self.current == Collecting::None
                {
                    self.current = Collecting::Variable(variable::VariableCollector {
                        initialized: true,
                        pos: defs::Cursor {
                            range_start: self.pos,
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                    continue 'charLoop;
                } else if (letter_char == "f"
                    && next_char == "n"
                    && next_next_char == " "
                    && self.current == Collecting::None)
                {
                    self.current = Collecting::Function(function::FunctionCollector::default());
                    continue 'charLoop;
                } else {
                    //println!("{} {} {}", letter_char, next_char, next_next_char);
                }
                match &mut self.current {
                    Collecting::Variable(data) => {
                        if !data.named {
                            if letter_char == ":" {
                                if data.name == "" {
                                    errors.push(error::Error {
                                        debug_message: "Redaktik".to_string(),
                                        title: error::errorList::error_s1.title.clone(),
                                        code: error::errorList::error_s1.code,
                                        message: error::errorList::error_s1.message.clone(),
                                        builded_message: error::Error::build(
                                            error::errorList::error_s1.message.clone(),
                                            vec![error::ErrorBuildField {
                                                key: "token".to_string(),
                                                value: letter_char.to_string(),
                                            }],
                                        ),
                                        pos: mapper::defs::Cursor {
                                            range_start: self.pos.clone(),
                                            range_end: self.pos.clone().skipChar(1),
                                        },
                                    });
                                } else {
                                    if data.dynamic {//TODO REMOVE THIS
                                        errors.push(error::Error {
                                            debug_message: "Ethicl".to_string(),
                                            title: error::errorList::error_s11.title.clone(),
                                            code: error::errorList::error_s11.code,
                                            message: error::errorList::error_s11.message.clone(),
                                            builded_message: error::errorList::error_s11.message.clone(),
                                            pos: mapper::defs::Cursor {
                                                range_start: self.pos.clone().skipChar(1),
                                                range_end: self.pos.clone().skipChar(2)
                                            },
                                        });
                                    }
                                    data.named = true;
                                    //println!("|DEBUG| VARIABLE NAMED: {}", data.name);
                                }
                            } else if letter_char == ";" && data.dynamic {
                                self.collected.push(self.current.clone());
                                self.current = Collecting::None;
                            } else if letter_char == "=" {
                                if !data.dynamic {
                                    errors.push(error::Error {
                                        debug_message: "Ertsalik".to_string(),
                                        title: error::errorList::error_s8.title.clone(),
                                        code: error::errorList::error_s8.code,
                                        message: error::errorList::error_s8.message.clone(),
                                        builded_message: error::errorList::error_s8.message.clone(),
                                        pos: mapper::defs::Cursor {
                                            range_start: self.pos.clone(),
                                            range_end: self.pos.clone().skipChar(1),
                                        },
                                    });
                                } else if data.name == "" {
                                    errors.push(error::Error {
                                        debug_message: "\\src\\mapper\\mod.rs:133:0".to_string(),
                                        title: error::errorList::error_s1.title.clone(),
                                        code: error::errorList::error_s1.code,
                                        message: error::errorList::error_s1.message.clone(),
                                        builded_message: error::Error::build(
                                            error::errorList::error_s1.message.clone(),
                                            vec![error::ErrorBuildField {
                                                key: "token".to_string(),
                                                value: letter_char.to_string(),
                                            }],
                                        ),
                                        pos: mapper::defs::Cursor {
                                            range_start: self.pos.clone(),
                                            range_end: self.pos.clone().skipChar(1),
                                        },
                                    });
                                } else {
                                    data.named = true;
                                    println!("|DEBUG| VARIABLE NAMED: {}", data.name);
                                }
                            } else {
                                let currentReliability = crate::utils::reliable_name_range(
                                    crate::utils::ReliableNameRanges::VariableName,
                                    letter_char.clone(),
                                );
                                if currentReliability.reliable {
                                    if last_char == " " && data.name != "" {
                                        errors.push(error::Error {
                                            debug_message: "\\src\\mapper\\mod.rs:161:0"
                                                .to_string(),
                                            title: error::errorList::error_s1.title.clone(),
                                            code: error::errorList::error_s1.code,
                                            message: error::errorList::error_s1.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s1.message.clone(),
                                                vec![error::ErrorBuildField {
                                                    key: "token".to_string(),
                                                    value: letter_char.to_string(),
                                                }],
                                            ),
                                            pos: mapper::defs::Cursor {
                                                range_start: self.pos.clone(),
                                                range_end: self.pos.clone().skipChar(1),
                                            },
                                        });
                                    } else {
                                        data.name = data.name.clone() + &letter_char.clone();
                                    }
                                } else if letter_char != " "
                                    && (letter_char != ":"
                                        || letter_char != "="
                                        || letter_char != ";")
                                    && (last_char == " " || data.name != "")
                                {
                                    errors.push(error::Error {
                                        debug_message: "Ahencam".to_string(),
                                        title: error::errorList::error_s1.title.clone(),
                                        code: error::errorList::error_s1.code,
                                        message: error::errorList::error_s1.message.clone(),
                                        builded_message: error::Error::build(
                                            error::errorList::error_s1.message.clone(),
                                            vec![error::ErrorBuildField {
                                                key: "token".to_string(),
                                                value: letter_char.to_string(),
                                            }],
                                        ),
                                        pos: mapper::defs::Cursor {
                                            range_start: self.pos.clone(),
                                            range_end: self.pos.clone().skipChar(1),
                                        },
                                    });
                                }
                            }
                        } else if !data.typed && !data.dynamic {
                            if letter_char == ";" {
                                if (data.dynamic) {}
                                self.collected.push(self.current.clone());
                                self.current = Collecting::None;
                            } else if letter_char == "=" {
                                if data.r#type == "" {
                                    errors.push(error::Error {
                                        debug_message: "Odio".to_string(),
                                        title: error::errorList::error_s1.title.clone(),
                                        code: error::errorList::error_s1.code,
                                        message: error::errorList::error_s1.message.clone(),
                                        builded_message: error::Error::build(
                                            error::errorList::error_s1.message.clone(),
                                            vec![error::ErrorBuildField {
                                                key: "token".to_string(),
                                                value: letter_char.to_string(),
                                            }],
                                        ),
                                        pos: mapper::defs::Cursor {
                                            range_start: self.pos.clone(),
                                            range_end: self.pos.clone().skipChar(1),
                                        },
                                    });
                                } else {
                                    data.typed = true;
                                }
                            } else {
                                let currentReliability = crate::utils::reliable_name_range(
                                    crate::utils::ReliableNameRanges::VariableName,
                                    letter_char.clone(),
                                );
                                if currentReliability.reliable && (data.r#type == "" || last_char != " ") {
                                    data.r#type = data.r#type.clone() + &letter_char.clone();
                                } else if data.r#type != "" && (last_char == " " && (letter_char != ":" && letter_char != " ")) {
                                    errors.push(error::Error {
                                        debug_message: "ThnicLimts".to_string(),
                                        title: error::errorList::error_s1.title.clone(),
                                        code: error::errorList::error_s1.code,
                                        message: error::errorList::error_s1.message.clone(),
                                        builded_message: error::Error::build(
                                            error::errorList::error_s1.message.clone(),
                                            vec![error::ErrorBuildField {
                                                key: "token".to_string(),
                                                value: letter_char.to_string(),
                                            }],
                                        ),
                                        pos: mapper::defs::Cursor {
                                            range_start: self.pos.clone(),
                                            range_end: self.pos.clone().skipChar(1),
                                        },
                                    });
                                }
                            }
                        } else {
                            let mut collecter_response: bool = false;
                            if letter_char == ";" {
                                if let Collecting::Variable(collected) = self.current.clone() {
                                    println!("{:#?}", collected.value);
                                    if collected.value.is_complete() {
                                        self.collected.push(self.current.clone());
                                        self.current = Collecting::None;
                                    } else {
                                        errors.push(error::Error {
                                            debug_message: "Protocol".to_string(),
                                            title: error::errorList::error_s1.title.clone(),
                                            code: error::errorList::error_s1.code,
                                            message: error::errorList::error_s1.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s1.message.clone(),
                                                vec![error::ErrorBuildField {
                                                    key: "token".to_string(),
                                                    value: letter_char.to_string(),
                                                }],
                                            ),
                                            pos: mapper::defs::Cursor {
                                                range_start: self.pos.clone().skipChar(1),
                                                range_end: self.pos.clone().skipChar(2),
                                            },
                                        });
                                    }
                                }
                                
                            } else {
                                let mut cd = data.clone();
                                let collected = processors::value_processor::collect(
                                    &mut cd,
                                    letter_char,
                                    next_char.to_string(),
                                    last_char.to_string(),
                                    self.pos.clone(),
                                );
                                for i in collected.errors {
                                    errors.push(i)
                                }
                                self.current = Collecting::Variable(collected.itered_data);
                            }
                        }
                    }
                    Collecting::Function(data) => {
                        if !data.initialized {
                            if last_char == " " && letter_char != " " {
                                data.initialized = true;
                                data.name_pos.range_start.0 = self.pos.0; //Function naming started so we set the position
                                data.name_pos.range_start.1 = self.pos.1; //Function naming started so we set the position
                                data.name += letter_char;
                                continue 'charLoop;
                            }
                        } else if !data.named {
                            if letter_char == "(" {
                                data.name_pos.range_end.0 = self.pos.0; // function naming ended
                                data.name_pos.range_end.1 = self.pos.1; // function naming ended
                                data.parameter_bracket_start_pos.range_start.0 = self.pos.0; //parameter start
                                data.parameter_bracket_start_pos.range_start.1 = self.pos.1; //parameter start
                                data.parameter_bracket_start_pos.range_end.0 =
                                    self.pos.skipChar(1).0; //parameter start
                                data.parameter_bracket_start_pos.range_end.1 =
                                    self.pos.skipChar(1).1; //parameter start
                                data.named = true;
                            } else if last_char == " " && letter_char != " " && data.name != "" {
                                errors.push(error::Error {
                                    debug_message: "Entropy".to_string(),
                                    title: error::errorList::error_s1.title.clone(),
                                    code: error::errorList::error_s1.code,
                                    message: error::errorList::error_s1.message.clone(),
                                    builded_message: error::Error::build(
                                        error::errorList::error_s1.message.clone(),
                                        vec![error::ErrorBuildField {
                                            key: "token".to_string(),
                                            value: letter_char.to_string(),
                                        }],
                                    ),
                                    pos: mapper::defs::Cursor {
                                        range_start: self.pos.clone(),
                                        range_end: self.pos.clone().skipChar(1),
                                    },
                                });
                            } else {
                                let currentReliability = crate::utils::reliable_name_range(
                                    crate::utils::ReliableNameRanges::VariableName,
                                    letter_char.clone(),
                                );
                                if currentReliability.reliable {
                                    if last_char == " " && data.name != "" {
                                        errors.push(error::Error {
                                            debug_message: "Physicist"
                                                .to_string(),
                                            title: error::errorList::error_s1.title.clone(),
                                            code: error::errorList::error_s1.code,
                                            message: error::errorList::error_s1.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s1.message.clone(),
                                                vec![error::ErrorBuildField {
                                                    key: "token".to_string(),
                                                    value: letter_char.to_string(),
                                                }],
                                            ),
                                            pos: mapper::defs::Cursor {
                                                range_start: self.pos.clone(),
                                                range_end: self.pos.clone().skipChar(1),
                                            },
                                        });
                                    } else {
                                        data.name += letter_char;
                                    }
                                } else if letter_char != " " {
                                    errors.push(error::Error {
                                        debug_message: "AntiLine".to_string(),
                                        title: error::errorList::error_s1.title.clone(),
                                        code: error::errorList::error_s1.code,
                                        message: error::errorList::error_s1.message.clone(),
                                        builded_message: error::Error::build(
                                            error::errorList::error_s1.message.clone(),
                                            vec![error::ErrorBuildField {
                                                key: "token".to_string(),
                                                value: letter_char.to_string(),
                                            }],
                                        ),
                                        pos: mapper::defs::Cursor {
                                            range_start: self.pos.clone(),
                                            range_end: self.pos.clone().skipChar(1),
                                        },
                                    });
                                }
                                //user naming the function
                            }
                        } else if !data.parameter_wrote {
                            let mut last_entry = data.parameters.len();
                            let typing_name = if last_entry == 0 {
                                true
                            } else {
                                !data.parameters[last_entry - 1].named
                            };

                            let colon_expected = if last_entry == 0 {
                                false
                            } else {
                                data.parameters[last_entry - 1].colon_expected
                            };

                            let typing = if last_entry == 0 {
                                false
                            } else {
                                if data.parameters[last_entry - 1].named
                                    && !data.parameters[last_entry - 1].colon_expected
                                {
                                    true
                                } else {
                                    false
                                }
                            };

                            let current_reliability = crate::utils::reliable_name_range(
                                crate::utils::ReliableNameRanges::VariableName,
                                letter_char.to_string(),
                            );

                            if typing_name {
                                if current_reliability.reliable {
                                    if last_entry == 0 {
                                        data.parameters.push(function::FunctionParameter {
                                            name: letter_char.to_string(),
                                            named: false,
                                            name_pos: defs::Cursor {
                                                range_start: self.pos,
                                                range_end: defs::CursorPosition::default(),
                                            },
                                            colon_expected: false,
                                            r#type: types::Types::Null,
                                            type_text: String::new(),
                                            typed: false,
                                            type_pos: defs::Cursor::default(),
                                        })
                                    } else {
                                        if last_char == " "
                                            && data.parameters[last_entry - 1].name != ""
                                        {
                                            errors.push(error::Error {
                                                debug_message: "Irenar"
                                                    .to_string(),
                                                title: error::errorList::error_s1.title.clone(),
                                                code: error::errorList::error_s1.code,
                                                message: error::errorList::error_s1.message.clone(),
                                                builded_message: error::Error::build(
                                                    error::errorList::error_s1.message.clone(),
                                                    vec![error::ErrorBuildField {
                                                        key: "token".to_string(),
                                                        value: letter_char.to_string(),
                                                    }],
                                                ),
                                                pos: mapper::defs::Cursor {
                                                    range_start: self.pos.clone(),
                                                    range_end: self.pos.clone().skipChar(1),
                                                },
                                            });
                                        } else {
                                            if data.parameters[last_entry - 1].name == "" {
                                                data.parameters[last_entry - 1]
                                                    .name_pos
                                                    .range_start = self.pos;
                                            }
                                            data.parameters[last_entry - 1].name += letter_char;
                                            data.parameters[last_entry - 1].name_pos.range_end =
                                                self.pos;
                                        }
                                    }
                                } else {
                                    if letter_char == ":" {
                                        data.parameters[last_entry - 1].named = true;
                                    } else if letter_char == " " {
                                        if last_entry != 0 {
                                            if data.parameters[last_entry - 1].name != "" {
                                                data.parameters[last_entry - 1].named = true;
                                                data.parameters[last_entry - 1].colon_expected =
                                                    true;
                                            }
                                        }
                                    } else {
                                        errors.push(error::Error {
                                            debug_message: "Quadro"
                                                .to_string(),
                                            title: error::errorList::error_s1.title.clone(),
                                            code: error::errorList::error_s1.code,
                                            message: error::errorList::error_s1.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s1.message.clone(),
                                                vec![error::ErrorBuildField {
                                                    key: "token".to_string(),
                                                    value: letter_char.to_string(),
                                                }],
                                            ),
                                            pos: mapper::defs::Cursor {
                                                range_start: self.pos.clone(),
                                                range_end: self.pos.clone().skipChar(1),
                                            },
                                        });
                                    }
                                }
                            } else if colon_expected {
                                if letter_char == ":" {
                                    data.parameters[last_entry - 1].colon_expected = false;
                                } else if letter_char != " " {
                                    errors.push(error::Error {
                                        debug_message: "Nucleic".to_string(),
                                        title: error::errorList::error_s1.title.clone(),
                                        code: error::errorList::error_s1.code,
                                        message: error::errorList::error_s1.message.clone(),
                                        builded_message: error::Error::build(
                                            error::errorList::error_s1.message.clone(),
                                            vec![error::ErrorBuildField {
                                                key: "token".to_string(),
                                                value: letter_char.to_string(),
                                            }],
                                        ),
                                        pos: mapper::defs::Cursor {
                                            range_start: self.pos.clone(),
                                            range_end: self.pos.clone().skipChar(1),
                                        },
                                    });
                                }
                            } else if typing {
                                if current_reliability.reliable {
                                    if last_char == " "
                                        && data.parameters[last_entry - 1].type_text != ""
                                    {
                                        errors.push(error::Error {
                                            debug_message: "Estetik"
                                                .to_string(),
                                            title: error::errorList::error_s1.title.clone(),
                                            code: error::errorList::error_s1.code,
                                            message: error::errorList::error_s1.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s1.message.clone(),
                                                vec![error::ErrorBuildField {
                                                    key: "token".to_string(),
                                                    value: letter_char.to_string(),
                                                }],
                                            ),
                                            pos: mapper::defs::Cursor {
                                                range_start: self.pos.clone(),
                                                range_end: self.pos.clone().skipChar(1),
                                            },
                                        });
                                    } else if data.parameters[last_entry - 1].type_text == "" {
                                        data.parameters[last_entry - 1].type_pos.range_start =
                                            self.pos;
                                    }
                                    data.parameters[last_entry - 1].type_pos.range_end = self.pos;
                                    data.parameters[last_entry - 1].type_text += letter_char;
                                } else if letter_char == "," {
                                    let parameter_names = data
                                        .parameters
                                        .iter()
                                        .map(|x| x.name.clone())
                                        .collect::<Vec<String>>();
                                    let mut parameter_names_deduped = data
                                        .parameters
                                        .iter()
                                        .map(|x| x.name.clone())
                                        .collect::<Vec<String>>();
                                    parameter_names_deduped.dedup();

                                    if parameter_names.len() != parameter_names_deduped.len() {
                                        errors.push(error::Error {
                                            debug_message: "Artiyik"
                                                .to_string(),
                                            title: error::errorList::error_s10.title.clone(),
                                            code: error::errorList::error_s10.code,
                                            message: error::errorList::error_s10.message.clone(),
                                            builded_message: error::errorList::error_s10
                                                .message
                                                .clone(),
                                            pos: mapper::defs::Cursor {
                                                range_start: data.parameters
                                                    [data.parameters.len() - 1]
                                                    .name_pos
                                                    .range_start,
                                                range_end: data.parameters
                                                    [data.parameters.len() - 1]
                                                    .type_pos
                                                    .range_end,
                                            },
                                        });
                                    }

                                    data.parameters.push(function::FunctionParameter {
                                        name: String::new(),
                                        named: false,
                                        name_pos: defs::Cursor::default(),
                                        colon_expected: false,
                                        r#type: types::Types::Null,
                                        type_text: String::new(),
                                        typed: false,
                                        type_pos: defs::Cursor::default(),
                                    });
                                } else if letter_char == ")" {
                                    let parameter_names = data
                                        .parameters
                                        .iter()
                                        .map(|x| x.name.clone())
                                        .collect::<Vec<String>>();
                                    let mut parameter_names_deduped = data
                                        .parameters
                                        .iter()
                                        .map(|x| x.name.clone())
                                        .collect::<Vec<String>>();
                                    parameter_names_deduped.dedup();

                                    if parameter_names.len() != parameter_names_deduped.len() {
                                        errors.push(error::Error {
                                            debug_message: "Selranda"
                                                .to_string(),
                                            title: error::errorList::error_s10.title.clone(),
                                            code: error::errorList::error_s10.code,
                                            message: error::errorList::error_s10.message.clone(),
                                            builded_message: error::errorList::error_s10
                                                .message
                                                .clone(),
                                            pos: mapper::defs::Cursor {
                                                range_start: data.parameters
                                                    [data.parameters.len() - 1]
                                                    .name_pos
                                                    .range_start
                                                    .clone()
                                                    .popChar(1),
                                                range_end: data.parameters
                                                    [data.parameters.len() - 1]
                                                    .type_pos
                                                    .range_end
                                                    .clone()
                                                    .skipChar(1),
                                            },
                                        });
                                    } else if data.parameters[data.parameters.len() - 1].type_text
                                        == ""
                                    {
                                        errors.push(error::Error {
                                            debug_message: "Elliead"
                                                .to_string(),
                                            title: error::errorList::error_s1.title.clone(),
                                            code: error::errorList::error_s1.code,
                                            message: error::errorList::error_s1.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s1.message.clone(),
                                                vec![error::ErrorBuildField {
                                                    key: "token".to_string(),
                                                    value: letter_char.to_string(),
                                                }],
                                            ),
                                            pos: mapper::defs::Cursor {
                                                range_start: self.pos.clone(),
                                                range_end: self.pos.clone().skipChar(1),
                                            },
                                        });
                                    }
                                    data.parameter_wrote = true;
                                }
                            } else if letter_char == ")" {
                                data.parameter_bracket_end_pos.range_start.0 = self.pos.0; //parameter start
                                data.parameter_bracket_end_pos.range_start.1 = self.pos.1; //parameter start
                                data.parameter_bracket_end_pos.range_end.0 = self.pos.skipChar(1).0; //parameter start
                                data.parameter_bracket_end_pos.range_end.1 = self.pos.skipChar(1).1; //parameter start
                                data.parameter_wrote = true;
                            }
                        } else if !data.return_typed {
                            if letter_char == "{" {
                                //Skipped return type it's void
                                data.return_typed = true;
                                data.return_type = types::Types::Void;
                                data.inside_code_wrote = true;
                                data.code_bracket_start.range_start.0 = self.pos.0; //Bracket start
                                data.code_bracket_start.range_start.1 = self.pos.1;
                            //Bracket start
                            } else if !data.pointer_typed {
                                if letter_char == ">" {
                                    data.pointer_typed = true
                                } else if letter_char != " " {
                                    errors.push(error::Error {
                                        debug_message: "Elase".to_string(),
                                        title: error::errorList::error_s1.title.clone(),
                                        code: error::errorList::error_s1.code,
                                        message: error::errorList::error_s1.message.clone(),
                                        builded_message: error::Error::build(
                                            error::errorList::error_s1.message.clone(),
                                            vec![error::ErrorBuildField {
                                                key: "token".to_string(),
                                                value: letter_char.to_string(),
                                            }],
                                        ),
                                        pos: mapper::defs::Cursor {
                                            range_start: self.pos.clone(),
                                            range_end: self.pos.clone().skipChar(1),
                                        },
                                    });
                                }
                            } else if data.pointer_typed && !data.return_typed {
                                let current_reliability = crate::utils::reliable_name_range(
                                    crate::utils::ReliableNameRanges::VariableName,
                                    letter_char.to_string(),
                                );

                                if current_reliability.reliable {
                                    if last_char == " " && data.return_type_text != "" {
                                        errors.push(error::Error {
                                            debug_message: "Ellie"
                                                .to_string(),
                                            title: error::errorList::error_s1.title.clone(),
                                            code: error::errorList::error_s1.code,
                                            message: error::errorList::error_s1.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s1.message.clone(),
                                                vec![error::ErrorBuildField {
                                                    key: "token".to_string(),
                                                    value: letter_char.to_string(),
                                                }],
                                            ),
                                            pos: mapper::defs::Cursor {
                                                range_start: self.pos.clone(),
                                                range_end: self.pos.clone().skipChar(1),
                                            },
                                        });
                                    } else {
                                        data.return_type_text += letter_char;
                                    }
                                } else {
                                    if letter_char == "{" {
                                        if data.return_type_text == "" {
                                            errors.push(error::Error {
                                                debug_message: "Aesthetics"
                                                    .to_string(),
                                                title: error::errorList::error_s8.title.clone(),
                                                code: error::errorList::error_s8.code,
                                                message: error::errorList::error_s8.message.clone(),
                                                builded_message: error::errorList::error_s8
                                                    .message
                                                    .clone(),
                                                pos: mapper::defs::Cursor {
                                                    range_start: self.pos.clone(),
                                                    range_end: self.pos.clone().skipChar(1),
                                                },
                                            });
                                        } else {
                                            data.return_typed = true;
                                            data.return_type = types::Types::Void;
                                            data.inside_code_wrote = true;
                                            data.code_bracket_start.range_start.0 = self.pos.0; //Bracket start
                                            data.code_bracket_start.range_start.1 = self.pos.1;
                                        }
                                    } else if letter_char != " " {
                                        errors.push(error::Error {
                                            debug_message: "Elsa"
                                                .to_string(),
                                            title: error::errorList::error_s1.title.clone(),
                                            code: error::errorList::error_s1.code,
                                            message: error::errorList::error_s1.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s1.message.clone(),
                                                vec![error::ErrorBuildField {
                                                    key: "token".to_string(),
                                                    value: letter_char.to_string(),
                                                }],
                                            ),
                                            pos: mapper::defs::Cursor {
                                                range_start: self.pos.clone(),
                                                range_end: self.pos.clone().skipChar(1),
                                            },
                                        });
                                    }
                                }
                            } else if letter_char != " " {
                                errors.push(error::Error {
                                    debug_message: "Aghtcnm".to_string(),
                                    title: error::errorList::error_s1.title.clone(),
                                    code: error::errorList::error_s1.code,
                                    message: error::errorList::error_s1.message.clone(),
                                    builded_message: error::Error::build(
                                        error::errorList::error_s1.message.clone(),
                                        vec![error::ErrorBuildField {
                                            key: "token".to_string(),
                                            value: letter_char.to_string(),
                                        }],
                                    ),
                                    pos: mapper::defs::Cursor {
                                        range_start: self.pos.clone(),
                                        range_end: self.pos.clone().skipChar(1),
                                    },
                                });
                            }
                        } else {
                            if letter_char == "{" {
                                data.inside_object_start = true;
                                data.inside_object_count += 1;
                            } else if letter_char == "}" {
                                if data.inside_object_start {
                                    if data.inside_object_count == 0 {
                                        data.inside_object_start = true;
                                    } else {
                                        data.inside_object_count -= 1;
                                    }
                                } else {
                                    let mut mapper = mapper::Mapper::new(
                                        data.inside_code_string.clone(),
                                        mapper::defs::MapperOptions {
                                            functions: true,
                                            break_on_error: false,
                                            loops: true,
                                            global_variables: true,
                                            collectives: true,
                                            variables: true,
                                        },
                                    );
                                    mapper.pos = self.pos;
                                    let mapped = mapper.Map();
                                    for i in mapped.syntax_errors {
                                        errors.push(i)
                                    }
                                    data.inside_code = mapped.items;
                                    self.collected.push(self.current.clone());
                                    self.current = Collecting::None;
                                }
                            } else {
                                data.inside_code_string += letter_char;
                            }
                        }
                    }
                    _ => (),
                };
                self.pos.1 += 1;
            } else if (last_char == "\r" || letter_char == "\n") {
                self.pos.0 += 1;
                self.pos.1 = 0;
            }
        }
        Mapped {
            items: self.collected.clone(),
            syntax_errors: errors,
        }
    }
}

/*

fn test() {

}
*/
