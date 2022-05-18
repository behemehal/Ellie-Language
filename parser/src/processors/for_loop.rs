use crate::processors::type_processor::process;
use alloc::{borrow::ToOwned, vec, vec::Vec};
use ellie_core::{definite::definers::DefinerCollecting, error};
use ellie_tokenizer::{syntax::items::for_loop::ForLoop, tokenizer::PageType};

impl super::Processor for ForLoop {
    fn process(self, parser: &mut crate::parser::Parser, page_id: u64) -> bool {
        let path = parser.find_page(page_id).unwrap().path.clone();
        if self.variable.current.as_variable().is_none() {
            parser
                .informations
                .push(&error::error_list::ERROR_S27.clone().build_with_path(
                    vec![],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    path,
                    self.variable_pos,
                ));
            return true;
        }

        let variable_name = self
            .variable
            .current
            .as_variable()
            .unwrap()
            .data
            .value
            .clone();

        let (duplicate, found) =
            parser.is_duplicate(page_id, variable_name.clone(), 0, self.variable_pos);

        if duplicate {
            if let Some((page, cursor_pos)) = found {
                let mut err = error::error_list::ERROR_S24.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: variable_name,
                    }],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    parser.find_page(page_id).unwrap().path.clone(),
                    self.variable_pos,
                );
                err.reference_block = Some((cursor_pos, page.path));
                err.reference_message = "Prime is here".to_owned();
                err.semi_assist = true;
                parser.informations.push(&err);
            } else {
                let page_path = parser.find_page(page_id).unwrap().path.clone();
                parser
                    .informations
                    .push(&error::error_list::ERROR_S24.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: variable_name.clone(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        page_path,
                        self.variable_pos,
                    ))
            }
            return false;
        } else {
            let iterator = match process(self.target_iterator.current, parser, page_id, None, false)
            {
                Ok(rtype) => rtype,
                Err(e) => {
                    parser.informations.extend(&e);
                    return false;
                }
            };

            let mut errors: Vec<error::Error> = vec![];

            let target_iterator = match crate::deep_search_extensions::resolve_type(
                iterator.clone(),
                page_id,
                parser,
                &mut errors,
                Some(self.iterator_pos),
            ) {
                Some(e) => e,
                None => {
                    parser.informations.extend(&errors);
                    return false;
                }
            };

            let mut inner_type: DefinerCollecting = DefinerCollecting::Dynamic;

            match &target_iterator {
                ellie_core::definite::definers::DefinerCollecting::ParentGeneric(e) => {
                    if e.rtype != "array" && e.rtype != "vector" {
                        parser.informations.push(
                            &error::error_list::ERROR_S29.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: target_iterator.to_string(),
                                }],
                                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                                path,
                                self.iterator_pos,
                            ),
                        );
                    }
                    inner_type = e.generics[0].clone().value
                }
                _ => {
                    parser.informations.push(
                        &error::error_list::ERROR_S29.clone().build_with_path(
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: target_iterator.to_string(),
                            }],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            path,
                            self.iterator_pos,
                        ),
                    );
                }
            }

            let inner_page_id: u64 = ellie_core::utils::generate_hash_u64();
            let page = parser.find_page(page_id).unwrap().clone();
            let mut dependencies = vec![ellie_tokenizer::tokenizer::Dependency {
                hash: page.hash.clone(),
                processed: false,
                module: None,
                deep_link: None,
                public: false,
            }];

            let mut items = self.body;

            items.push(ellie_tokenizer::processors::items::Processors::Variable(
                ellie_tokenizer::syntax::items::variable::VariableCollector {
                    data: ellie_tokenizer::syntax::items::variable::Variable {
                        name: variable_name,
                        constant: true,
                        public: false,
                        has_type: true,
                        has_value: false,
                        pos: self.variable_pos,
                        name_pos: self.variable_pos,
                        type_pos: self.variable_pos,
                        rtype: ellie_tokenizer::syntax::items::definers::DefinerCollector {
                            definer_type: ellie_core::definite::Converter::from_definite(
                                ellie_tokenizer::syntax::items::definers::DefinerTypes::Dynamic,
                                inner_type.clone(),
                            ),
                            complete: true,
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ));

            dependencies.extend(page.dependencies);
            let inner = ellie_tokenizer::tokenizer::Page {
                hash: inner_page_id,
                inner: Some(page.hash),
                path: page.path.clone(),
                page_type: PageType::ForBody,
                items,
                dependents: vec![],
                dependencies,
                ..Default::default()
            };
            parser.pages.push(inner);
            let processed_page = parser.find_processed_page(page_id).unwrap();
            processed_page
                .items
                .push(ellie_core::definite::items::Collecting::ForLoop(
                    ellie_core::definite::items::for_loop::ForLoop {
                        variable: self.variable.current.to_definite(),
                        iterator,
                        parameter: self.parameter,
                        body_pos: self.body_pos,
                        inner_page_id,
                        pos: self.pos,
                        variable_pos: self.variable_pos,
                        iterator_pos: self.iterator_pos,
                    },
                ));
            parser.process_page(inner_page_id);
        }
        true
    }
}