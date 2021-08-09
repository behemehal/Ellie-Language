use crate::alloc::string::String;
use crate::alloc::vec::Vec;
use crate::parser;
use crate::processors;
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;
use ellie_core::error;

pub fn collect_for(
    parser: &mut parser::Parser,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) {
    let parser_clone = parser.clone();
    if let parser::Collecting::ForLoop(ref mut for_loop_data) = parser.current {
        //panic!("NOT IMPLEMENTED");

        if !for_loop_data.parameters_collected {
            if for_loop_data
                .cloak_itered_data
                .data
                .value
                .is_type_complete()
                && letter_char == "{"
            {
                for_loop_data.parameters_collected = true;

                let deep_call = parser_clone
                    .resolve_deep_call(for_loop_data.cloak_itered_data.data.value.clone());

                if !parser_clone.is_iterable(deep_call) {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "ef813b31e7f9edb1a41ad3c24b042ad1".to_string(),
                        title: error::errorList::error_s29.title.clone(),
                        code: error::errorList::error_s29.code,
                        message: error::errorList::error_s29.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s29.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: for_loop_data.cloak_itered_data.data.value.get_type(),
                            }],
                        ),
                        pos: for_loop_data.data.parameter_pos,
                    });
                }

                for_loop_data.data.parameter =
                    Box::new(for_loop_data.cloak_itered_data.data.value.clone());
            } else {
                if letter_char != " " && for_loop_data.data.parameter_pos.range_start.is_zero() {
                    for_loop_data.data.parameter_pos.range_start = parser_clone.pos.clone();
                }
                for_loop_data.data.parameter_pos.range_end = parser_clone.pos.clone();

                let collected = processors::value_processor::collect_value(
                    parser_clone,
                    &mut for_loop_data.cloak_itered_data,
                    letter_char,
                    next_char,
                    last_char,
                );
                for i in collected.errors {
                    errors.push(i)
                }

                for_loop_data.cloak_itered_data = collected.itered_data;
            }
        } else if letter_char == "}" {
            if for_loop_data.inside_object_start {
                if for_loop_data.inside_object_count == 0 {
                    for_loop_data.inside_object_start = true;
                } else {
                    for_loop_data.inside_object_count -= 1;
                }
            } else {
                let mut child_parser = parser::Parser::new(
                    for_loop_data.inside_code_string.clone(),
                    |_, _| parser::ResolvedImport::default(),
                    parser.options.clone(),
                );
                child_parser.pos = parser.pos;
                let mapped = child_parser.map();
                for i in mapped.syntax_errors {
                    errors.push(i)
                }
                for_loop_data.data.pos.range_end = parser.pos.clone().skip_char(1);
                for_loop_data.inside_code = mapped.parsed.items;
                parser.collected.push(parser.current.clone());
                parser.current = parser::Collecting::None;
            }
        } else {
            let code_letter = if last_char.clone() == "\n" || last_char.clone() == "\r" {
                last_char + letter_char //Make sure we get the lines correctly
            } else {
                letter_char.to_string()
            };

            for_loop_data.inside_code_string += &code_letter;
        }
    }
}
