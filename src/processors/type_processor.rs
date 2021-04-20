use crate::mapper;
use crate::syntax::{condition, function, variable};

pub fn collect(
    mapper: &mut mapper::Mapper,
    _letter_char: &str,
    _next_char: String,
    _next_next_char: String,
    _next_next_next_char: String,
) {
    let keyword = crate::utils::trim_good(mapper.keyword_catch.trim_start().to_string()); //one step next

    println!(
        "{:#?}:{:#?} | {}",
        keyword,
        keyword == "else if",
        keyword == "else if"
    );
    //println!("{:#?}", keyword);

    if keyword == "v " {
        println!("Variable started");
        mapper.current = mapper::Collecting::Variable(variable::VariableCollector {
            initialized: true,
            data: variable::Variable {
                pos: mapper::defs::Cursor {
                    range_start: mapper.pos,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });
    } else if keyword == "d " {
        println!("Dynamic Variable Started");
        mapper.current = mapper::Collecting::Variable(variable::VariableCollector {
            initialized: true,
            data: variable::Variable {
                dynamic: true,
                pos: mapper::defs::Cursor {
                    range_start: mapper.pos,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });
    } else if keyword == "fn " {
        println!("Function started");
        mapper.current = mapper::Collecting::Function(function::FunctionCollector::default());
    } else if keyword == "if" {
        println!("IF {:#?}", _letter_char);
        mapper.current = mapper::Collecting::Condition(condition::ConditionCollector::default());
    } else if keyword == "else if" {
        println!("ELSE IF");
        let collected_length = mapper.collected.clone().len();
        if collected_length == 0 {
            panic!("Error");
        } else if let mapper::Collecting::Condition(value) =
            &mut mapper.collected[collected_length - 1]
        {
            let mut repeated_condition = condition::ConditionCollector {
                chains: value.chains.clone(),
                initialized: true,
                cloak_collected: false,
                cloak_pos: mapper::defs::Cursor {
                    range_start: mapper::defs::CursorPosition(mapper.pos.0, mapper.pos.0 + 1),
                    ..Default::default()
                },
                keyword_pos: mapper::defs::Cursor {
                    range_start: mapper::defs::CursorPosition(mapper.pos.0 - 1, mapper.pos.0),
                    range_end: mapper::defs::CursorPosition(mapper.pos.0, mapper.pos.0 + 1),
                },
                ..Default::default()
            };
            repeated_condition.chains.push(condition::ConditionChain {
                r#type: condition::ConditionType::ElseIf,
                ..Default::default()
            });
            mapper.current = mapper::Collecting::Condition(repeated_condition);
            mapper.collected.remove(collected_length - 1);
            println!("ELSE IF {:#?}", mapper);
        } else {
            //User used else statement without if
            panic!("Error: {:#?}", mapper.collected);
        }
    } else if keyword == "else" {
        
    }
}
