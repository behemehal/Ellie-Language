use crate::parser;
use crate::syntax::{
    caller, class, condition, constructor, file_key, function, import, ret, types, variable,
};
use ellie_core::{defs, error, utils};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

/*

*/

pub fn collect_type(
    parser: &mut parser::Parser,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    last_char: String,
    next_char: String,
) {
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
    } else if (keyword == "import " || keyword == "pub import " || keyword == "pri import ")
        && parser.options.allow_import
    {
        parser.current = parser::Collecting::Import(import::Import {
            public: keyword == "pub import ",
            pri_keyword: keyword == "pri import ",
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
                    range_start: parser.pos,
                    ..Default::default()
                },
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
                    range_start: parser.pos,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });
        parser.keyword_catch = String::new();
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
                    range_start: parser.pos,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });
        parser.keyword_catch = String::new();
    } else if (keyword == "fn " || keyword == "pub fn" || keyword == "pri fn")
        && parser.options.functions
    {
        parser.current = parser::Collecting::Function(function::FunctionCollector {
            data: function::Function {
                public: keyword == "pub fn ",
                ..Default::default()
            },
            ..Default::default()
        });
        parser.keyword_catch = String::new();
    } else if keyword == "co " && parser.options.parser_type == defs::ParserType::ClassParser {
        parser.current =
            parser::Collecting::Constructor(constructor::ConstructorCollector::default());
        parser.keyword_catch = String::new();
    } else if keyword == "@" && parser.options.parser_type == defs::ParserType::RawParser {
        parser.current = parser::Collecting::FileKey(file_key::FileKeyCollector::default());
        parser.keyword_catch = String::new();
    } else if keyword == "if" && parser.options.parser_type == defs::ParserType::RawParser {
        parser.current = parser::Collecting::Condition(condition::ConditionCollector::default());
        parser.keyword_catch = String::new();
    } else if keyword == "else if" && parser.options.parser_type == defs::ParserType::RawParser {
        let collected_length = parser.collected.clone().len();
        if collected_length == 0 {
            panic!("Error");
        } else if let parser::Collecting::Condition(value) =
            &mut parser.collected[collected_length - 1]
        {
            let mut repeated_condition = condition::ConditionCollector {
                chains: value.chains.clone(),
                initialized: true,
                cloak_collected: false,
                cloak_pos: defs::Cursor {
                    range_start: defs::CursorPosition(parser.pos.0, parser.pos.0 + 1),
                    ..Default::default()
                },
                keyword_pos: defs::Cursor {
                    range_start: defs::CursorPosition(parser.pos.0 - 1, parser.pos.0),
                    range_end: defs::CursorPosition(parser.pos.0, parser.pos.0 + 1),
                },
                ..Default::default()
            };
            repeated_condition.chains.push(condition::ConditionChain {
                rtype: condition::ConditionType::ElseIf,
                ..Default::default()
            });
            parser.current = parser::Collecting::Condition(repeated_condition);
            parser.collected.remove(collected_length - 1);
        } else {
            //User used else statement without if
            panic!("Error: {:#?}", parser.collected);
        }
        parser.keyword_catch = String::new();
    } else if keyword == "else {" && parser.options.parser_type == defs::ParserType::RawParser {
        let collected_length = parser.collected.clone().len();
        if collected_length == 0 {
            errors.push(error::Error {
                scope: "definer_processor".to_string(),
                debug_message: "ae03d887fdb59a68af1a0ac0264a5c7f".to_string(),
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
                chains: value.chains.clone(),
                initialized: true,
                cloak_collected: true,
                keyword_pos: defs::Cursor {
                    range_start: defs::CursorPosition(parser.pos.0 - 1, parser.pos.0),
                    range_end: defs::CursorPosition(parser.pos.0, parser.pos.0 + 1),
                },
                ..Default::default()
            };
            repeated_condition.chains.push(condition::ConditionChain {
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
    } else if keyword == "new " {
        parser.current = parser::Collecting::Caller(caller::Caller {
            value: types::Types::ClassCall(types::class_call::ClassCallCollector {
                keyword_collected: true,
                ..Default::default()
            }),
            pos: defs::Cursor {
                range_start: parser.pos,
                ..Default::default()
            },
        });
        parser.keyword_catch = String::new();
    } else if letter_char == "(" && keyword.trim() != "(" && !keyword.trim().is_empty() {
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
    } else if next_char == "." && keyword.trim() != "" {
        #[cfg(feature = "std")]
        std::println!("[ParserWarning]: Appliying no position data to VariableType[226] will cause error showing problem in cli");
        parser.current = parser::Collecting::Caller(caller::Caller {
            value: types::Types::Refference(types::refference_type::RefferenceType {
                refference: Box::new(types::Types::VariableType(
                    types::variable_type::VariableType {
                        value: keyword.clone(),
                        value_complete: true,
                        ..Default::default()
                    },
                )),
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
