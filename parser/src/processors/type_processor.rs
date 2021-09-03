use crate::parser;
use crate::syntax::{
    caller, class, condition, constructor, file_key, for_loop, function, getter, import, ret,
    setter, types, variable,
};
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::{defs, error, utils};

pub fn collect_type<F>(
    parser: &mut parser::Parser<F>,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    last_char: String,
    next_char: String,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
{
    let keyword = utils::trim_good(parser.keyword_catch.trim_start().to_string()); //one step next

    if (letter_char == "*" && last_char == "/") && !parser.on_comment && !parser.on_line_comment {
        parser.on_comment = true;
    } else if (letter_char == "/" && last_char == "*")
        && parser.on_comment
        && !parser.on_line_comment
    {
        parser.on_comment = false;
        parser.keyword_catch = String::new();
    } else if parser.on_comment {
        parser.keyword_catch = String::new();
    } else if (keyword == "import "
        || keyword == "pub import "
        || keyword == "pri import "
        || keyword == "importNative "
        || keyword == "pub importNative "
        || keyword == "pri importNative ")
        && parser.options.allow_import
        && (parser.options.parser_type == defs::ParserType::RawParser
            || parser.options.parser_type == defs::ParserType::HeaderParser)
    {
        parser.current = parser::Collecting::Import(import::Import {
            public: keyword == "pub import " || keyword == "pub importNative ",
            pri_keyword: keyword == "pri import " || keyword == "pri importNative ",
            native: keyword == "importNative "
                || keyword == "pub importNative "
                || keyword == "pri importNative ",
            pos: defs::Cursor {
                range_start: parser.keyword_pos.range_start,
                ..Default::default()
            },
            ..Default::default()
        });
        parser.keyword_catch = String::new();
    } else if (keyword == "c " || keyword == "pub c " || keyword == "pri c ")
        && parser.options.constants
    {
        parser.current = parser::Collecting::Variable(variable::VariableCollector {
            initialized: true,
            data: variable::Variable {
                public: keyword == "pub c ",
                constant: true,
                pos: defs::Cursor {
                    range_start: parser.pos.pop_char(keyword.len() - 1),
                    ..Default::default()
                },
                hash: utils::generate_hash(),
                ..Default::default()
            },
            ..Default::default()
        });
        parser.keyword_catch = String::new();
    } else if (keyword == "v " || keyword == "pub v " || keyword == "pri v ")
        && parser.options.variables
    {
        parser.current = parser::Collecting::Variable(variable::VariableCollector {
            initialized: true,
            data: variable::Variable {
                public: keyword == "pub v ",
                pos: defs::Cursor {
                    range_start: parser.pos.pop_char(keyword.len() - 1),
                    ..Default::default()
                },
                hash: utils::generate_hash(),
                ..Default::default()
            },
            ..Default::default()
        });
        parser.keyword_catch = String::new();
    } else if (keyword == "s " || keyword == "pub s " || keyword == "pri s ")
        && parser.options.setters
    {
        parser.current = parser::Collecting::Setter(setter::SetterCollector::default());
    } else if (keyword == "g " || keyword == "pub g " || keyword == "pri g ")
        && parser.options.getters
    {
        parser.current = parser::Collecting::Getter(getter::GetterCollector::default());
    } else if (keyword == "d " || keyword == "pub d " || keyword == "pri d ")
        && parser.options.dynamics
        && parser.options.variables
    {
        parser.current = parser::Collecting::Variable(variable::VariableCollector {
            initialized: true,
            data: variable::Variable {
                dynamic: true,
                public: keyword == "pub d ",
                pos: defs::Cursor {
                    range_start: parser.pos.pop_char(keyword.len() - 1),
                    ..Default::default()
                },
                hash: utils::generate_hash(),
                ..Default::default()
            },
            ..Default::default()
        });
        parser.keyword_catch = String::new();
    } else if (keyword == "fn " || keyword == "pub fn " || keyword == "pri fn ")
        && parser.options.functions
    {
        parser.current = parser::Collecting::Function(function::FunctionCollector {
            data: function::Function {
                public: keyword == "pub fn ",
                pos: defs::Cursor {
                    range_start: parser.pos.pop_char(keyword.len() - 1),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });
        parser.keyword_catch = String::new();
    } else if keyword == "co " && parser.options.parser_type == defs::ParserType::ClassParser {
        parser.current = parser::Collecting::Constructor(constructor::ConstructorCollector {
            data: constructor::Constructor {
                pos: defs::Cursor {
                    range_start: parser.pos.pop_char(keyword.len() - 1),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });
        parser.keyword_catch = String::new();
    } else if keyword == "@"
        && (parser.options.parser_type == defs::ParserType::RawParser
            || parser.options.parser_type == defs::ParserType::HeaderParser)
    {
        parser.current = parser::Collecting::FileKey(file_key::FileKeyCollector {
            data: file_key::FileKey {
                pos: defs::Cursor {
                    range_start: parser.pos.pop_char(keyword.len() - 1),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });
        parser.keyword_catch = String::new();
    } else if keyword == "for " && parser.options.parser_type == defs::ParserType::RawParser {
        parser.current = parser::Collecting::ForLoop(for_loop::ForLoopCollector {
            data: for_loop::ForLoop {
                pos: defs::Cursor {
                    range_start: parser.pos.pop_char(keyword.len() - 1),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });
        parser.keyword_catch = String::new();
    } else if keyword == "if"
        && (parser.options.parser_type == defs::ParserType::RawParser
            || parser.options.parser_type == defs::ParserType::HeaderParser)
    {
        parser.current = parser::Collecting::Condition(condition::ConditionCollector::default());
        parser.keyword_catch = String::new();
    } else if keyword == "else if"
        && (parser.options.parser_type == defs::ParserType::RawParser
            || parser.options.parser_type == defs::ParserType::HeaderParser)
    {
        let collected_length = parser.collected.clone().len();
        if collected_length == 0 {
            panic!("Error");
        } else if let parser::Collecting::Condition(value) =
            &mut parser.collected[collected_length - 1]
        {
            let mut repeated_condition = condition::ConditionCollector {
                data: condition::Condition {
                    chains: value.data.chains.clone(),
                    cloak_pos: defs::Cursor {
                        range_start: defs::CursorPosition(parser.pos.0, parser.pos.0 + 1),
                        ..Default::default()
                    },
                    keyword_pos: defs::Cursor {
                        range_start: defs::CursorPosition(parser.pos.0 - 1, parser.pos.0),
                        range_end: defs::CursorPosition(parser.pos.0, parser.pos.0 + 1),
                    },
                },
                initialized: true,
                cloak_collected: false,
                ..Default::default()
            };
            repeated_condition
                .data
                .chains
                .push(condition::ConditionChain {
                    rtype: condition::ConditionType::ElseIf,
                    ..Default::default()
                });
            parser.current = parser::Collecting::Condition(repeated_condition);
            parser.collected.remove(collected_length - 1);
        } else {
            errors.push(error::Error {
                path: parser.options.path.clone(),
                scope: "definer_processor".to_string(),
                debug_message: "ed217387143f0e07294b32eff7448afe".to_string(),
                title: error::errorList::error_s1.title.clone(),
                code: error::errorList::error_s1.code,
                message: error::errorList::error_s1.message.clone(),
                builded_message: error::Error::build(
                    error::errorList::error_s1.message.clone(),
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: keyword,
                    }],
                ),
                pos: parser.keyword_pos,
            });
        }
        parser.keyword_catch = String::new();
    } else if keyword == "else {"
        && (parser.options.parser_type == defs::ParserType::RawParser
            || parser.options.parser_type == defs::ParserType::HeaderParser)
    {
        let collected_length = parser.collected.clone().len();
        if collected_length == 0 {
            errors.push(error::Error {
                path: parser.options.path.clone(),
                scope: "definer_processor".to_string(),
                debug_message: "44ead32f8245d7c8dd474e74a60528d5".to_string(),
                title: error::errorList::error_s1.title.clone(),
                code: error::errorList::error_s1.code,
                message: error::errorList::error_s1.message.clone(),
                builded_message: error::Error::build(
                    error::errorList::error_s1.message.clone(),
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: keyword,
                    }],
                ),
                pos: parser.keyword_pos,
            });
        } else if let parser::Collecting::Condition(value) =
            &mut parser.collected[collected_length - 1]
        {
            let mut repeated_condition = condition::ConditionCollector {
                data: condition::Condition {
                    chains: value.data.chains.clone(),
                    keyword_pos: defs::Cursor {
                        range_start: defs::CursorPosition(parser.pos.0 - 1, parser.pos.0),
                        range_end: defs::CursorPosition(parser.pos.0, parser.pos.0 + 1),
                    },
                    ..Default::default()
                },
                initialized: true,
                cloak_collected: true,
                ..Default::default()
            };
            repeated_condition
                .data
                .chains
                .push(condition::ConditionChain {
                    rtype: condition::ConditionType::Else,
                    ..Default::default()
                });
            parser.current = parser::Collecting::Condition(repeated_condition);
            parser.collected.remove(collected_length - 1);
        } else {
            //User used else statement without if
            panic!("Error: {:#?}", parser.collected);
        }
        parser.keyword_catch = String::new();
    } else if (keyword == "class " || keyword == "pub class " || keyword == "pri class ")
        && parser.options.parser_type == defs::ParserType::RawParser
    {
        parser.current = parser::Collecting::Class(class::ClassCollector {
            data: class::Class {
                public: keyword == "pub class ",
                pos: defs::Cursor {
                    range_start: parser.pos.pop_char(keyword.len() - 1),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });
        parser.keyword_catch = String::new();
    } else if keyword == "ret " && parser.options.parser_type == defs::ParserType::RawParser {
        parser.current = parser::Collecting::Ret(ret::Ret {
            keyword_pos: defs::Cursor {
                range_start: parser.pos.pop_char(3),
                range_end: parser.pos.clone().skip_char(1),
            },
            ..Default::default()
        });
        parser.keyword_catch = String::new();
    } else if keyword == "new " && parser.options.parser_type == defs::ParserType::RawParser {
        parser.current = parser::Collecting::Caller(caller::Caller {
            value: types::Types::ConstructedClass(
                types::constructed_class::ConstructedClassCollector {
                    data: types::constructed_class::ConstructedClass {
                        value: Box::new(types::Types::Null),
                        keyword_pos: defs::Cursor {
                            range_start: parser.pos.pop_char(3),
                            range_end: parser.pos.clone().skip_char(1),
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ),
            pos: defs::Cursor {
                range_start: parser.pos,
                ..Default::default()
            },
        });
        parser.keyword_catch = String::new();
    } else if letter_char == "("
        && keyword.trim() != "("
        && !keyword.trim().is_empty()
        && parser.options.parser_type == defs::ParserType::RawParser
    {
        parser.current = parser::Collecting::Caller(caller::Caller {
            value: types::Types::FunctionCall(types::function_call::FunctionCallCollector {
                data: types::function_call::FunctionCall {
                    name: keyword.clone().replace("(", ""),
                    name_pos: defs::Cursor {
                        range_start: if keyword.clone().trim().len() - 1 > parser.pos.1 {
                            parser.pos
                        } else {
                            parser
                                .pos
                                .clone()
                                .pop_char(keyword.clone().trim().len() - 1)
                        },
                        range_end: parser.pos,
                    },
                    ..Default::default()
                },
                ..Default::default()
            }),
            pos: defs::Cursor {
                range_start: parser.pos,
                ..Default::default()
            },
        });
        parser.keyword_catch = String::new();
    } else if next_char == "."
        && keyword.trim() != ""
        && parser.options.parser_type == defs::ParserType::RawParser
    {
        parser.current = parser::Collecting::Caller(caller::Caller {
            value: types::Types::Reference(types::reference_type::ReferenceTypeCollector {
                data: types::reference_type::ReferenceType {
                    reference: Box::new(types::Types::VariableType(
                        types::variable_type::VariableTypeCollector {
                            value_complete: true,
                            data: types::variable_type::VariableType {
                                value: keyword.clone(),
                                pos: parser.keyword_pos,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                    )),
                    reference_pos: parser.keyword_pos,
                    ..Default::default()
                },
                on_dot: true,
                ..Default::default()
            }),
            pos: defs::Cursor {
                range_start: parser.pos,
                ..Default::default()
            },
        });
        parser.keyword_catch = String::new();
    }
}
