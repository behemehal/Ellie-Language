use crate::deep_search_extensions::{
    deep_search, deep_search_hash, find_type, generate_type_from_defining, resolve_type,
    ProcessedDeepSearchItems,
};
use alloc::{borrow::ToOwned, boxed::Box, string::ToString, vec, vec::Vec};
use ellie_core::{definite::types, error};
use ellie_tokenizer::syntax::types::brace_reference_type;

impl super::TypeParserProcessor for brace_reference_type::BraceReferenceTypeCollector {
    fn process(
        &self,
        options: &mut super::TypeParserProcessorOptions,
    ) -> Result<types::Types, Vec<error::Error>> {
        let mut errors = Vec::new();

        let mut _options = super::TypeParserProcessorOptions::new(options.parser, options.page_id);

        let index = self
            .data
            .value
            .process(&mut _options.dont_include_setter().dont_ignore_type().build());

        match index {
            Ok(index) => {
                //if matches!(index, types::Types::Integer(x) if matches!()) {}

                let index_type = match resolve_type(
                    index.clone(),
                    options.page_id,
                    options.parser,
                    &mut errors,
                    Some(self.data.brace_pos),
                ) {
                    Some(e) => e,
                    None => {
                        return Err(errors);
                    }
                };

                let mut _options = super::TypeParserProcessorOptions::new(options.parser, options.page_id);

                let reference = self
                    .data
                    .reference
                    .process(&mut _options.dont_include_setter().dont_ignore_type().build());

                match reference {
                    Ok(found_reference) => {
                        let reference_type = resolve_type(
                            found_reference.clone(),
                            options.page_id,
                            options.parser,
                            &mut errors,
                            Some(self.data.reference_pos),
                        );
                        // TODO Ellie should let developers implement indexable properties in classes
                        //Example
                        // class A {
                        //     pub v indexable_property : array<int>;
                        //
                        //     @index="GET"
                        //     fn get(index: int) : int {
                        //         ret indexable_property[index];
                        //     }
                        //
                        //     @index="SET"
                        //     fn set(index: int, value: int) {
                        //         indexable_property[index] = value;
                        //     }
                        //
                        // }
                        //
                        // v a = new A();
                        // a[0] = 1;
                        match reference_type.clone() {
                            Some(reference_type) => {
                                match reference_type.clone() {
                                    ellie_core::definite::definers::DefinerCollecting::ParentGeneric(reference_generic) => {
                                        if reference_generic.rtype == "array" {
                                            match index_type.clone() {
                                                ellie_core::definite::definers::DefinerCollecting::Generic(index_generic_type) => {
                                                    if index_generic_type.rtype == "int" {
                                                        Ok(types::Types::BraceReference(types::brace_reference::BraceReferenceType {
                                                            reference: Box::new(found_reference),
                                                            reference_pos: self.data.reference_pos,
                                                            brace_pos: self.data.brace_pos,
                                                            value: Box::new(index),
                                                            pos: self.data.pos,
                                                        }))
                                                    } else {
                                                        errors.push(error::error_list::ERROR_S49.clone().build_with_path(
                                                            vec![error::ErrorBuildField {
                                                                key: "target".to_string(),
                                                                value: reference_type.to_string(),
                                                            },error::ErrorBuildField {
                                                                key: "token".to_string(),
                                                                value: index_type.to_string(),
                                                            }],
                                                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                                                            options.parser.find_page(options.page_id).unwrap().path.clone(),
                                                            self.data.brace_pos
                                                        ));
                                                        Err(errors)
                                                    }
                                                },
                                                _ => {
                                                    errors.push(error::error_list::ERROR_S49.clone().build_with_path(
                                                        vec![error::ErrorBuildField {
                                                            key: "target".to_string(),
                                                            value: reference_type.to_string(),
                                                        },error::ErrorBuildField {
                                                            key: "token".to_string(),
                                                            value: index_type.to_string(),
                                                        }],
                                                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                                                        options.parser.find_page(options.page_id).unwrap().path.clone(),
                                                        self.data.brace_pos
                                                    ));
                                                    Err(errors)
                                                }
                                            }
                                        } else if reference_generic.rtype == "cloak" {
                                            match index_type.clone() {
                                                ellie_core::definite::definers::DefinerCollecting::Generic(index_generic_type) => {
                                                    if index_generic_type.rtype == "int" {
                                                        Ok(types::Types::BraceReference(types::brace_reference::BraceReferenceType {
                                                            reference: Box::new(found_reference),
                                                            reference_pos: self.data.reference_pos,
                                                            brace_pos: self.data.brace_pos,
                                                            value: Box::new(index),
                                                            pos: self.data.pos,
                                                        }))
                                                    } else {
                                                        errors.push(error::error_list::ERROR_S49.clone().build_with_path(
                                                            vec![error::ErrorBuildField {
                                                                key: "target".to_string(),
                                                                value: reference_type.to_string(),
                                                            },error::ErrorBuildField {
                                                                key: "token".to_string(),
                                                                value: index_type.to_string(),
                                                            }],
                                                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                                                            options.parser.find_page(options.page_id).unwrap().path.clone(),
                                                            self.data.brace_pos
                                                        ));
                                                        Err(errors)
                                                    }
                                                },
                                                _ => {
                                                    errors.push(error::error_list::ERROR_S49.clone().build_with_path(
                                                        vec![error::ErrorBuildField {
                                                            key: "target".to_string(),
                                                            value: reference_type.to_string(),
                                                        },error::ErrorBuildField {
                                                            key: "token".to_string(),
                                                            value: index_type.to_string(),
                                                        }],
                                                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                                                        options.parser.find_page(options.page_id).unwrap().path.clone(),
                                                        self.data.brace_pos
                                                    ));
                                                    Err(errors)
                                                }
                                            }
                                        } else if reference_generic.rtype == "collective" {
                                            errors.push(error::error_list::ERROR_S59.clone().build_with_path(
                                                vec![error::ErrorBuildField {
                                                    key: "token".to_string(),
                                                    value: "Collective Index Queries".to_string(),
                                                }],
                                                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                                                options.parser.find_page(options.page_id).unwrap().path.clone(),
                                                self.data.brace_pos
                                            ));
                                            Err(errors)
                                        } else {
                                            errors.push(error::error_list::ERROR_S59.clone().build_with_path(
                                                vec![error::ErrorBuildField {
                                                    key: "token".to_string(),
                                                    value: "Custom Index Queries".to_string(),
                                                }],
                                                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                                                options.parser.find_page(options.page_id).unwrap().path.clone(),
                                                self.data.brace_pos
                                            ));
                                            Err(errors)
                                        }
                                    },
                                    _ => {
                                        errors.push(error::error_list::ERROR_S48.clone().build_with_path(
                                            vec![error::ErrorBuildField {
                                                key: "token".to_string(),
                                                value: reference_type.to_string(),
                                            }],
                                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                                            options.parser.find_page(options.page_id).unwrap().path.clone(),
                                            self.data.reference_pos
                                        ));
                                        Err(errors)
                                    }
                                }
                            },
                            None => Err(errors)
                        }
                    }
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }
}
