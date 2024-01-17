use crate::deep_search_extensions::{find_type, generate_type_from_defining};
use crate::processors::definer::{DefinerParserProcessor, DefinerParserProcessorOptions};
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::definite::types;
use ellie_core::definite::Converter;
use ellie_core::error::{self, error_list};
use ellie_tokenizer::syntax::types::variable_type;
use ellie_tokenizer::tokenizer::PageType;
use types::Types;

impl super::TypeParserProcessor for variable_type::VariableTypeCollector {
    fn process(
        &self,
        options: &mut super::TypeParserProcessorOptions,
    ) -> Result<types::Types, Vec<error::Error>> {
        let mut errors = Vec::new();
        let deep_search_result = options.parser.deep_search(
            options.page_id,
            self.data.value.clone(),
            options.ignore_hash,
            Vec::new(),
            0,
            options.variable_pos,
        );

        if deep_search_result.found {
            match deep_search_result.found_item {
                crate::parser::DeepSearchItems::Class(_) => {
                    //ERROR_S15
                    let path = options
                        .parser
                        .find_page(options.page_id)
                        .unwrap()
                        .path
                        .clone();
                    errors.push(error::error_list::ERROR_S15.clone().build_with_path(
                        vec![],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        path.clone(),
                        self.data.pos,
                    ));
                    Err(errors)
                }
                crate::parser::DeepSearchItems::Variable(e) => {
                    let page = options.parser.find_page(options.page_id).unwrap().clone();
                    if !e.constant
                        && matches!(page.page_type, PageType::FunctionBody(_))
                        && deep_search_result.found_page.hash != page.hash
                    {
                        //ERROR_S16
                        let path = options
                            .parser
                            .find_page(options.page_id)
                            .unwrap()
                            .path
                            .clone();
                        let mut error = error::error_list::ERROR_S61.clone().build_with_path(
                            vec![],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            path.clone(),
                            self.data.pos,
                        );
                        error.reference_block =
                            Some((e.pos, deep_search_result.found_page.path.clone()));
                        error.reference_message = "Defined here".to_owned();
                        errors.push(error);
                        return Err(errors);
                    }

                    Ok(types::Types::VariableType(types::variable::VariableType {
                        value: e.name,
                        reference: e.hash,
                        pos: self.data.pos,
                    }))
                }
                crate::parser::DeepSearchItems::Enum(e) => {
                    Ok(types::Types::VariableType(types::variable::VariableType {
                        value: e.name,
                        reference: e.hash,
                        pos: self.data.pos,
                    }))
                }
                crate::parser::DeepSearchItems::Getter(e) => {
                    if options.exclude_getter {
                        errors.push(
                            error::error_list::ERROR_S4.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: e.name,
                                }],
                                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                                options
                                    .parser
                                    .find_page(options.page_id)
                                    .unwrap()
                                    .path
                                    .clone(),
                                self.data.pos,
                            ),
                        );
                        Err(errors)
                    } else {
                        match generate_type_from_defining(
                            e.return_type.definer_type.to_definite(),
                            options.page_id,
                            options.parser,
                        ) {
                            Some(e) => Ok(e),
                            None => Err(errors),
                        }
                    }
                }
                crate::parser::DeepSearchItems::Setter(e) => {
                    if options.include_setter {
                        Ok(Types::SetterCall(
                            e.parameters
                                .first()
                                .unwrap()
                                .rtype
                                .definer_type
                                .clone()
                                .to_definite(),
                        ))
                    } else {
                        errors.push(
                            error::error_list::ERROR_S23.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: e.name,
                                }],
                                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                                options
                                    .parser
                                    .find_page(options.page_id)
                                    .unwrap()
                                    .path
                                    .clone(),
                                self.data.pos,
                            ),
                        );
                        Err(errors)
                    }
                }
                crate::parser::DeepSearchItems::Function(function) => {
                    match function.return_type.definer_type.process(
                        &mut DefinerParserProcessorOptions::new(options.parser, options.page_id)
                            .optional_ignore_hash(options.ignore_hash)
                            .build(),
                    ) {
                        Ok(_) => {
                            match find_type("function".to_owned(), options.page_id, options.parser)
                            {
                                Some(_) => Ok(ellie_core::definite::types::Types::VariableType(
                                    ellie_core::definite::types::variable::VariableType {
                                        value: function.name,
                                        reference: function.hash,
                                        pos: ellie_core::defs::Cursor::default(),
                                    },
                                )),
                                None => {
                                    errors.push(
                                        error::error_list::ERROR_S38.clone().build_with_path(
                                            vec![error::ErrorBuildField {
                                                key: "token".to_owned(),
                                                value: "function".to_string(),
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
                                            self.data.pos,
                                        ),
                                    );
                                    Err(errors)
                                }
                            }
                        }
                        Err(e) => {
                            errors.extend(e);
                            Err(errors)
                        }
                    }
                }
                crate::parser::DeepSearchItems::ImportReference(_) => {
                    todo!("import reference type not yet implemented")
                }
                crate::parser::DeepSearchItems::BrokenPageGraph => todo!(),
                crate::parser::DeepSearchItems::MixUp(_) => todo!(),
                crate::parser::DeepSearchItems::None => todo!(),
                crate::parser::DeepSearchItems::ClassInstance(self_item) => {
                    Ok(types::Types::VariableType(types::variable::VariableType {
                        value: String::from("self"),
                        reference: self_item.class_hash,
                        pos: self.data.pos,
                    }))
                }
                crate::parser::DeepSearchItems::GenericItem(_) => todo!(),
                crate::parser::DeepSearchItems::FunctionParameter(e) => Ok(
                    types::Types::FunctionParameter(types::function::FunctionParameter {
                        name: e.name,
                        rtype: Some(e.rtype),
                        name_pos: e.name_pos,
                        rtype_pos: e.rtype_pos,
                    }),
                ),
                crate::parser::DeepSearchItems::ConstructorParameter(e) => {
                    Ok(types::Types::VariableType(types::variable::VariableType {
                        value: e.name,
                        reference: 0,
                        pos: e.pos,
                    }))
                }
                crate::parser::DeepSearchItems::SelfItem(e) => {
                    Ok(types::Types::VariableType(types::variable::VariableType {
                        value: String::from("self"),
                        reference: e.class_hash,
                        pos: self.data.pos,
                    }))
                }
            }
        } else {
            errors.push(
                error::error_list::ERROR_S6.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: self.data.value.clone(),
                    }],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    options
                        .parser
                        .find_page(options.page_id)
                        .unwrap()
                        .path
                        .clone(),
                    self.data.pos,
                ),
            );
            Err(errors)
        }
    }
}
