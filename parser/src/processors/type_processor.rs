use crate::parser;
use crate::syntax::{class, condition, constructor, function, ret, variable};
use ellie_core::{defs, error, utils};

use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub fn collect_type(
    parser: &mut parser::Parser,
    _errors: &mut Vec<error::Error>,
    _letter_char: &str,
    _next_char: String,
    _next_next_char: String,
    _next_next_next_char: String,
    options: defs::ParserOptions,
) {
    let keyword = utils::trim_good(parser.keyword_catch.trim_start().to_string()); //one step next

    if (keyword == "c " || keyword == "pub c " || keyword == "pri c ") && options.constants {
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
    } else if (keyword == "fn " || keyword == "pub fn" || keyword == "pri fn") && options.functions
    {
        parser.current = parser::Collecting::Function(function::FunctionCollector {
            data: function::Function {
                public: keyword == "pub fn ",
                ..Default::default()
            },
            ..Default::default()
        });
    } else if keyword == "co " && options.parser_type == defs::ParserType::ClassParser {
        parser.current =
            parser::Collecting::Constructor(constructor::ConstructorCollector::default());
    } else if keyword == "if" && options.parser_type == defs::ParserType::RawParser {
        parser.current = parser::Collecting::Condition(condition::ConditionCollector::default());
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
        } else {
            //User used else statement without if
            panic!("Error: {:#?}", parser.collected);
        }
    } else if keyword == "else {" && options.parser_type == defs::ParserType::RawParser {
        let collected_length = parser.collected.clone().len();
        if collected_length == 0 {
            panic!("Error");
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
    } else if keyword == "class " && options.parser_type == defs::ParserType::RawParser {
        parser.current = parser::Collecting::Class(class::ClassCollector::default());
    } else if keyword == "ret " && options.parser_type == defs::ParserType::RawParser {
        parser.current = parser::Collecting::Ret(ret::Ret {
            keyword_pos: defs::Cursor {
                range_start: parser.pos.clone().popChar(3),
                range_end: parser.pos.skipChar(1),
            },
            ..Default::default()
        });
    }
}
