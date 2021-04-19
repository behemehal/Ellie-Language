use crate::mapper;
use crate::syntax::{condition, function, variable};

pub fn collect(
    mapper: &mut mapper::Mapper,
    _letter_char: &str,
    next_char: String,
    _next_next_char: String,
    _next_next_next_char: String,
) {
    
    let keyword = crate::utils::trim_good(mapper.keyword_catch.trim_start().to_string()); //one step next
    
    println!("{:#?}:{:#?} | {}", keyword, keyword == "else if", keyword == "else if");
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
    } else if keyword == "if" || keyword == "if" {
        println!("IF {:#?}", _letter_char);
        mapper.current = mapper::Collecting::Condition(condition::ConditionCollector::default());
    } else if keyword == "else if" || keyword == "else if"{
        println!("ELSE IF");
        let collected_length = mapper.collected.clone().len();
        if collected_length == 0 {
            panic!("Error");
        } else if let mapper::Collecting::Condition(value) = &mut mapper.collected[collected_length - 1] {
            let mut repeated_condition = condition::ConditionCollector::default();
            repeated_condition.chains = value.chains.clone();
            repeated_condition.initialized = true;
            repeated_condition.initialized = true;
            repeated_condition.cloak_collected = false;
            repeated_condition.cloak_pos.range_start.0 = mapper.pos.0; //Function naming started so we set the position
            repeated_condition.keyword_pos.range_start.0 = mapper.pos.0 - 1; //Function naming started so we set the position
            repeated_condition.keyword_pos.range_end.0 = mapper.pos.0; //Function naming started so we set the position
            repeated_condition.chains.push(condition::ConditionChain {
                r#type: condition::ConditionType::ElseIf,
                ..Default::default()
            });
            repeated_condition.else_if_keyword_collector = "".to_string();
            repeated_condition.might_be_else_if = false;
            
            mapper.current = mapper::Collecting::Condition(repeated_condition);
            mapper.collected.remove(collected_length - 1);
            println!("ELSE IF {:#?}", mapper);
        } else {
            //User used else statement without if
            panic!("Error: {:#?}", mapper.collected);
        }
        /*
        if let mapper::Collecting::Condition(ref mut data) = mapper.current {
            data.initialized = true;
            data.cloak_collected = false;
            data.cloak_pos.range_start.0 = mapper.pos.0; //Function naming started so we set the position
            data.keyword_pos.range_start.0 = mapper.pos.0 - 1; //Function naming started so we set the position
            data.keyword_pos.range_end.0 = mapper.pos.0; //Function naming started so we set the position
            data.chains.push(condition::ConditionChain {
                r#type: condition::ConditionType::ElseIf,
                ..Default::default()
            });
            data.else_if_keyword_collector = "".to_string();
            data.might_be_else_if = false;
        }
        */
    } else if keyword == "else" || keyword == "else" {

    }
}
