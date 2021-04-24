use crate::parser;
use crate::syntax::{condition, function, variable};
use ellie_core::{defs, utils};

pub fn collect(
    parser: &mut parser::Parser,
    _letter_char: &str,
    _next_char: String,
    _next_next_char: String,
    _next_next_next_char: String,
) {
    let keyword = utils::trim_good(parser.keyword_catch.trim_start().to_string()); //one step next

    //println!("{:#?}", keyword);

    if keyword == "v " ||  keyword == "pub v" || keyword == "pri v" {
        println!("Variable started");
        parser.current = parser::Collecting::Variable(variable::VariableCollector {
            initialized: true,
            data: variable::Variable {
                pos: defs::Cursor {
                    range_start: parser.pos,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });
    } else if keyword == "d " || keyword == "pub d" || keyword == "pri d"{
        println!("Dynamic Variable Started");
        parser.current = parser::Collecting::Variable(variable::VariableCollector {
            initialized: true,
            data: variable::Variable {
                dynamic: true,
                pos: defs::Cursor {
                    range_start: parser.pos,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });
    } else if keyword == "fn " {
        println!("Function started");
        parser.current = parser::Collecting::Function(function::FunctionCollector::default());
    } else if keyword == "if" {
        println!("IF {:#?}", _letter_char);
        parser.current = parser::Collecting::Condition(condition::ConditionCollector::default());
    } else if keyword == "else if" {
        println!("ELSE IF");
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
                r#type: condition::ConditionType::ElseIf,
                ..Default::default()
            });
            parser.current = parser::Collecting::Condition(repeated_condition);
            parser.collected.remove(collected_length - 1);
            println!("ELSE IF {:#?}", parser);
        } else {
            //User used else statement without if
            panic!("Error: {:#?}", parser.collected);
        }
    } else if keyword == "else {" {
        println!("ELSE");
        let collected_length = parser.collected.clone().len();
        if collected_length == 0 {
            panic!("Error");
        } else if let parser::Collecting::Condition(value) = &mut parser.collected[collected_length - 1] {
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
                r#type: condition::ConditionType::Else,
                ..Default::default()
            });
            parser.current = parser::Collecting::Condition(repeated_condition);
            parser.collected.remove(collected_length - 1);
            println!("ELSE IF {:#?}", parser);
        } else {
            //User used else statement without if
            panic!("Error: {:#?}", parser.collected);
        }
    } else if keyword == "class " {
        println!("CLASS");
    }
}
