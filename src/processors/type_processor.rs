use crate::mapper;
use crate::syntax::{condition, function, variable};

pub fn collect(
    mapper: &mut mapper::Mapper,
    letter_char: &str,
    next_char: String,
    next_next_char: String,
    next_next_next_char: String,
) {
    if (letter_char == "d" && next_char == " ") && mapper.current == mapper::Collecting::None {
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
    } else if (letter_char == "v" && next_char == " ") && mapper.current == mapper::Collecting::None
    {
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
    } else if letter_char == "f"
        && next_char == "n"
        && next_next_char == " "
        && mapper.current == mapper::Collecting::None
    {
        mapper.current = mapper::Collecting::Function(function::FunctionCollector::default());
    } else if (letter_char == "e"
        && next_char == "l"
        && next_next_char == "s"
        && next_next_next_char == "e")
        && mapper.current == mapper::Collecting::None
    {
        let collected_length = mapper.collected.clone().len();
        if collected_length == 0 {
            panic!("Error");
        } else if let mapper::Collecting::Condition(value) =
            &mut mapper.collected[collected_length - 1]
        {
            let mut repeated_condition = condition::ConditionCollector::default();
            repeated_condition.chains = value.chains.clone();
            repeated_condition.might_be_else_if = true;
            mapper.current = mapper::Collecting::Condition(repeated_condition);
            mapper.collected.remove(collected_length - 1);
        } else {
            //User used else statement without if
            panic!("Error");
        }
    } else if (letter_char == "i" && next_char == "f" && next_next_char == " ")
        && mapper.current == mapper::Collecting::None
    {
        mapper.current = mapper::Collecting::Condition(condition::ConditionCollector {
            ..Default::default()
        });
    }
}
