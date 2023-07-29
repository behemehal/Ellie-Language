pub mod array_processor;
pub mod as_processor;
pub mod brace_reference_processor;
pub mod byte_processor;
pub mod char_processor;
pub mod class_call_processor;
pub mod cloak_processor;
pub mod collective_processor;
pub mod decimal_processor;
pub mod enum_data_processor;
pub mod function_call_processor;
pub mod integer_processor;
pub mod negative_processor;
pub mod null_resolver_processor;
pub mod operator_processor;
pub mod reference_processor;
pub mod string_processor;
pub mod variable_processor;

use crate::syntax::types::*;
use ellie_core::{
    definite::{self, Converter},
    defs, utils,
};
use enum_as_inner::EnumAsInner;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, EnumAsInner)]
pub enum Processors {
    Integer(integer_type::IntegerTypeCollector),
    Byte(byte_type::ByteType),
    Decimal(decimal_type::DecimalTypeCollector),
    Char(char_type::CharType),
    String(string_type::StringTypeCollector),
    Variable(variable_type::VariableTypeCollector),
    Negative(negative_type::Negative),
    Array(array_type::ArrayTypeCollector),
    Operator(operator_type::OperatorTypeCollector),
    Reference(reference_type::ReferenceTypeCollector),
    BraceReference(brace_reference_type::BraceReferenceTypeCollector),
    EnumData(enum_data::EnumDataCollector),
    NullResolver(null_resolver::NullResolver),
    FunctionCall(function_call_type::FunctionCallCollector),
    ClassCall(class_call_type::ClassCallCollector),
    Cloak(cloak_type::CloakTypeCollector),
    Collective(collective_type::CollectiveTypeCollector),
    AsKeyword(as_keyword::AsKeywordCollector),
}

impl Processors {
    pub fn to_definite(&self) -> ellie_core::definite::types::Types {
        match self.clone() {
            Processors::Integer(e) => ellie_core::definite::types::Types::Integer(e.to_definite()),
            Processors::Byte(e) => ellie_core::definite::types::Types::Byte(e.to_definite()),
            Processors::Decimal(e) => ellie_core::definite::types::Types::Decimal(e.to_definite()),
            Processors::Char(e) => ellie_core::definite::types::Types::Char(e.to_definite()),
            Processors::String(e) => ellie_core::definite::types::Types::String(e.to_definite()),
            Processors::Variable(e) => {
                ellie_core::definite::types::Types::VariableType(e.to_definite())
            }
            Processors::Negative(e) => {
                ellie_core::definite::types::Types::Negative(e.to_definite())
            }
            Processors::Array(e) => ellie_core::definite::types::Types::Array(e.to_definite()),
            Processors::Operator(e) => {
                ellie_core::definite::types::Types::Operator(e.to_definite())
            }
            Processors::Reference(e) => {
                ellie_core::definite::types::Types::Reference(e.to_definite())
            }
            Processors::BraceReference(e) => {
                ellie_core::definite::types::Types::BraceReference(e.to_definite())
            }
            Processors::EnumData(e) => {
                ellie_core::definite::types::Types::EnumData(e.to_definite())
            }
            Processors::FunctionCall(e) => {
                ellie_core::definite::types::Types::FunctionCall(e.to_definite())
            }
            Processors::ClassCall(e) => {
                ellie_core::definite::types::Types::ClassCall(e.to_definite())
            }
            Processors::Cloak(e) => ellie_core::definite::types::Types::Cloak(e.to_definite()),
            Processors::Collective(e) => {
                ellie_core::definite::types::Types::Collective(e.to_definite())
            }
            Processors::AsKeyword(e) => {
                ellie_core::definite::types::Types::AsKeyword(e.to_definite())
            }
            Processors::NullResolver(e) => {
                ellie_core::definite::types::Types::NullResolver(e.to_definite())
            }
        }
    }

