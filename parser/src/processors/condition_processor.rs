use crate::parser;
use crate::processors;
use crate::syntax::condition;
use ellie_core::error;

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub fn collect_condition<F>(
    parser: &mut parser::Parser<F>,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
{
    let parser_clone = parser.clone();
    if let parser::Collecting::Condition(ref mut data) = parser.current {
        if !data.initialized {
            if last_char == "i" && letter_char == "f" {
                data.initialized = true;
                //TODO BROKEN
                //data.cloak_pos.range_start.0 = parser.pos.0; //Function naming started so we set the position
                //data.keyword_pos.range_start.0 = parser.pos.0 - 1; //Function naming started so we set the position
                //data.keyword_pos.range_end.0 = parser.pos.0; //Function naming started so we set the position
            }
        } else if !data.cloak_collected {
            if data.cloak_itered_data.data.value.is_type_complete() && letter_char == "{" {
                data.cloak_collected = true;
                let chain_length = if data.chains.is_empty() {
                    0
                } else {
                    data.chains.len() - 1
                };

                if chain_length == 0 {
                    data.chains.push(condition::ConditionChain::default());
                }

                data.chains[chain_length].condition =
                    Box::new(data.cloak_itered_data.data.value.clone());
            } else {
                let collected = processors::value_processor::collect_value(
                    parser_clone,
                    &mut data.cloak_itered_data,
                    letter_char,
                    next_char,
                    last_char,
                );
                for i in collected.errors {
                    errors.push(i)
                }
                data.cloak_itered_data = collected.itered_data;
            }
        } else if letter_char == "}" {
            if data.inside_object_start {
                if data.inside_object_count == 0 {
                    data.inside_object_start = true;
                } else {
                    data.inside_object_count -= 1;
                }
            } else {
                let mut child_parser = parser::Parser::new(
                    data.inside_code_string.clone(),
                    |_, _, _| parser::ResolvedImport::default(),
                    parser.emit_message.clone(),
                    parser.options.clone(),
                );
                child_parser.pos = parser.pos;
                let mapped = child_parser.map();
                for i in mapped.syntax_errors {
                    errors.push(i)
                }
                let chains_length = data.chains.clone().len() - 1;
                data.chains[chains_length].inside_code = mapped.parsed.items;
                parser.collected.push(parser.current.clone());
                parser.current = parser::Collecting::None;
            }
        } else {
            let code_letter = if last_char.clone() == "\n" || last_char.clone() == "\r" {
                last_char + letter_char //Make sure we get the lines correctly
            } else {
                letter_char.to_string()
            };
            data.inside_code_string += &code_letter;
        }
    }
}
