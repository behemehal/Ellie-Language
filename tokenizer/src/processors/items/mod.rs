use ellie_core::{
    definite::{items::Collecting, Converter},
    defs, error,
};
use serde::{Deserialize, Serialize};

use crate::syntax::items::*;
pub mod definer_processor;
pub mod file_key;
pub mod function_processor;
pub mod getter_call;
mod import_processor;
pub mod setter_call;
pub mod variable_processor;

pub trait Processor {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    );
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Processors {
    Variable(variable::VariableCollector),
    GetterCall(getter_call::GetterCall),
    SetterCall(setter_call::SetterCall),
    Function(function::FunctionCollector),
    FileKey(file_key::FileKey),
    Import(import::Import),
}

impl Processors {
    pub fn is_complete(&self) -> bool {
        match self.clone() {
            Processors::GetterCall(e) => e.complete,
            Processors::Variable(e) => e.complete,
            Processors::SetterCall(e) => e.complete,
            Processors::FileKey(e) => e.complete,
            Processors::Function(e) => e.complete,
            Processors::Import(e) => e.complete,
        }
    }

    pub fn is_initalized(&self) -> bool {
        match self.clone() {
            Processors::GetterCall(e) => !e.data.is_not_initialized(),
            _ => true,
        }
    }

    pub fn get_pos(&self) -> defs::Cursor {
        match self {
            Processors::Variable(e) => e.data.pos,
            Processors::GetterCall(e) => e.pos,
            Processors::SetterCall(e) => e.pos,
            Processors::Function(e) => e.data.pos,
            Processors::FileKey(e) => e.pos,
            Processors::Import(e) => e.pos,
        }
    }

    pub fn to_definite(self) -> Collecting {
        match self {
            Processors::Variable(e) => Collecting::Variable(e.to_definite()),
            Processors::GetterCall(e) => Collecting::GetterCall(e.to_definite()),
            Processors::SetterCall(e) => Collecting::SetterCall(e.to_definite()),
            Processors::FileKey(e) => Collecting::FileKey(e.to_definite()),
            Processors::Function(e) => Collecting::Function(e.to_definite()),
            Processors::Import(e) => Collecting::Import(e.to_definite()),
        }
    }

    pub fn from_definite(self, from: Collecting) -> Processors {
        match from {
            Collecting::ImportItem(_) => todo!(),
            Collecting::Variable(e) => {
                Processors::Variable(variable::VariableCollector::default().from_definite(e))
            }
            Collecting::Function(e) => {
                Processors::Function(function::FunctionCollector::default().from_definite(e))
            }
            Collecting::ForLoop(_) => todo!(),
            Collecting::Condition(_) => todo!(),
            Collecting::Class(_) => todo!(),
            Collecting::Ret(_) => todo!(),
            Collecting::Constructor(_) => todo!(),
            Collecting::Import(_) => todo!(),
            Collecting::FileKey(e) => {
                Processors::FileKey(file_key::FileKey::default().from_definite(e))
            }
            Collecting::Getter(_) => todo!(),
            Collecting::Setter(_) => todo!(),
            Collecting::NativeClass => todo!(),
            Collecting::GetterCall(e) => {
                Processors::GetterCall(getter_call::GetterCall::default().from_definite(e))
            }
            Collecting::SetterCall(e) => {
                Processors::SetterCall(setter_call::SetterCall::default().from_definite(e))
            }
            Collecting::Enum(_) => todo!(),
            Collecting::NativeFunction(_) => todo!(),
            Collecting::None => todo!(),
        }
    }
}

impl Default for Processors {
    fn default() -> Self {
        Processors::GetterCall(getter_call::GetterCall::default())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Modifier {
    Pri,
    Pub,
    None,
}

impl Default for Modifier {
    fn default() -> Self {
        Modifier::None
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemProcessor {
    pub current: Processors,
    pub used_modifier: Modifier,
}

impl ItemProcessor {
    pub fn is_complete(&self) -> bool {
        self.current.is_complete()
    }
}

impl Processor for ItemProcessor {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) {
        let (is_var, keyword) = if let Processors::GetterCall(x) = self.current.clone() {
            match x.cache.current {
                super::types::Processors::Variable(e) => (true, e.data.value),
                super::types::Processors::Operator(e) => {
                    if e.first_filled
                        && e.operator_collected
                        && matches!(
                            e.data.operator,
                            crate::syntax::types::operator_type::Operators::AssignmentType(_)
                        )
                    {
                        self.current = Processors::SetterCall(setter_call::SetterCall {
                            target: *e.data.first,
                            operator: e.data.operator.as_assignment_type().unwrap().clone(),
                            cache: *e.itered_cache,
                            pos: x.pos,
                            ..Default::default()
                        })
                    }
                    (false, "".to_string())
                }
                _ => (false, "".to_string()),
            }
        } else {
            (false, "".to_string())
        };

        let not_initialized = is_var && keyword == "";

        if (keyword == "pri" || keyword == "pub")
            && self.used_modifier == Modifier::None
            && letter_char == ' '
        {
            if keyword == "pri" {
                self.used_modifier = Modifier::Pri;
            } else {
                self.used_modifier = Modifier::Pub;
            }
            self.current = Processors::default();
        } else if (keyword == "v" || keyword == "c") && letter_char == ' ' {
            self.current = Processors::Variable(variable::VariableCollector {
                data: variable::Variable {
                    public: self.used_modifier == Modifier::Pub,
                    constant: letter_char == 'c',
                    pos: defs::Cursor {
                        range_start: cursor,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            });
            self.used_modifier = Modifier::None;
        } else if keyword == "fn" && letter_char == ' ' {
            self.current = Processors::Function(function::FunctionCollector {
                data: function::Function {
                    public: self.used_modifier == Modifier::Pub,
                    ..Default::default()
                },
                ..Default::default()
            })
        } else if keyword == "enum" && letter_char == ' ' {
            panic!("enum not implemented");
        } else if keyword == "set" && letter_char == ' ' {
            panic!("setter not implemented");
        } else if keyword == "get" && letter_char == ' ' {
            panic!("getter not implemented");
        } else if keyword == "class" && letter_char == ' ' {
            panic!("class not implemented");
        } else if not_initialized && self.used_modifier == Modifier::None && letter_char == '@' {
            self.current = Processors::FileKey(file_key::FileKey {
                pos: defs::Cursor {
                    range_start: cursor,
                    ..Default::default()
                },
                ..Default::default()
            });
        } else if self.used_modifier == Modifier::None && keyword == "co" && letter_char == ' ' {
            panic!("co not implemented");
        } else if self.used_modifier == Modifier::None && keyword == "for" && letter_char == ' ' {
            panic!("for not implemented");
        } else if self.used_modifier == Modifier::None && keyword == "if" && letter_char == ' ' {
            panic!("if not implemented");
        } else if self.used_modifier == Modifier::None && keyword == "else" && letter_char == ' ' {
            panic!("else not implemented");
        } else if self.used_modifier == Modifier::None && keyword == "import" && letter_char == ' '
        {
            self.current = Processors::Import(import::Import::default());
        }

        match &mut self.current {
            Processors::GetterCall(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Variable(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::SetterCall(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::FileKey(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Function(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Import(e) => e.iterate(errors, cursor, last_char, letter_char),
        }
    }
}