    pub fn from_definite(self, from: definite::types::Types) -> Self {
        match from {
            definite::types::Types::Integer(e) => {
                Processors::Integer(integer_type::IntegerTypeCollector::default().from_definite(e))
            }
            definite::types::Types::Byte(e) => {
                Processors::Byte(byte_type::ByteType::default().from_definite(e))
            }
            definite::types::Types::Decimal(e) => {
                Processors::Decimal(decimal_type::DecimalTypeCollector::default().from_definite(e))
            }
            definite::types::Types::String(e) => {
                Processors::String(string_type::StringTypeCollector::default().from_definite(e))
            }
            definite::types::Types::Char(e) => {
                Processors::Char(char_type::CharType::default().from_definite(e))
            }
            definite::types::Types::VariableType(e) => Processors::Variable(
                variable_type::VariableTypeCollector::default().from_definite(e),
            ),
            definite::types::Types::Bool(e) => {
                Processors::Variable(variable_type::VariableTypeCollector {
                    data: variable_type::VariableType {
                        value: if e.value {
                            "true".to_string()
                        } else {
                            "false".to_string()
                        },
                        ..Default::default()
                    },
                    complete: true,
                })
            }
            definite::types::Types::Array(e) => {
                Processors::Array(array_type::ArrayTypeCollector::default().from_definite(e))
            }
            definite::types::Types::Reference(e) => Processors::Reference(
                reference_type::ReferenceTypeCollector::default().from_definite(e),
            ),
            definite::types::Types::BraceReference(e) => Processors::BraceReference(
                brace_reference_type::BraceReferenceTypeCollector::default().from_definite(e),
            ),
            definite::types::Types::FunctionCall(e) => Processors::FunctionCall(
                function_call_type::FunctionCallCollector::default().from_definite(e),
            ),
            definite::types::Types::ClassCall(e) => Processors::ClassCall(
                class_call_type::ClassCallCollector::default().from_definite(e),
            ),
            definite::types::Types::Cloak(e) => {
                Processors::Cloak(cloak_type::CloakTypeCollector::default().from_definite(e))
            }
            definite::types::Types::Collective(e) => Processors::Collective(
                collective_type::CollectiveTypeCollector::default().from_definite(e),
            ),
            definite::types::Types::AsKeyword(e) => {
                Processors::AsKeyword(as_keyword::AsKeywordCollector::default().from_definite(e))
            }
            _ => panic!("NOT SUPPORTED, {:?}", from),
        }
    }

    pub fn is_static(&self) -> bool {
        match self {
            Processors::Integer(_) => true,
            Processors::Byte(_) => true,
            Processors::Decimal(_) => true,
            Processors::Char(_) => true,
            Processors::String(_) => true,
            Processors::FunctionCall(_) => false,
            Processors::Variable(e) => {
                if e.data.value == "false" || e.data.value == "true" {
                    true
                } else {
                    false
                }
            }
            Processors::Negative(e) => e.value.is_static(),
            Processors::Array(e) => e.data.collective.iter().all(|e| e.value.is_static()),
            Processors::Operator(e) => e.data.first.is_static() && e.data.second.is_static(),
            Processors::Reference(_) => false,
            Processors::BraceReference(_) => false,
            Processors::EnumData(_) => false,
            Processors::ClassCall(_) => false,
            Processors::Cloak(e) => e.data.collective.iter().all(|e| e.value.is_static()),
            Processors::Collective(e) => e.data.entries.iter().all(|e| e.value.is_static()),
            Processors::AsKeyword(e) => {
                if matches!(e.data.rtype.definer_type, crate::syntax::items::definers::DefinerTypes::Generic(ref e) if e.rtype == "bool")
                    && matches!(*e.data.target, Processors::Integer(ref e) if e.data.value == 1 || e.data.value == 0)
                {
                    true
                } else {
                    false
                }
            }
            Processors::NullResolver(_) => false,
        }
    }

