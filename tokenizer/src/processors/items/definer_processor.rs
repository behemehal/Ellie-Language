use crate::processors::types::{Processors, TypeProcessor};
use crate::syntax::items::definers::*;
use ellie_core::{defs, error, utils};

impl crate::processors::Processor for DefinerCollector {
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
                        array_type.child_cache = Box::new(DefinerCollector::default());
                    } else {
                        array_type
                            .child_cache
                            .iterate(errors, cursor, last_char, letter_char);
                    }
                } else if !array_type.size_collected {
                    if array_type.size_child_cache.is_complete() && letter_char == ']' {
                        array_type.size =
                            Box::new(array_type.size_child_cache.current.to_definite());
                        array_type.size_child_cache = Box::new(TypeProcessor::default());
                        self.complete = true;
                    } else if matches!(array_type.size_child_cache.current.clone(), Processors::Variable(x) if x.data.value == "")
                        && letter_char == '*'
                    {
                        self.definer_type = DefinerTypes::Vector(VectorType {
                            rtype: array_type.rtype.clone(),
                            pos: array_type.pos.clone(),
                        });
                    } else {
                        array_type
                            .size_child_cache
                            .iterate(errors, cursor, last_char, letter_char);
                        if array_type.size_child_cache.is_complete() {
                            array_type.size =
                                Box::new(array_type.size_child_cache.current.to_definite());
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
                        collective_type.child_cache = Box::new(DefinerCollector::default());
                    } else {
                        collective_type
                            .child_cache
                            .iterate(errors, cursor, last_char, letter_char);
                    }
                } else {
                    if collective_type.child_cache.complete && letter_char == '}' {
                        collective_type.pos.range_end = cursor.clone().skip_char(1);
                        self.complete = true;
                        collective_type.value =
                            Box::new(collective_type.child_cache.definer_type.clone());
                        collective_type.child_cache = Box::new(DefinerCollector::default());
                    } else {
                        collective_type
                            .child_cache
                            .iterate(errors, cursor, last_char, letter_char);
                    }
                }
            }
            DefinerTypes::Vector(ref mut vector_type) => {
                //Vector type resolved in array, if another character is found after brace
                //it's a syntax error
                if self.complete || letter_char != ']' {
                    //If brace is already put or char is not close brace
                    errors.push(error::error_list::ERROR_S1.clone().build(
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
                    vector_type.pos.range_end = cursor.clone().skip_char(1);
                    self.complete = true;
                }
            }
            DefinerTypes::Nullable(ref mut nullable_type) => {
                if letter_char == ' ' && last_char == '?' {
                    errors.push(error::error_list::ERROR_S1.clone().build(
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
            DefinerTypes::ParentGeneric(ref mut parent_generic_type) => {
                let len = parent_generic_type.generics.len();
                if letter_char == ',' && parent_generic_type.cache.complete {
                    parent_generic_type.generics[len - 1].value =
                        parent_generic_type.cache.definer_type.clone();
                    parent_generic_type
                        .generics
                        .push(GenericParameter::default());
                    parent_generic_type.cache = Box::new(DefinerCollector::default());
                } else if letter_char == '>' && parent_generic_type.cache.complete {
                    self.complete = true;
                    parent_generic_type.pos.range_end = cursor.clone().skip_char(1);
                    parent_generic_type.generics[len - 1].value =
                        parent_generic_type.cache.definer_type.clone();
                    parent_generic_type.cache = Box::new(DefinerCollector::default());
                } else {
                    if parent_generic_type.generics[len - 1]
                        .pos
                        .range_start
                        .is_zero()
                        && letter_char != ' '
                    {
                        parent_generic_type.generics[len - 1].pos.range_start = cursor;
                    }
                    parent_generic_type.generics[len - 1].pos.range_end =
                        cursor.clone().skip_char(1);
                    parent_generic_type
                        .cache
                        .iterate(errors, cursor, last_char, letter_char);
                }
            }
            DefinerTypes::Generic(ref mut generic_type) => {
                if utils::reliable_name_range(utils::ReliableNameRanges::Type, letter_char).reliable
                {
                    if last_char == ' ' && generic_type.rtype != "" {
                        errors.push(error::error_list::ERROR_S1.clone().build(
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }],
                            "0x022".to_owned(),
                            defs::Cursor::build_with_skip_char(cursor),
                        ));
                    } else {
                        if generic_type.rtype.is_empty() {
                            generic_type.pos.range_start = cursor;
                            self.complete = true;
                        }
                        generic_type.pos.range_end = cursor;
                        generic_type.rtype += &letter_char.to_string();
                    }
                } else {
                    if letter_char == '?' && generic_type.rtype.is_empty() {
                        self.definer_type = DefinerTypes::Nullable(NullableType {
                            pos: defs::Cursor::build_with_skip_char(cursor),
                            rtype: Box::new(self.definer_type.clone()),
                            ..Default::default()
                        });
                    } else if letter_char == '{' && generic_type.rtype.is_empty() {
                        self.definer_type = DefinerTypes::Collective(CollectiveType {
                            pos: defs::Cursor::build_with_skip_char(cursor.clone()),
                            ..Default::default()
                        });
                    } else if letter_char == '@' && generic_type.rtype.is_empty() {
                        self.definer_type = DefinerTypes::Function(FunctionType {
                            pos: defs::Cursor::build_with_skip_char(cursor.clone()),
                            ..Default::default()
                        });
                    } else if letter_char == '(' && generic_type.rtype.is_empty() {
                        self.definer_type = DefinerTypes::Cloak(CloakType {
                            pos: defs::Cursor::build_with_skip_char(cursor.clone()),
                            ..Default::default()
                        });
                    } else if letter_char == '[' && generic_type.rtype.is_empty() {
                        self.definer_type = DefinerTypes::Array(ArrayType {
                            pos: defs::Cursor::build_with_skip_char(cursor.clone()),
                            ..Default::default()
                        });
                    } else if letter_char == '<' && !generic_type.rtype.is_empty() {
                        self.definer_type = DefinerTypes::ParentGeneric(ParentGenericType {
                            parent: generic_type.rtype.clone(),
                            parent_pos: generic_type.pos.clone(),
                            pos: generic_type.pos.clone(),
                            generics: vec![GenericParameter::default()],
                            ..Default::default()
                        });
                        self.complete = false;
                    } else if letter_char != ' ' {
                        errors.push(error::error_list::ERROR_S1.clone().build(
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
                                    ..Default::default()
                                }));
                            function_type.pos.range_end = cursor.clone().skip_char(1);

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
                            errors.push(error::error_list::ERROR_S1.clone().build(
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
                            errors.push(error::error_list::ERROR_S1.clone().build(
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
                        function_type.pos.range_end = cursor.clone().skip_char(1);
                    }
                }
            }
            DefinerTypes::Dynamic => panic!("Unexpected behaviour"),
        }
    }
}

//Cloak     : (int, string)
//Collective: {int, string}
//Array     : [int, 3]
//Vector    : [int, *]
//Future    : >int
//Nullable  : ?int
