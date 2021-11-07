use crate::processors::{reliable_char, Processor};
use ellie_core::{defs, error};

#[derive(Default)]
pub struct CloakType {
    pub entries: Vec<DefinerTypes>,
    pub at_comma: bool,
    pub child_cache: Box<DefinerProcessor>,
}

#[derive(Default)]
pub struct ArrayType {
    pub rtype: Box<DefinerTypes>,
    pub size: usize,
    pub at_comma: bool,
    pub child_cache: Box<DefinerProcessor>,
}

#[derive(Default)]
pub struct CollectiveType {
    pub key: Box<DefinerTypes>,
    pub value: Box<DefinerTypes>,
    pub at_comma: bool,
    pub child_cache: Box<DefinerProcessor>,
}

#[derive(Default)]
pub struct VectorType {
    pub rtype: Box<DefinerTypes>,
}

#[derive(Default)]
pub struct FutureType {
    pub rtype: Box<DefinerTypes>,
}

#[derive(Default)]
pub struct NullableType {
    pub rtype: Box<DefinerTypes>,
}

#[derive(Default)]
pub struct GenericType {
    pub rtype: String,
}

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

#[derive(Default)]
pub struct DefinerProcessor {
    pub definer_type: DefinerTypes,
    pub errors: Vec<error::Error>,
    pub cursor: defs::CursorPosition,
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
            DefinerTypes::Collective(_) => todo!(),
            DefinerTypes::Vector(_) => todo!(),
            DefinerTypes::Future(_) => todo!(),
            DefinerTypes::Nullable(_) => todo!(),
            DefinerTypes::Generic(generic_type) => {
                if reliable_char(&letter_char) {
                } else {
                    if letter_char == '>' {
                    } else if letter_char == '?' {
                    } else if letter_char != ' ' {
                        self.errors.push(error::errorList::error_s1.clone().build(
                            vec![error::ErrorBuildField {
                                key: "$token",
                                value: todo!(),
                            }],
                            "0x00".to_owned(),
                            defs::Cursor {
                                range_start: cursor,
                                range_end: cursor.skip_char(1),
                            },
                        ));
                    }
                }
            }
        }
    }

    fn has_error(&self) -> bool {
        todo!()
    }

    fn errors(&self) -> Vec<ellie_core::error::Error> {
        todo!()
    }

    fn is_complete(&self) -> bool {
        todo!()
    }
}

//Cloak     : (int, string)
//Collective: {int, string}
//Array     : [int, 3]
//Vector    : [int, *]
//Future    : int>
//Nullable  : int?
