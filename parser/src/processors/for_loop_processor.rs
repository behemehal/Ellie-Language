use crate::alloc::borrow::ToOwned;
use crate::alloc::vec::Vec;
use crate::parser;
use crate::processors;
use crate::syntax::definers;
use crate::syntax::import_item;
use crate::syntax::types;
use crate::syntax::variable;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec;
use ellie_core::defs;
use ellie_core::error;

pub fn collect_for<F>(
    parser: &mut parser::Parser<F>,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
{
    let parser_clone = parser.clone();
    if let parser::Collecting::ForLoop(ref mut for_loop_data) = parser.current {
        for_loop_data.cloak_itered_data.ignore_existence = true;

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

                if let Some(error_type) = parser_clone.is_iterable(deep_call) {
                    if error_type == 1 {
                        errors.push(error::Error {
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "replace_for_loop_processor_42".to_owned(),
                            title: error::errorList::error_s1.title.clone(),
                            code: error::errorList::error_s1.code,
                            message: error::errorList::error_s1.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s1.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: ellie_core::utils::trim_good(
                                        for_loop_data.raw_parameter.clone(),
                                    ),
                                }],
                            ),
                            pos: for_loop_data.data.parameter_pos,
                        });
                    } else {
                        errors.push(error::Error {
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "13e544d3d16cc94cd2e6a16a27c283b3".to_owned(),
                            title: error::errorList::error_s29.title.clone(),
                            code: error::errorList::error_s29.code,
                            message: error::errorList::error_s29.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s29.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: for_loop_data.cloak_itered_data.data.value.get_type(),
                                }],
                            ),
                            pos: for_loop_data.data.parameter_pos,
                        });
                    }
                }

                for_loop_data.data.parameter =
                    Box::new(for_loop_data.cloak_itered_data.data.value.clone());
            } else {
                if letter_char != " " && for_loop_data.data.parameter_pos.range_start.is_zero() {
                    for_loop_data.data.parameter_pos.range_start = parser_clone.pos.clone();
                    for_loop_data.raw_parameter = String::new();
                }
                for_loop_data.data.parameter_pos.range_end = parser_clone.pos.clone();
                for_loop_data.raw_parameter += letter_char;
                processors::value_processor::collect_value(
                    parser_clone,
                    &mut for_loop_data.cloak_itered_data,
                    errors,
                    letter_char,
                    next_char,
                    last_char,
                );
            }
        } else if letter_char == "}" {
            let mut filtered_items: Vec<parser::Collecting> = Vec::new();
            for item in for_loop_data.code.collected.clone() {
                match item {
                    parser::Collecting::ImportItem(e) => {
                        if e.from_path != "<temporary>" {
                            filtered_items.push(parser::Collecting::ImportItem(e))
                        }
                    }
                    e => filtered_items.push(e),
                }
            }

            for_loop_data.data.code = filtered_items;
            for_loop_data.data.pos.range_end = parser.pos.clone().skip_char(1);
            parser.collected.push(parser.current.clone());
            parser.current = parser::Collecting::None;
        } else {
            if letter_char == "{" {
                for_loop_data.brace_count += 1;
            } else if letter_char == "}" && for_loop_data.brace_count != 0 {
                for_loop_data.brace_count -= 1;
            }
            let mut child_parser = for_loop_data.code.clone().to_no_resolver_parser();

            if for_loop_data.code.pos.is_zero() {
                //Make sure upper scope imported once

                if let types::Types::Cloak(param) = *for_loop_data.data.parameter.clone() {
                    if let types::Types::VariableType(first_param) =
                        *param.data.collective[0].value.clone()
                    {
                        child_parser.collected.push(parser::Collecting::ImportItem(
                            import_item::ImportItem {
                                resolution_id: 0,
                                from_import: 0,
                                from_path: "<temporary>".to_owned(),
                                public: true,
                                item: Box::new(parser::Collecting::Variable(
                                    variable::VariableCollector {
                                        data: variable::Variable {
                                            name: first_param.data.value,
                                            dynamic: true,
                                            constant: true,
                                            public: true,
                                            rtype: definers::DefinerCollecting::Dynamic,
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                )),
                            },
                        ));
                        std::println!("PARAM IMPORTED: {:#?}", child_parser.collected);
                    }
                }

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
            child_parser.current = for_loop_data.code.current.clone();
            child_parser.keyword_catch = for_loop_data.code.keyword_catch.clone();
            child_parser.keyword_cache = for_loop_data.code.keyword_cache.clone();

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

            for_loop_data.code = Box::new(child_parser.to_raw());
        }
    } else {
        panic!("Unexpected parser behaviour")
    }
}
