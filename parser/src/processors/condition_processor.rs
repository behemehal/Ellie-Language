use crate::parser;
use crate::processors;
use crate::syntax::condition;
use alloc::boxed::Box;
use alloc::vec::Vec;
use ellie_core::error;

pub fn collect_condition<F>(
    parser: &mut parser::Parser<F>,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
{
    let parser_clone = parser.clone();
    if let parser::Collecting::Condition(ref mut condition_data) = parser.current {
        if !condition_data.initialized {
            if last_char == "i" && letter_char == "f" {
                condition_data.initialized = true;
                //TODO BROKEN
                //data.cloak_pos.range_start.0 = parser.pos.0; //Function naming started so we set the position
                //data.keyword_pos.range_start.0 = parser.pos.0 - 1; //Function naming started so we set the position
                //data.keyword_pos.range_end.0 = parser.pos.0; //Function naming started so we set the position
            }
        } else if !condition_data.cloak_collected {
            if condition_data
                .cloak_itered_data
                .data
                .value
                .is_type_complete()
                && letter_char == "{"
            {
                condition_data.cloak_collected = true;
                let chain_length = if condition_data.data.chains.is_empty() {
                    0
                } else {
                    condition_data.data.chains.len() - 1
                };

                if chain_length == 0 {
                    condition_data
                        .data
                        .chains
                        .push(condition::ConditionChain::default());
                }

                condition_data.data.chains[chain_length].condition =
                    Box::new(condition_data.cloak_itered_data.data.value.clone());
            } else {
                processors::value_processor::collect_value(
                    parser_clone,
                    &mut condition_data.cloak_itered_data,
                    errors,
                    letter_char,
                    next_char,
                    last_char,
                );
            }
        } else if letter_char == "}" {
            if condition_data.inside_object_start {
                if condition_data.inside_object_count == 0 {
                    condition_data.inside_object_start = true;
                } else {
                    condition_data.inside_object_count -= 1;
                }
            } else {
                let mut child_parser = parser::Parser::new(
                    condition_data.inside_code_string.clone(),
                    |_, _, _| parser::ResolvedImport::default(),
                    parser.emit_message.clone(),
                    parser.options.clone(),
                );
                child_parser.pos = parser.pos;
                let mapped = child_parser.map();
                for i in mapped.syntax_errors {
                    errors.push(i)
                }
                let chains_length = condition_data.data.chains.clone().len() - 1;
                condition_data.data.chains[chains_length].inside_code = mapped.parsed.items;
                parser.collected.push(parser.current.clone());
                parser.current = parser::Collecting::None;
            }
        } else {
            //let code_letter = if last_char.clone() == "\n" || last_char.clone() == "\r" {
            //    last_char + letter_char //Make sure we get the lines correctly
            //} else {
            //    letter_char.to_string()
            //};
            //condition_data.inside_code_string += &code_letter;
        }
    }
}