    pub fn is_complete(&self) -> bool {
        match &self {
            Processors::Integer(e) => e.complete,
            Processors::Byte(e) => e.complete,
            Processors::Char(e) => e.complete,
            Processors::String(e) => e.complete,
            Processors::Variable(e) => e.complete,
            Processors::Decimal(e) => e.complete,
            Processors::Array(e) => e.complete,
            Processors::Negative(e) => e.value.is_complete(),
            Processors::Operator(e) => e.data.second.is_complete(),
            Processors::Reference(e) => !e.on_dot,
            Processors::EnumData(e) => e.complete,
            Processors::BraceReference(e) => e.complete,
            Processors::FunctionCall(e) => e.complete,
            Processors::ClassCall(e) => e.complete,
            Processors::Cloak(e) => e.complete,
            Processors::Collective(e) => e.complete,
            Processors::AsKeyword(e) => e.complete,
            Processors::NullResolver(_) => true,
        }
    }

    pub fn is_not_initialized(&self) -> bool {
        match &self {
            Processors::Integer(_) => false,
            Processors::Byte(_) => false,
            Processors::Decimal(_) => false,
            Processors::Char(_) => false,
            Processors::String(_) => false,
            Processors::Variable(e) => e.data.value == "",
            Processors::Negative(_) => false,
            Processors::Array(_) => false,
            Processors::Operator(_) => false,
            Processors::Reference(_) => false,
            Processors::EnumData(_) => false,
            Processors::BraceReference(_) => false,
            Processors::FunctionCall(_) => false,
            Processors::ClassCall(_) => false,
            Processors::Cloak(_) => false,
            Processors::Collective(_) => false,
            Processors::AsKeyword(_) => false,
            Processors::NullResolver(_) => false,
        }
    }

    pub fn is_assignable(&self) -> bool {
        match self {
            Processors::Variable(_) => true,
            Processors::Reference(_) => true,
            Processors::BraceReference(_) => true,
            _ => false
        }
    }

    pub fn get_pos(&self) -> defs::Cursor {
        match self {
            Processors::Integer(e) => e.data.pos,
            Processors::Byte(e) => e.pos,
            Processors::Decimal(e) => e.data.pos,
            Processors::Char(e) => e.pos,
            Processors::String(e) => e.data.pos,
            Processors::Variable(e) => e.data.pos,
            Processors::Negative(e) => e.pos,
            Processors::Array(e) => e.data.pos,
            Processors::Operator(e) => e.data.pos,
            Processors::Reference(e) => e.data.pos,
            Processors::BraceReference(e) => e.data.pos,
            Processors::FunctionCall(e) => e.data.pos,
            Processors::ClassCall(e) => e.data.pos,
            Processors::Cloak(e) => e.data.pos,
            Processors::Collective(e) => e.data.pos,
            Processors::AsKeyword(e) => e.data.pos,
            Processors::NullResolver(e) => e.pos,
            Processors::EnumData(e) => e.data.pos,
        }
    }
}

