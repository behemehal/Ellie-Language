use crate::processors::Processor;
use crate::syntax::items::definers::*;
use ellie_core::{defs, error, utils};

impl Processor for DefinerCollector {
    fn new() -> Self {
        DefinerCollector::default()
    }

    fn keyword(&self) -> &str {
        ""
    }

    fn has_accessibility(&self) -> bool {
        false
    }

    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) {
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

                    cloak_type.child_cache = Box::new(DefinerCollector::default());
                    cloak_type
                        .entries
                        .push(DefinerTypes::Generic(GenericType::default()));
                } else if (cloak_type.child_cache.complete || !cloak_type.not_empty)
                    && letter_char == ')'
                    && !cloak_type.at_comma
                {
                    let entries_len = cloak_type.entries.len();
                    if entries_len == 0 {
                        if cloak_type.not_empty {
                            cloak_type
                                .entries
                                .push(cloak_type.child_cache.definer_type.clone())
                        }
                    } else {
                        cloak_type.entries[entries_len - 1] =
                            cloak_type.child_cache.definer_type.clone();
                    }
                    cloak_type.child_cache = Box::new(DefinerCollector::default());
                    self.complete = true;
                } else {
                    cloak_type.not_empty = true;
                    cloak_type.at_comma = false;
                    cloak_type
                        .child_cache
                        .iterate(errors, cursor, last_char, letter_char);
                }
            }
            DefinerTypes::Array(ref mut array_type) => {
                if !array_type.type_collected {
                    if array_type.child_cache.complete && letter_char == ',' {
                        array_type.type_collected = true;
                        array_type.rtype = Box::new(array_type.child_cache.definer_type.clone());
                        array_type.child_cache = Box::new(Processor::new());
                    } else {
                        array_type
                            .child_cache
                            .iterate(errors, cursor, last_char, letter_char);
                    }
                } else {
                    let is_num =
                        (array_type.size.to_string() + &letter_char.to_string()).parse::<isize>();

                    if let Ok(num) = is_num {
                        if last_char == ' ' && array_type.raw_size != "" {
                            errors.push(error::errorList::error_s1.clone().build(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: letter_char.to_string(),
                                }],
                                "0x00172".to_owned(),
                                defs::Cursor::build_with_skip_char(cursor),
                            ));
                        } else if num.is_negative() || array_type.raw_size == "-" {
                            errors.push(error::errorList::error_s20.clone().build(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: "array".to_string(),
                                }],
                                "0x00184".to_owned(),
                                defs::Cursor::build_with_skip_char(cursor),
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
                                errors.push(error::errorList::error_s1.clone().build(
                                    vec![error::ErrorBuildField {
                                        key: "token".to_string(),
                                        value: letter_char.to_string(),
                                    }],
                                    "0x00210".to_owned(),
                                    defs::Cursor::build_with_skip_char(cursor),
                                ));
                            }
                        }
                    }
                }
            }
            DefinerTypes::Collective(ref mut collective_type) => {
                if !collective_type.key_collected {
                    if collective_type.child_cache.complete && letter_char == ',' {
                        collective_type.key_collected = true;
                        collective_type.key =
                            Box::new(collective_type.child_cache.definer_type.clone());
                        collective_type.child_cache = Box::new(Processor::new());
                    } else {
                        collective_type
                            .child_cache
                            .iterate(errors, cursor, last_char, letter_char);
                    }
                } else {
                    if collective_type.child_cache.complete && letter_char == '}' {
                        self.complete = true;
                        collective_type.value =
                            Box::new(collective_type.child_cache.definer_type.clone());
                        collective_type.child_cache = Box::new(Processor::new());
                    } else {
                        collective_type
                            .child_cache
                            .iterate(errors, cursor, last_char, letter_char);
                    }
                }
            }
            DefinerTypes::Vector(_) => {
                //Vector type resolved in array, if another character is found after brace
                //it's a syntax error
                if self.complete || letter_char != ']' {
                    //If brace is already put or char is not close brace
                    errors.push(error::errorList::error_s1.clone().build(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: letter_char.to_string(),
                        }],
                        "0x00257".to_owned(),
                        defs::Cursor::build_with_skip_char(cursor),
                    ));
                } else if letter_char == ']' {
                    //After we see '*' char in array size position, we update
                    //Type as vector, so completing array brace is up to vector
                    self.complete = true;
                }
            }
            DefinerTypes::Future(ref mut future_type) => {
                if letter_char == ' ' && last_char == '>' {
                    errors.push(error::errorList::error_s1.clone().build(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: letter_char.to_string(),
                        }],
                        "0x276".to_owned(),
                        defs::Cursor::build_with_skip_char(cursor),
                    ));
                }
                future_type
                    .child_cache
                    .iterate(errors, cursor, last_char, letter_char);
                if future_type.child_cache.complete {
                    self.complete = true;
                    future_type.rtype = Box::new(future_type.child_cache.definer_type.clone());
                }
            }
            DefinerTypes::Nullable(ref mut nullable_type) => {
                if letter_char == ' ' && last_char == '?' {
                    errors.push(error::errorList::error_s1.clone().build(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: letter_char.to_string(),
                        }],
                        "0x276".to_owned(),
                        defs::Cursor::build_with_skip_char(cursor),
                    ));
                }
                nullable_type
                    .child_cache
                    .iterate(errors, cursor, last_char, letter_char);
                if nullable_type.child_cache.complete {
                    self.complete = true;
                    nullable_type.rtype = Box::new(nullable_type.child_cache.definer_type.clone());
                }
            }
            DefinerTypes::Generic(ref mut generic_type) => {
                if utils::reliable_name_range(utils::ReliableNameRanges::Type, letter_char).reliable
                {
                    if last_char == ' ' && generic_type.rtype != "" {
                        errors.push(error::errorList::error_s1.clone().build(
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }],
                            "0x022".to_owned(),
                            defs::Cursor::build_with_skip_char(cursor),
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
                    } else if letter_char == '@' && generic_type.rtype.is_empty() {
                        self.definer_type = DefinerTypes::Function(FunctionType::default());
                    } else if letter_char == '(' && generic_type.rtype.is_empty() {
                        self.definer_type = DefinerTypes::Cloak(CloakType::default());
                    } else if letter_char == '[' && generic_type.rtype.is_empty() {
                        self.definer_type = DefinerTypes::Array(ArrayType::default());
                    } else if letter_char != ' ' {
                        errors.push(error::errorList::error_s1.clone().build(
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }],
                            "0x00360".to_owned(),
                            defs::Cursor::build_with_skip_char(cursor),
                        ));
                    }
                }
            }
            DefinerTypes::Function(ref mut function_type) => {
                if !function_type.parameters_collected {
                    if function_type.brace_stared {
                        if function_type.child_cache.complete
                            && letter_char == ','
                            && !function_type.at_comma
                        {
                            function_type.at_comma = true;
                            let params_len = function_type.params.len();
                            if params_len == 0 {
                                function_type
                                    .params
                                    .push(function_type.child_cache.definer_type.clone())
                            } else {
                                function_type.params[params_len - 1] =
                                    function_type.child_cache.definer_type.clone();
                            }

                            function_type.child_cache = Box::new(DefinerCollector::default());
                            function_type
                                .params
                                .push(DefinerTypes::Generic(GenericType::default()));
                        } else if (function_type.child_cache.complete || !function_type.not_empty)
                            && letter_char == ')'
                            && !function_type.at_comma
                        {
                            let params_len = function_type.params.len();
                            if params_len == 0 {
                                if function_type.not_empty {
                                    function_type
                                        .params
                                        .push(function_type.child_cache.definer_type.clone())
                                }
                            } else {
                                function_type.params[params_len - 1] =
                                    function_type.child_cache.definer_type.clone();
                            }
                            function_type.child_cache = Box::new(DefinerCollector::default());
                            function_type.parameters_collected = true;
                            function_type.returning =
                                Box::new(DefinerTypes::Generic(GenericType {
                                    rtype: "void".to_owned(),
                                }));
                            self.complete = true;
                        } else {
                            function_type.not_empty = true;
                            function_type.at_comma = false;
                            function_type.child_cache.iterate(
                                errors,
                                cursor,
                                last_char,
                                letter_char,
                            );
                        }
                    } else {
                        if letter_char == '(' {
                            function_type.brace_stared = true;
                        } else {
                            errors.push(error::errorList::error_s1.clone().build(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: letter_char.to_string(),
                                }],
                                "0x00267".to_owned(),
                                defs::Cursor::build_with_skip_char(cursor),
                            ));
                        }
                    }
                } else {
                    if !function_type.return_char_typed {
                        if letter_char == ':' {
                            self.complete = false;
                            function_type.return_char_typed = true;
                            function_type.returning = Box::new(DefinerTypes::default());
                        } else {
                            errors.push(error::errorList::error_s1.clone().build(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: letter_char.to_string(),
                                }],
                                "0x00267".to_owned(),
                                defs::Cursor::build_with_skip_char(cursor),
                            ));
                        }
                    } else {
                        function_type
                            .child_cache
                            .iterate(errors, cursor, last_char, letter_char);
                        function_type.returning =
                            Box::new(function_type.child_cache.definer_type.clone());
                        self.complete = function_type.child_cache.complete;
                    }
                }
            }
        }
    }
}

//Cloak     : (int, string)
//Collective: {int, string}
//Array     : [int, 3]
//Vector    : [int, *]
//Future    : >int
//Nullable  : ?int
