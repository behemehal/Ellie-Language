use crate::parser;
use crate::processors;
use crate::syntax::condition;
use crate::syntax::import_item;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::vec::Vec;
use ellie_core::defs;
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
        } else if letter_char == "}" && condition_data.brace_count == 0 {
            let chain_length = condition_data.data.chains.len();

            let mut filtered_items: Vec<parser::Collecting> = Vec::new();
            for item in condition_data.code.collected.clone() {
                match item {
                    parser::Collecting::ImportItem(e) => {
                        if e.from_path != "<temporary>" {
                            filtered_items.push(parser::Collecting::ImportItem(e))
                        }
                    }
                    e => filtered_items.push(e),
                }
            }

            condition_data.data.chains[chain_length - 1].code = filtered_items;
            condition_data.data.chains[chain_length - 1].pos.range_end =
                parser.pos.clone().skip_char(1);
            parser.collected.push(parser.current.clone());
            parser.current = parser::Collecting::None;
        } else {
            if letter_char == "{" {
                condition_data.brace_count += 1;
            } else if letter_char == "}" && condition_data.brace_count != 0 {
                condition_data.brace_count -= 1;
            }
            let mut child_parser = condition_data.code.clone().to_no_resolver_parser();

            if condition_data.code.pos.is_zero() {
                //Make sure upper scope imported once

                for item in parser.collected.clone() {
                    //Import variables as temporary for syntax support, we will remove them after collecting complete
                    child_parser.collected.push(parser::Collecting::ImportItem(
                        import_item::ImportItem {
                            resolution_id: 0,
                            from_import: 0,
                            from_path: "<temporary>".to_owned(),
                            public: true,
                            item: Box::new(item),
                        },
                    ));
                }
            }

            child_parser.options = parser.options.clone();
            child_parser.options.parser_type = defs::ParserType::RawParser;
            child_parser.pos = parser.pos;
            child_parser.scope.scope_name = "core/for_loop_processor".to_owned();
            child_parser.current = condition_data.code.current.clone();
            child_parser.keyword_catch = condition_data.code.keyword_catch.clone();
            child_parser.keyword_cache = condition_data.code.keyword_cache.clone();

            let mut child_parser_errors: Vec<error::Error> = Vec::new();
            parser::iterator::iter(
                &mut child_parser,
                &mut child_parser_errors,
                letter_char,
                next_char,
                last_char,
            );
            for i in child_parser_errors {
                errors.push(i);
            }

            condition_data.code = Box::new(child_parser.to_raw());
        }
    } else {
        panic!("Unexpected parser behaviour")
    }
}
