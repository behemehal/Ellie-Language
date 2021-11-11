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
    pub raw_size: String,
    pub at_comma: bool,
    pub type_collected: bool,
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
    pub child_cache: Box<DefinerProcessor>,
}

#[derive(Default, Clone, Debug)]
pub struct NullableType {
    pub rtype: Box<DefinerTypes>,
    pub child_cache: Box<DefinerProcessor>,
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

impl Processor<bool> for DefinerProcessor {
    fn new() -> Self {
        DefinerProcessor::default()
    }

    fn keyword(&self) -> &str {
        ""
    }

    fn is_forwarded() -> bool {
        false
    }

    fn has_accessibility(&self) -> bool {
        false
    }

    fn iterate(&mut self, cursor: defs::CursorPosition, last_char: char, letter_char: char) {
        match self.definer_type {
            DefinerTypes::Cloak(ref mut cloak_type) => {
                if cloak_type.child_cache.complete && letter_char == ',' && !cloak_type.at_comma {
                    cloak_type.at_comma = true;
                    let entries_len = cloak_type.entries.len();
                    if entries_len == 0 {
                        cloak_type
                            .entries
                            .push(cloak_type.child_cache.definer_type.clone())
                    } else {
                        cloak_type.entries[entries_len - 1] =
                            cloak_type.child_cache.definer_type.clone();
                    }

                    cloak_type.child_cache = Box::new(DefinerProcessor::default());
                    cloak_type
                        .entries
                        .push(DefinerTypes::Generic(GenericType::default()));
                } else if cloak_type.child_cache.complete
                    && letter_char == ')'
                    && !cloak_type.at_comma
                {
                    let entries_len = cloak_type.entries.len();
                    if entries_len == 0 {
                        cloak_type
                            .entries
                            .push(cloak_type.child_cache.definer_type.clone())
                    } else {
                        cloak_type.entries[entries_len - 1] =
                            cloak_type.child_cache.definer_type.clone();
                    }
                    cloak_type.child_cache = Box::new(DefinerProcessor::default());
                    self.complete = true;
                } else {
                    cloak_type.at_comma = false;
                    cloak_type
                        .child_cache
                        .iterate(cursor, last_char, letter_char);
                    self.errors.extend(cloak_type.child_cache.errors.clone());
                }
            }
            DefinerTypes::Array(ref mut array_type) => {
                if !array_type.type_collected {
                    if array_type.child_cache.is_complete() && letter_char == ',' {
                        array_type.type_collected = true;
                        array_type.rtype = Box::new(array_type.child_cache.definer_type.clone());
                        array_type.child_cache = Box::new(Processor::new());
                    } else {
                        array_type
                            .child_cache
                            .iterate(cursor, last_char, letter_char);
                        self.errors.extend(array_type.child_cache.errors.clone());
                    }
                } else {
                    let is_num =
                        (array_type.size.to_string() + &letter_char.to_string()).parse::<isize>();

                    if let Ok(num) = is_num {
                        if last_char == ' ' && array_type.raw_size != "" {
                            self.errors.push(error::errorList::error_s1.clone().build(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: letter_char.to_string(),
                                }],
                                "0x00172".to_owned(),
                                defs::Cursor {
                                    range_start: cursor,
                                    range_end: cursor.clone().skip_char(1),
                                },
                            ));
                        } else if num.is_negative() || array_type.raw_size == "-" {
                            self.errors.push(error::errorList::error_s20.clone().build(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: "array".to_string(),
                                }],
                                "0x00184".to_owned(),
                                defs::Cursor {
                                    range_start: cursor,
                                    range_end: cursor.clone().skip_char(1),
                                },
                            ));
                        } else {
                            array_type.raw_size += &letter_char.to_string();
                            array_type.size = num as usize;
                        }
                    } else {
                        if letter_char == '*' && array_type.raw_size == "" {
                            self.definer_type = DefinerTypes::Vector(VectorType {
                                rtype: array_type.rtype.clone(),
                            });
                        } else if letter_char == ']' && array_type.raw_size != "" {
                            self.complete = true;
                        } else if letter_char != ' ' {
                            if letter_char == '-' && array_type.raw_size == "" {
                                array_type.raw_size = "-".to_string();
                            } else {
                                self.errors.push(error::errorList::error_s1.clone().build(
                                    vec![error::ErrorBuildField {
                                        key: "token".to_string(),
                                        value: letter_char.to_string(),
                                    }],
                                    "0x00210".to_owned(),
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
                        self.errors
                            .extend(collective_type.child_cache.errors.clone());
                    }
                } else {
                    collective_type
                        .child_cache
                        .iterate(cursor, last_char, letter_char);
                    if collective_type.child_cache.is_complete() && letter_char == '}' {
                        self.complete = true;
                        collective_type.value =
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
                            key: "token".to_string(),
                            value: letter_char.to_string(),
                        }],
                        "0x00257".to_owned(),
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
            DefinerTypes::Future(ref mut future_type) => {
                if letter_char == ' ' && last_char == '>' {
                    self.errors.push(error::errorList::error_s1.clone().build(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: letter_char.to_string(),
                        }],
                        "0x276".to_owned(),
                        defs::Cursor {
                            range_start: cursor,
                            range_end: cursor.clone().skip_char(1),
                        },
                    ));
                }
                future_type
                    .child_cache
                    .iterate(cursor, last_char, letter_char);
                self.errors.extend(future_type.child_cache.errors.clone());
                if future_type.child_cache.is_complete() {
                    self.complete = true;
                    future_type.rtype = Box::new(future_type.child_cache.definer_type.clone());
                }
            }
            DefinerTypes::Nullable(ref mut nullable_type) => {
                if letter_char == ' ' && last_char == '?' {
                    self.errors.push(error::errorList::error_s1.clone().build(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: letter_char.to_string(),
                        }],
                        "0x276".to_owned(),
                        defs::Cursor {
                            range_start: cursor,
                            range_end: cursor.clone().skip_char(1),
                        },
                    ));
                }
                nullable_type
                    .child_cache
                    .iterate(cursor, last_char, letter_char);
                self.errors.extend(nullable_type.child_cache.errors.clone());
                if nullable_type.child_cache.is_complete() {
                    self.complete = true;
                    nullable_type.rtype = Box::new(nullable_type.child_cache.definer_type.clone());
                }
            }
            DefinerTypes::Generic(ref mut generic_type) => {
                if reliable_char(&letter_char) {
                    if last_char == ' ' && generic_type.rtype != "" {
                        self.errors.push(error::errorList::error_s1.clone().build(
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }],
                            "0x022".to_owned(),
                            defs::Cursor {
                                range_start: cursor,
                                range_end: cursor.clone().skip_char(1),
                            },
                        ));
                    } else {
                        if generic_type.rtype.is_empty() {
                            self.complete = true;
                        }
                        generic_type.rtype += &letter_char.to_string();
                    }
                } else {
                    if letter_char == '>' && generic_type.rtype.is_empty() {
                        self.definer_type = DefinerTypes::Future(FutureType {
                            rtype: Box::new(self.definer_type.clone()),
                            ..Default::default()
                        });
                    } else if letter_char == '?' && generic_type.rtype.is_empty() {
                        self.definer_type = DefinerTypes::Nullable(NullableType {
                            rtype: Box::new(self.definer_type.clone()),
                            ..Default::default()
                        });
                    } else if letter_char == '{' && generic_type.rtype.is_empty() {
                        self.definer_type = DefinerTypes::Collective(CollectiveType::default());
                    } else if letter_char == '(' && generic_type.rtype.is_empty() {
                        self.definer_type = DefinerTypes::Cloak(CloakType::default());
                    } else if letter_char == '[' && generic_type.rtype.is_empty() {
                        self.definer_type = DefinerTypes::Array(ArrayType::default());
                    } else if letter_char != ' ' {
                        self.errors.push(error::errorList::error_s1.clone().build(
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }],
                            "0x00360".to_owned(),
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
        !self.errors.is_empty()
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
//Future    : >int
//Nullable  : ?int
