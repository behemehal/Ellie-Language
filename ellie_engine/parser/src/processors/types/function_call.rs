use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::definite::definers::DefinerCollecting;
use ellie_core::{definite::types, error};
use ellie_tokenizer::syntax::types::function_call_type;

use crate::deep_search_extensions::{generate_type_from_defining, resolve_type};

use super::TypeParserProcessorOptions;

impl super::TypeParserProcessor for function_call_type::FunctionCallCollector {
    fn process(
        &self,
        options: &mut super::TypeParserProcessorOptions,
    ) -> Result<types::Types, Vec<error::Error>> {
        let mut errors = Vec::new();

        let mut _options = TypeParserProcessorOptions::new(options.parser, options.page_id);

        let target = self.data.target.process(
            &mut _options
                .dont_include_setter()
                .dont_exclude_getter()
                .dont_ignore_type()
                .build(),
        );

        match target {
            Ok(e) => {
                let resolved = resolve_type(
                    e,
                    options.page_id,
                    options.parser,
                    &mut errors,
                    Some(self.data.target_pos),
                );
                match resolved {
                    Some(index) => match &index {
                        DefinerCollecting::Function(function) => {
                            let mut resolved_params = Vec::new();
                            let used_params = self
                                .data
                                .parameters
                                .iter()
                                .filter_map(|param| {
                                    match param.value.process(
                                        &mut &mut options
                                            .dont_include_setter()
                                            .dont_exclude_getter()
                                            .dont_ignore_type()
                                            .build(),
                                    ) {
                                        Ok(resolved) => {
                                            let found = resolve_type(
                                                resolved.clone(),
                                                options.page_id,
                                                options.parser,
                                                &mut errors,
                                                Some(self.data.target_pos),
                                            );

                                            if errors.is_empty() {
                                                resolved_params.push((resolved.clone(), param.pos));
                                                Some((resolved, found.unwrap(), param.pos))
                                            } else {
                                                None
                                            }
                                        }
                                        Err(e) => {
                                            errors.extend(e);
                                            None
                                        }
                                    }
                                })
                                .collect::<Vec<_>>();

                            if function
                                .params
                                .iter()
                                .filter(|x| match x {
                                    DefinerCollecting::Generic(generic) => generic.rtype != "self",
                                    _ => true,
                                })
                                .count()
                                != used_params.len()
                                && errors.is_empty()
                            {
                                errors.push(
                                    error::error_list::ERROR_S7.clone().build_with_path(
                                        vec![
                                            error::ErrorBuildField::new(
                                                "name",
                                                &(index.clone().to_string()),
                                            ),
                                            error::ErrorBuildField::new(
                                                "token",
                                                &function
                                                    .params
                                                    .iter()
                                                    .filter(|x| match x {
                                                        DefinerCollecting::Generic(generic) => {
                                                            generic.rtype != "self"
                                                        }
                                                        _ => true,
                                                    })
                                                    .count()
                                                    .to_string(),
                                            ),
                                            error::ErrorBuildField::new(
                                                "token2",
                                                &used_params.len().to_string(),
                                            ),
                                        ],
                                        alloc::format!(
                                            "{}:{}:{}",
                                            file!().to_owned(),
                                            line!(),
                                            column!()
                                        ),
                                        options
                                            .parser
                                            .find_page(options.page_id)
                                            .unwrap()
                                            .path
                                            .clone(),
                                        self.data.target_pos,
                                    ),
                                );
                                return Err(errors);
                            }

                            if errors.is_empty() {
                                for (index, param) in function.params.iter().filter(|x| matches!(x, DefinerCollecting::Generic(generic) if generic.rtype != "self")).enumerate() {
                                    let used = used_params[index].1.clone();
                                    if !param.same_as(used.clone()) {
                                        errors.push(
                                            error::error_list::ERROR_S3
                                                .clone()
                                                .build_with_path(
                                                    vec![
                                                        error::ErrorBuildField::new(
                                                            "token1",
                                                            &param.to_string(),
                                                        ),
                                                        error::ErrorBuildField {
                                                            key: "token2".to_string(),
                                                            value: used.to_string(),
                                                        },
                                                    ],
                                                    alloc::format!(
                                                        "{}:{}:{}",
                                                        file!().to_owned(),
                                                        line!(),
                                                        column!()
                                                    ),
                                                    options.parser
                                                        .find_page(options.page_id)
                                                        .unwrap()
                                                        .path
                                                        .clone(),
                                                    self.data.parameters[index].pos,
                                                ),
                                        );
                                    }
                                }
                            }

                            if errors.is_empty() {
                                let mut _options = TypeParserProcessorOptions::new(
                                    options.parser,
                                    options.page_id,
                                );
                                match self.data.target.process(
                                    &mut _options
                                        .clone()
                                        .dont_include_setter()
                                        .dont_exclude_getter()
                                        .dont_ignore_type()
                                        .build(),
                                ) {
                                    Ok(resolved) => {
                                        Ok(ellie_core::definite::types::Types::FunctionCall(
                                            ellie_core::definite::types::function_call::FunctionCall {
                                                target: Box::new(resolved),
                                                target_pos: ellie_core::defs::Cursor::default(),
                                                returning: *function.returning.clone(),
                                                params: function.params.iter()
                                                .filter(|x| match x {
                                                    DefinerCollecting::Generic(generic) => {
                                                        generic.rtype != "self"
                                                    }
                                                    _ => true,
                                                }).enumerate()
                                                .map(|(idx, _)| {
                                                    ellie_core::definite::types::function_call::FunctionCallParameter {
                                                        value: resolved_params[idx].0.clone(),
                                                        pos: resolved_params[idx].1
                                                    }
                                                }).collect::<Vec<_>>(),
                                                pos: ellie_core::defs::Cursor::default(),
                                                generic_parameters: vec![],
                                            },
                                        ))
                                    }
                                    Err(e) => {
                                        errors.extend(e);
                                        Err(errors)
                                    }
                                }
                            } else {
                                Err(errors)
                            }
                        }
                        DefinerCollecting::EnumField(e) => {
                            Ok(ellie_core::definite::types::Types::EnumData(
                                ellie_core::definite::types::enum_data::EnumData {
                                    reference: Box::new(types::Types::VariableType(
                                        types::variable::VariableType {
                                            value: e.name.clone(),
                                            reference: e.hash,
                                            pos: ellie_core::defs::Cursor::default(),
                                        },
                                    )),
                                    reference_pos: ellie_core::defs::Cursor::default(),
                                    brace_pos: ellie_core::defs::Cursor::default(),
                                    value: match e.field_data.clone() {
                                        ellie_core::definite::definers::EnumFieldData::NoData => {
                                            types::enum_data::Pointer::NoData
                                        }
                                        ellie_core::definite::definers::EnumFieldData::Data(
                                            rtype,
                                        ) => {
                                            let value = generate_type_from_defining(
                                                *rtype,
                                                options.page_id,
                                                options.parser,
                                            )
                                            .unwrap();
                                            ellie_core::definite::types::enum_data::Pointer::Data(
                                                Box::new(value),
                                            )
                                        }
                                    },
                                    field_name: e.field_name.clone(),
                                    //TODO this is not correct
                                    pos: ellie_core::defs::Cursor::default(),
                                },
                            ))
                        }
                        _ => {
                            errors.push(
                                error::error_list::ERROR_S25.clone().build_with_path(
                                    vec![error::ErrorBuildField {
                                        key: "token".to_string(),
                                        value: index.to_string(),
                                    }],
                                    alloc::format!(
                                        "{}:{}:{}",
                                        file!().to_owned(),
                                        line!(),
                                        column!()
                                    ),
                                    options
                                        .parser
                                        .find_page(options.page_id)
                                        .unwrap()
                                        .path
                                        .clone(),
                                    self.data.target_pos,
                                ),
                            );
                            Err(errors)
                        }
                    },
                    None => Err(errors),
                }
            }
            Err(e) => {
                errors.extend(e);
                Err(errors)
            }
        }
    }
}
