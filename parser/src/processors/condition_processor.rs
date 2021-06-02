use crate::parser;
use crate::processors;
use crate::syntax::{condition, types};
use ellie_core::{defs, error};

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

pub fn collect_condition(
    parser: &mut parser::Parser,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
    options: defs::ParserOptions,
) {
    if let parser::Collecting::Condition(ref mut data) = parser.current {
        if !data.initialized {
            if last_char == "i" && letter_char == "f" {
                data.initialized = true;
                data.cloak_pos.range_start.0 = parser.pos.0; //Function naming started so we set the position
                data.keyword_pos.range_start.0 = parser.pos.0 - 1; //Function naming started so we set the position
                data.keyword_pos.range_end.0 = parser.pos.0; //Function naming started so we set the position
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

                data.chains[chain_length].condition.collective.push(
                    types::cloak_type::CloakEntry {
                        value: Box::new(data.cloak_itered_data.data.value.clone()),
                        value_complete: true,
                    },
                );
            } else {
                let collected = processors::value_processor::collect_value(
                    &mut data.cloak_itered_data,
                    letter_char,
                    next_char,
                    last_char,
                    parser.pos,
                    options,
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
                let child_parser = parser::Parser::new(data.inside_code_string.clone(), options);
                parser.pos = child_parser.pos;
                let mapped = child_parser.map();
                for i in mapped.syntax_errors {
                    errors.push(i)
                }
                let chains_length = data.chains.clone().len() - 1;
                data.chains[chains_length].inside_code = mapped.items;
                parser.collected.push(parser.current.clone());
                parser.current = parser::Collecting::None;
            }
        } else {
            data.inside_code_string += letter_char;
        }
    }
}
