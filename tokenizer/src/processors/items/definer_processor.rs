use std::panic;

use crate::processors::{reliable_char, Processor};
use ellie_core::{defs, error};

#[derive(Default, Clone, Debug)]
pub struct CloakType {
    pub entries: Vec<DefinerTypes>,
    pub at_comma: bool,
    pub child_cache: Box<DefinerProcessor>,
}

#[derive(Default, Clone, Debug)]
pub struct ArrayType {
    pub rtype: Box<DefinerTypes>,
    pub size: usize,
    pub at_comma: bool,
    pub child_cache: Box<DefinerProcessor>,
}

#[derive(Default, Clone, Debug)]
pub struct CollectiveType {
    pub key: Box<DefinerTypes>,
    pub value: Box<DefinerTypes>,
    pub key_collected: bool,
    pub at_comma: bool,
    pub child_cache: Box<DefinerProcessor>,
}

#[derive(Default, Clone, Debug)]
pub struct VectorType {
    pub rtype: Box<DefinerTypes>,
}

#[derive(Default, Clone, Debug)]
pub struct FutureType {
    pub rtype: Box<DefinerTypes>,
}

#[derive(Default, Clone, Debug)]
pub struct NullableType {
    pub rtype: Box<DefinerTypes>,
}

#[derive(Default, Clone, Debug)]
pub struct GenericType {
    pub rtype: String,
}

#[derive(Clone, Debug)]
pub enum DefinerTypes {
    Cloak(CloakType),
    Array(ArrayType),
    Collective(CollectiveType),
    Vector(VectorType),
    Future(FutureType),
    Nullable(NullableType),
    Generic(GenericType),
}

impl Default for DefinerTypes {
    fn default() -> Self {
        DefinerTypes::Generic(GenericType {
            rtype: String::new(),
        })
    }
}

#[derive(Default, Clone, Debug)]
pub struct DefinerProcessor {
    pub definer_type: DefinerTypes,
    pub errors: Vec<error::Error>,
    pub cursor: defs::CursorPosition,
    pub complete: bool,
}

impl Processor for DefinerProcessor {
    fn new() -> Self {
        DefinerProcessor::default()
    }

    fn keyword(&self) -> &str {
        ""
    }

    fn has_accessibility(&self) -> bool {
        false
    }

    fn iterate(&mut self, cursor: defs::CursorPosition, last_char: char, letter_char: char) {
        match self.definer_type {
            DefinerTypes::Cloak(_) => todo!(),
            DefinerTypes::Array(_) => todo!(),
            DefinerTypes::Collective(ref mut collective_type) => {
                if !collective_type.key_collected {
                    if collective_type.child_cache.is_complete() && letter_char == ',' {
                        collective_type.key_collected = true;
                        collective_type.key =
                            Box::new(collective_type.child_cache.definer_type.clone());
                        collective_type.child_cache = Box::new(Processor::new());
                    } else {
                        collective_type
                            .child_cache
                            .iterate(cursor, last_char, letter_char);
                    }
                } else {
                    collective_type
                        .child_cache
                        .iterate(cursor, last_char, letter_char);
                    if collective_type.child_cache.is_complete() && letter_char == '}' {
                        collective_type.key_collected = true;
                        collective_type.key =
                            Box::new(collective_type.child_cache.definer_type.clone());
                        collective_type.child_cache = Box::new(Processor::new());
                    }
                }
            }
            DefinerTypes::Vector(_) => {
                //Vector type resolved in array, if another character is found after brace
                //it's a syntax error
                if self.complete || letter_char != ']' {
                    //If brace is already put or char is not close brace
                    self.errors.push(error::errorList::error_s1.clone().build(
                        vec![error::ErrorBuildField {
                            key: "$token".to_string(),
                            value: letter_char.to_string(),
                        }],
                        "0x00".to_owned(),
                        defs::Cursor {
                            range_start: cursor,
                            range_end: cursor.clone().skip_char(1),
                        },
                    ));
                } else if letter_char == ']' {
                    //After we see '*' char in array size position, we update
                    //Type as vector, so completing array brace is up to vector
                    self.complete = true;
                }
            }
            DefinerTypes::Future(_) => {
                //Future type resolved in generic so if anything after future definer char exists
                //it's a syntax error
                self.errors.push(error::errorList::error_s1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "$token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    "0x00".to_owned(),
                    defs::Cursor {
                        range_start: cursor,
                        range_end: cursor.clone().skip_char(1),
                    },
                ));
            }
            DefinerTypes::Nullable(_) => {
                //Nullable type resolved in generic so if anything after nullable definer char exists
                //it's a syntax error
                self.errors.push(error::errorList::error_s1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "$token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    "0x00".to_owned(),
                    defs::Cursor {
                        range_start: cursor,
                        range_end: cursor.clone().skip_char(1),
                    },
                ));
            }
            DefinerTypes::Generic(ref mut generic_type) => {
                if reliable_char(&letter_char) {
                    if last_char == ' ' {
                        panic!("Error: UNEXPECTED TOKEN, {:#?}", letter_char);
                    } else {
                        if generic_type.rtype.is_empty() {
                            self.complete = true;
                        }
                        generic_type.rtype += &letter_char.to_string();
                    }
                } else {
                    if letter_char == '>' && !generic_type.rtype.is_empty() {
                        self.definer_type = DefinerTypes::Future(FutureType {
                            rtype: Box::new(self.definer_type.clone()),
                        });
                        self.complete = true;
                    } else if letter_char == '?' && !generic_type.rtype.is_empty() {
                        self.definer_type = DefinerTypes::Nullable(NullableType {
                            rtype: Box::new(self.definer_type.clone()),
                        });
                        self.complete = true;
                    } else if letter_char == '{' && generic_type.rtype.is_empty() {
                        self.definer_type = DefinerTypes::Collective(CollectiveType::default());
                    } else if letter_char == '(' && generic_type.rtype.is_empty() {
                        self.definer_type = DefinerTypes::Cloak(CloakType::default());
                    } else if letter_char == '[' && generic_type.rtype.is_empty() {
                        self.definer_type = DefinerTypes::Array(ArrayType::default());
                    } else if letter_char != ' ' {
                        self.errors.push(error::errorList::error_s1.clone().build(
                            vec![error::ErrorBuildField {
                                key: "$token".to_string(),
                                value: letter_char.to_string(),
                            }],
                            "0x00".to_owned(),
                            defs::Cursor {
                                range_start: cursor,
                                range_end: cursor.clone().skip_char(1),
                            },
                        ));
                    }
                }
            }
        }
    }

    fn has_error(&self) -> bool {
        self.errors.is_empty()
    }

    fn errors(&self) -> Vec<ellie_core::error::Error> {
        self.errors.clone()
    }

    fn is_complete(&self) -> bool {
        self.complete
    }
}

//Cloak     : (int, string)
//Collective: {int, string}
//Array     : [int, 3]
//Vector    : [int, *]
//Future    : int>
//Nullable  : int?
