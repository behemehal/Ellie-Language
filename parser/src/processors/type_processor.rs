use crate::parser;
use crate::syntax::{
    caller, class, condition, constructor, function, import, ret, types, variable,
};
use ellie_core::{defs, error, utils};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_type(
    parser: &mut parser::Parser,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    _last_char: String,
    next_char: String,
    options: defs::ParserOptions,
) {
    let keyword = utils::trim_good(parser.keyword_catch.trim_start().to_string()); //one step next

    if keyword == "*\\" && parser.on_comment && !parser.on_line_comment {
        parser.on_comment = false;
    } else if keyword == "/*" && !parser.on_comment && !parser.on_line_comment {
        parser.on_comment = true;
    } else if parser.on_comment {
    } else if (keyword == "import " || keyword == "pub import " || keyword == "pri import")
        && options.allow_import
    {
        if keyword == "pri import" {
            #[cfg(feature = "std")]
            std::println!(
                "[ParserInfo] imports are private in default, but use it anyway its your choice"
            )
        }

        parser.current = parser::Collecting::Import(import::Import {
            public: keyword == "pub import ",
            pos: defs::Cursor {
                range_start: parser.pos,
                ..Default::default()
            },
            ..Default::default()
        });
        parser.keyword_cache = variable::VariableCollector::default();
    } else if (keyword == "c " || keyword == "pub c " || keyword == "pri c ") && options.constants {
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
        parser.keyword_cache = variable::VariableCollector::default();
    } else if (keyword == "v " || keyword == "pub v " || keyword == "pri v ") && options.variables {
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
        parser.keyword_cache = variable::VariableCollector::default();
    } else if (keyword == "d " || keyword == "pub d " || keyword == "pri d ")
        && options.dynamics
        && options.variables
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
        parser.keyword_cache = variable::VariableCollector::default();
    } else if (keyword == "fn " || keyword == "pub fn" || keyword == "pri fn") && options.functions
    {
        parser.current = parser::Collecting::Function(function::FunctionCollector {
            data: function::Function {
                public: keyword == "pub fn ",
                ..Default::default()
            },
            ..Default::default()
        });
        parser.keyword_cache = variable::VariableCollector::default();
    } else if keyword == "co " && options.parser_type == defs::ParserType::ClassParser {
        parser.current =
            parser::Collecting::Constructor(constructor::ConstructorCollector::default());
        parser.keyword_cache = variable::VariableCollector::default();
    } else if keyword == "if" && options.parser_type == defs::ParserType::RawParser {
        parser.current = parser::Collecting::Condition(condition::ConditionCollector::default());
        parser.keyword_cache = variable::VariableCollector::default();
    } else if keyword == "else if" && options.parser_type == defs::ParserType::RawParser {
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
            parser.keyword_cache = variable::VariableCollector::default();
        } else {
            //User used else statement without if
            panic!("Error: {:#?}", parser.collected);
        }
    } else if keyword == "else {" && options.parser_type == defs::ParserType::RawParser {
        let collected_length = parser.collected.clone().len();
        if collected_length == 0 {
            errors.push(error::Error {
                scope: "definer_processor".to_string(),
                debug_message: "700312010d036bf0e3737e03c5b5484d".to_string(),
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
            parser.keyword_cache = variable::VariableCollector::default();
        } else {
            //User used else statement without if
            panic!("Error: {:#?}", parser.collected);
        }
    } else if keyword == "class " && options.parser_type == defs::ParserType::RawParser {
        parser.current = parser::Collecting::Class(class::ClassCollector::default());
        parser.keyword_cache = variable::VariableCollector::default();
    } else if keyword == "ret " && options.parser_type == defs::ParserType::RawParser {
        parser.current = parser::Collecting::Ret(ret::Ret {
            keyword_pos: defs::Cursor {
                range_start: parser.pos.clone().popChar(3),
                range_end: parser.pos.skipChar(1),
            },
            ..Default::default()
        });
        parser.keyword_cache = variable::VariableCollector::default();
    } else if letter_char == "(" && keyword.trim() != "(" && !keyword.trim().is_empty() {
        parser.current = parser::Collecting::Caller(caller::Caller {
            value: types::Types::FunctionCall(types::function_call::FunctionCallCollector {
                data: types::function_call::FunctionCall {
                    name: keyword.clone().replace("(", ""),
                    name_pos: defs::Cursor {
                        range_start: if keyword.clone().trim().len() - 1 > parser.pos.1 {
                            parser.pos
                        } else {
                            parser.pos.clone().popChar(keyword.clone().trim().len() - 1)
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
    } else if next_char == "." && keyword.trim() != "" {
        parser.current = parser::Collecting::Caller(caller::Caller {
            value: types::Types::Refference(types::refference_type::RefferenceType {
                refference: Box::new(types::Types::VariableType(
                    types::variable_type::VariableType {
                        value: keyword.clone(),
                        value_complete: true,
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
    }
}