impl Default for Processors {
    fn default() -> Self {
        Processors::Variable(variable_type::VariableTypeCollector::default())
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TypeProcessor {
    pub current: Processors,
    pub ignore: bool,
}

impl TypeProcessor {
    pub fn is_complete(&self) -> bool {
        self.current.is_complete() && !self.ignore
    }
}

impl super::Processor for TypeProcessor {
    fn iterate(
        &mut self,
        errors: &mut Vec<ellie_core::error::Error>,
        cursor: ellie_core::defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) -> bool {
        let not_initalized = matches!(&self.current, Processors::Variable(x) if x.data.value == "");

        if letter_char == '{' && not_initalized {
            self.current = Processors::Collective(collective_type::CollectiveTypeCollector {
                data: collective_type::CollectiveType {
                    pos: defs::Cursor::build_from_cursor(cursor),
                    ..Default::default()
                },
                ..Default::default()
            });
        } else if letter_char == ' '
            && matches!(&self.current, Processors::Variable(x) if x.data.value == "new")
        {
            self.current = Processors::ClassCall(class_call_type::ClassCallCollector {
                data: class_call_type::ClassCall {
                    pos: defs::Cursor::build_from_cursor(cursor),
                    target_pos: self.current.get_pos(),
                    ..Default::default()
                },
                ..Default::default()
            });
        } else if letter_char == '[' && (not_initalized || self.is_complete()) {
            if not_initalized {
                self.current = Processors::Array(array_type::ArrayTypeCollector {
                    data: array_type::ArrayType {
                        pos: defs::Cursor::build_from_cursor(cursor),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            } else if self.current.as_operator().is_none() {
                self.current =
                    Processors::BraceReference(brace_reference_type::BraceReferenceTypeCollector {
                        data: brace_reference_type::BraceReferenceType {
                            reference: Box::new(self.current.clone()),
                            reference_pos: self.current.get_pos(),
                            pos: self.current.get_pos(),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
            }
        } else if letter_char == '(' && (not_initalized || self.is_complete()) {
            if not_initalized {
                self.current = Processors::Cloak(cloak_type::CloakTypeCollector {
                    data: cloak_type::CloakType {
                        pos: defs::Cursor::build_from_cursor(cursor),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            } else {
                match self.current.as_operator() {
                    Some(operator) => {
                        self.current = Processors::Operator(operator_type::OperatorTypeCollector {
                            data: operator_type::OperatorType {
                                first: operator.data.first.clone(),
                                first_pos: self.current.get_pos(),
                                operator: operator.data.operator.clone(),
                                pos: defs::Cursor::build_from_cursor(cursor),
                                ..Default::default()
                            },
                            itered_cache: Box::new(TypeProcessor {
                                current: Processors::FunctionCall(
                                    function_call_type::FunctionCallCollector {
                                        data: function_call_type::FunctionCall {
                                            target: operator.data.second.clone(),
                                            target_pos: operator.data.second_pos,
                                            pos: operator.data.second_pos,
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                ),
                                ignore: false,
                            }),
                            operator_collected: true,
                            first_filled: true,
                            ..Default::default()
                        });
                    }
                    None => {
                        self.current =
                            Processors::FunctionCall(function_call_type::FunctionCallCollector {
                                data: function_call_type::FunctionCall {
                                    target: Box::new(self.current.clone()),
                                    target_pos: self.current.get_pos(),
                                    pos: self.current.get_pos(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            });
                    }
                }
            }
        } else if letter_char == '\'' && not_initalized {
            self.current = Processors::Char(char_type::CharType {
                pos: defs::Cursor::build_from_cursor(cursor),
                ..Default::default()
            });
        } else if letter_char == '!' && not_initalized {
            self.current = Processors::Negative(negative_type::Negative {
                pos: defs::Cursor::build_from_cursor(cursor),
                ..Default::default()
            });
        } else if letter_char == '!' && last_char != ' ' && self.current.is_complete() {
            self.current = Processors::NullResolver(null_resolver::NullResolver {
                target: Box::new(self.current.clone()),
                target_pos: self.current.get_pos(),
                pos: defs::Cursor {
                    range_start: self.current.get_pos().range_start,
                    range_end: defs::CursorPosition::default(),
                },
            });
        } else if letter_char == '"' && not_initalized {
            self.current = Processors::String(string_type::StringTypeCollector {
                data: string_type::StringType {
                    pos: defs::Cursor::build_from_cursor(cursor),
                    ..Default::default()
                },
                ..Default::default()
            });
        } else if letter_char == 'x'
            && matches!(&self.current, Processors::Integer(x) if x.raw == "0")
        {
            self.current = Processors::Byte(byte_type::ByteType {
                value: 0,
                pos: self.current.get_pos(),
                complete: false,
            });
            // Skip to next iteration
            return true;
        } else if letter_char == '.'
            && (not_initalized || matches!(&self.current, Processors::Integer(_)))
        {
            self.current = Processors::Decimal(decimal_type::DecimalTypeCollector {
                base: if let Processors::Integer(e) = &self.current {
                    e.raw.to_string()
                } else {
                    "0.".to_string()
                },
                base_p: if not_initalized {
                    integer_type::IntegerTypeCollector::default()
                } else {
                    self.current.as_integer().unwrap().clone()
                },
                data: decimal_type::DecimalType {
                    raw: if let Processors::Integer(e) = &self.current {
                        e.raw.to_string() + "."
                    } else {
                        "0.".to_string()
                    },
                    pos: defs::Cursor::build_from_cursor(cursor),
                    ..Default::default()
                },
                ..Default::default()
            });
        } else if (letter_char.to_string().parse::<i8>().is_ok() || letter_char == '-')
            && not_initalized
        {
            self.current = Processors::Integer(integer_type::IntegerTypeCollector {
                data: integer_type::IntegerType {
                    pos: defs::Cursor::build_from_cursor(cursor),
                    ..Default::default()
                },
                ..Default::default()
            });
        } else if matches!(&self.current, Processors::Decimal(e) if e.no_base)
            && last_char == '.'
            && letter_char.to_string().parse::<i8>().is_err()
            && utils::reliable_name_range(utils::ReliableNameRanges::VariableName, letter_char)
                .reliable
        {
            if self.current.as_operator().is_none() {
                self.current = Processors::Reference(reference_type::ReferenceTypeCollector {
                    data: reference_type::ReferenceType {
                        reference: Box::new(Processors::Integer(
                            self.current.as_decimal().unwrap().base_p.clone(),
                        )),
                        reference_pos: self.current.get_pos(),
                        chain: vec![reference_type::Chain {
                            pos: defs::Cursor {
                                range_start: cursor,
                                ..Default::default()
                            },
                            ..Default::default()
                        }],
                        pos: defs::Cursor::build_from_cursor(cursor),
                        ..Default::default()
                    },
                    on_dot: false,
                    complete: true,
                });
            }
        } else if self.is_complete() && letter_char == 'a' && last_char == ' ' {
            self.current = Processors::AsKeyword(as_keyword::AsKeywordCollector {
                data: as_keyword::AsKeyword {
                    target: Box::new(self.current.clone()),
                    pos: self.current.get_pos(),
                    target_pos: self.current.get_pos(),
                    ..Default::default()
                },
                keyword_pos: 0,
                ..Default::default()
            })
        } else if self.is_complete() && letter_char == '.' {
            if self.current.as_reference().is_none() && self.current.as_operator().is_none() {
                self.current = Processors::Reference(reference_type::ReferenceTypeCollector {
                    data: reference_type::ReferenceType {
                        reference: Box::new(self.current.clone()),
                        reference_pos: self.current.get_pos(),
                        pos: defs::Cursor::build_from_cursor(cursor),
                        ..Default::default()
                    },
                    on_dot: false,
                    complete: false,
                });
            }
        } else if self.is_complete()
            && utils::is_operator_start(letter_char)
            && self.current.as_operator().is_none()
            && !(matches!(&self.current, Processors::AsKeyword(e) if matches!(e.data.rtype.definer_type, crate::syntax::items::definers::DefinerTypes::Generic(_))))
        {
            //Operator priority
            if self.current.as_null_resolver().is_some() && last_char != ' ' && letter_char == '=' {
                let null_r = self.current.as_null_resolver().unwrap();
                self.current = Processors::Operator(operator_type::OperatorTypeCollector {
                    data: operator_type::OperatorType {
                        first: Box::new(*null_r.target.clone()),
                        first_pos: null_r.target_pos,
                        pos: defs::Cursor::build_from_cursor(cursor),
                        ..Default::default()
                    },
                    operator_collect: "!".to_string(),
                    first_filled: true,
                    ..Default::default()
                });
            } else {
                self.current = Processors::Operator(operator_type::OperatorTypeCollector {
                    data: operator_type::OperatorType {
                        first: Box::new(self.current.clone()),
                        first_pos: self.current.get_pos(),
                        pos: defs::Cursor::build_from_cursor(cursor),
                        ..Default::default()
                    },
                    first_filled: true,
                    ..Default::default()
                });
            }
        }

        match &mut self.current {
            Processors::Integer(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Byte(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Char(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::String(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Variable(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Decimal(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Negative(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Array(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Operator(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Reference(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::BraceReference(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::EnumData(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::FunctionCall(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::ClassCall(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Cloak(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Collective(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::AsKeyword(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::NullResolver(e) => e.iterate(errors, cursor, last_char, letter_char),
        }
    }
}
