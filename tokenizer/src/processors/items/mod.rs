use ellie_core::{
    definite::{items::Collecting, Converter},
    defs, error,
};
use enum_as_inner::EnumAsInner;
use serde::{Deserialize, Serialize};

use crate::syntax::items::*;
mod class_processor;
mod condition_processor;
mod constructor_processor;
mod definer_processor;
mod file_key;
mod for_loop_processor;
mod function_processor;
mod getter_call;
mod import_processor;
mod ret_processor;
mod setter_call;
mod variable_processor;

#[derive(Debug, Clone, Serialize, Deserialize, EnumAsInner)]
pub enum Processors {
    Variable(variable::VariableCollector),
    GetterCall(getter_call::GetterCall),
    SetterCall(setter_call::SetterCall),
    Function(function::FunctionCollector),
    FileKey(file_key::FileKey),
    Import(import::Import),
    ForLoop(for_loop::ForLoop),
    Condition(condition::Condition),
    Constructor(constructor::Constructor),
    Class(class::Class),
    Ret(ret::Ret),

    SelfItem(self_item::SelfItem),          //VirtualValues
    GenericItem(generic_item::GenericItem), //VirtualValues
    FunctionParameter(function_parameter::FunctionParameter), //VirtualValues
    ConstructorParameter(constructor_parameter::ConstructorParameter), //DISABLED
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
            Processors::ForLoop(e) => e.complete,
            Processors::Condition(e) => {
                e.chains.len() != 0 && e.chains.clone()[e.chains.len() - 1].complete
            }
            Processors::Constructor(e) => e.complete,
            Processors::Ret(e) => e.complete,
            Processors::Class(e) => e.complete,
            Processors::SelfItem(_) => panic!("Unexpected behaviour"),
            Processors::GenericItem(_) => panic!("Unexpected behaviour"),
            Processors::FunctionParameter(_) => panic!("Unexpected behaviour"),
            Processors::ConstructorParameter(_) => panic!("Unexpected behaviour"),
        }
    }

    pub fn is_initalized(&self) -> bool {
        match self.clone() {
            Processors::GetterCall(e) => !e.data.is_not_initialized(),
            Processors::SelfItem(_) => panic!("Unexpected behaviour"),
            Processors::GenericItem(_) => panic!("Unexpected behaviour"),
            Processors::FunctionParameter(_) => panic!("Unexpected behaviour"),
            Processors::ConstructorParameter(_) => panic!("Unexpected behaviour"),
            _ => true,
        }
    }

    pub fn is_virtual(&self) -> bool {
        match self.clone() {
            Processors::SelfItem(_) => true,
            Processors::GenericItem(_) => true,
            Processors::FunctionParameter(_) => true,
            Processors::ConstructorParameter(_) => true,
            _ => false,
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
            Processors::ForLoop(e) => e.pos,
            Processors::Condition(e) => e.pos,
            Processors::Constructor(e) => e.pos,
            Processors::Ret(e) => e.pos,
            Processors::Class(e) => e.pos,
            Processors::SelfItem(_) => ellie_core::defs::Cursor::default(),
            Processors::GenericItem(_) => ellie_core::defs::Cursor::default(),
            Processors::FunctionParameter(e) => e.pos,
            Processors::ConstructorParameter(_) => ellie_core::defs::Cursor::default(),
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
            Processors::ForLoop(e) => Collecting::ForLoop(e.to_definite()),
            Processors::Condition(e) => Collecting::Condition(e.to_definite()),
            Processors::Constructor(e) => Collecting::Constructor(e.to_definite()),
            Processors::Class(e) => Collecting::Class(e.to_definite()),
            Processors::Ret(e) => Collecting::Ret(e.to_definite()),

            Processors::SelfItem(_) => panic!("Unexpected behaviour"),
            Processors::GenericItem(_) => panic!("Unexpected behaviour"),
            Processors::FunctionParameter(_) => panic!("Unexpected behaviour"),
            Processors::ConstructorParameter(_) => panic!("Unexpected behaviour"),
        }
    }

    pub fn from_definite(self, from: Collecting) -> Processors {
        match from {
            Collecting::Variable(e) => {
                Processors::Variable(variable::VariableCollector::default().from_definite(e))
            }
            Collecting::Function(e) => {
                Processors::Function(function::FunctionCollector::default().from_definite(e))
            }
            Collecting::ForLoop(e) => {
                Processors::ForLoop(for_loop::ForLoop::default().from_definite(e))
            }
            Collecting::Condition(e) => {
                Processors::Condition(condition::Condition::default().from_definite(e))
            }
            Collecting::Class(e) => Processors::Class(class::Class::default().from_definite(e)),
            Collecting::Ret(e) => Processors::Ret(ret::Ret::default().from_definite(e)),
            Collecting::Constructor(e) => {
                Processors::Constructor(constructor::Constructor::default().from_definite(e))
            }
            Collecting::Import(e) => Processors::Import(import::Import::default().from_definite(e)),
            Collecting::FileKey(e) => {
                Processors::FileKey(file_key::FileKey::default().from_definite(e))
            }
            Collecting::Getter(_) => todo!(),
            Collecting::Setter(_) => todo!(),
            Collecting::Generic(_) => todo!(),
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

impl super::Processor for ItemProcessor {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) -> bool {
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
                    pos: self.current.get_pos(),
                    ..Default::default()
                },
                ..Default::default()
            });
            self.used_modifier = Modifier::None;
        } else if keyword == "fn" && letter_char == ' ' {
            self.current = Processors::Function(function::FunctionCollector {
                data: function::Function {
                    public: self.used_modifier == Modifier::Pub,
                    pos: self.current.get_pos(),
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
        } else if keyword == "import" && letter_char == ' ' {
            self.current = Processors::Import(import::Import {
                public: self.used_modifier == Modifier::Pub,
                pos: self.current.get_pos(),
                ..Default::default()
            });
        } else if keyword == "class" && letter_char == ' ' {
            self.current = Processors::Class(class::Class {
                public: self.used_modifier == Modifier::Pub,
                pos: self.current.get_pos(),
                ..Default::default()
            });
        } else if not_initialized && self.used_modifier == Modifier::None && letter_char == '@' {
            self.current = Processors::FileKey(file_key::FileKey {
                pos: self.current.get_pos(),
                ..Default::default()
            });
        } else if self.used_modifier == Modifier::None && keyword == "ret" && letter_char == ' ' {
            self.current = Processors::Ret(ret::Ret {
                keyword_pos: self.current.get_pos(),
                pos: self.current.get_pos(),
                ..Default::default()
            });
        } else if self.used_modifier == Modifier::None && keyword == "co" && letter_char == '(' {
            self.current = Processors::Constructor(constructor::Constructor {
                pos: self.current.get_pos(),
                ..Default::default()
            });
        } else if self.used_modifier == Modifier::None && keyword == "for" && letter_char == ' ' {
            self.current = Processors::ForLoop(for_loop::ForLoop::default());
        } else if self.used_modifier == Modifier::None && keyword == "if" && letter_char == ' ' {
            self.current = Processors::Condition(condition::Condition {
                chains: vec![condition::ConditionChain {
                    rtype: condition::ConditionType::If,
                    keyword_pos: self.current.get_pos(),
                    ..Default::default()
                }],
                pos: self.current.get_pos(),
                ..Default::default()
            });
        } else if self.used_modifier == Modifier::None && keyword == "else" && letter_char == ' ' {
            self.current = Processors::Condition(condition::Condition {
                chains: vec![condition::ConditionChain {
                    rtype: condition::ConditionType::ElseIf,
                    keyword_pos: self.current.get_pos(),
                    ..Default::default()
                }],
                pos: self.current.get_pos(),
                ..Default::default()
            });
        }

        match &mut self.current {
            Processors::GetterCall(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Variable(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::SetterCall(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::FileKey(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Function(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Import(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::ForLoop(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Condition(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Constructor(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Ret(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Class(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::SelfItem(_) => panic!("Unexpected behaviour"),
            Processors::GenericItem(_) => panic!("Unexpected behaviour"),
            Processors::FunctionParameter(_) => panic!("Unexpected behaviour"),
            Processors::ConstructorParameter(_) => panic!("Unexpected behaviour"),
        }
    }
}
