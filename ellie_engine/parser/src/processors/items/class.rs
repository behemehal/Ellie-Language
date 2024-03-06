use alloc::{borrow::ToOwned, vec, vec::Vec};
#[cfg(feature = "standard_rules")]
use ellie_core::warning;
use ellie_core::{defs, error, utils};
use ellie_tokenizer::{
    processors::items::Processors,
    syntax::items::{class::Class, constructor::Constructor},
    tokenizer::{ClassPageType, PageType},
};

impl super::ItemParserProcessor for Class {
    fn process(&self, options: &mut super::ItemParserProcessorOptions) -> bool {
        let (duplicate, found) =
            options
                .parser
                .is_duplicate(options.page_hash, self.name.clone(), self.hash, self.pos);

        let path = options
            .parser
            .pages
            .nth(options.page_idx)
            .unwrap()
            .path
            .clone();
        let class_key_definings = options
            .parser
            .processed_pages
            .nth_mut(options.processed_page_idx)
            .unwrap()
            .unassigned_file_keys
            .clone();

        if utils::is_reserved(
            &self.name,
            class_key_definings
                .iter()
                .any(|x| x.key_name == "dont_fix_variant"),
        ) {
            options.parser.informations.push(
                &error::error_list::ERROR_S21.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: self.name.clone(),
                    }],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    path.clone(),
                    self.name_pos,
                ),
            );
        }

        if duplicate {
            if let Some((page, cursor_pos)) = found {
                let mut err = error::error_list::ERROR_S24.clone().build_with_path(
                    vec![error::ErrorBuildField::new("token", &self.name)],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    path.clone(),
                    self.name_pos,
                );
                err.reference_block = Some((cursor_pos, page.path));
                err.reference_message = "Prime is here".to_owned();
                err.semi_assist = true;
                options.parser.informations.push(&err);
            } else {
                options.parser.informations.push(
                    &error::error_list::ERROR_S24.clone().build_with_path(
                        vec![error::ErrorBuildField::new("token", &self.name)],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        path,
                        self.name_pos,
                    ),
                )
            }
            return false;
        } else {
            #[cfg(feature = "standard_rules")]
            {
                let (is_correct, fixed) =
                    (ellie_standard_rules::rules::CLASS_NAMING_ISSUE.worker)(self.name.clone());
                if !is_correct
                    && !options.parser.global_key_matches(
                        options.page_hash,
                        "allow",
                        "ClassNameRule",
                    )
                {
                    parser
                        .informations
                        .push(&warning::warning_list::WARNING_S1.clone().build(
                            vec![
                                warning::WarningBuildField {
                                    key: "current".to_owned(),
                                    value: self.name.clone(),
                                },
                                warning::WarningBuildField {
                                    key: "correct".to_owned(),
                                    value: fixed,
                                },
                            ],
                            path.clone(),
                            self.name_pos,
                        ))
                }
            }

            let page = options.parser.pages.nth(options.page_idx).unwrap();

            let mut constructors = self.body.iter().filter_map(|item| item.as_constructor());
            let mut items = Vec::new();

            if let Some(prime) = constructors.next() {
                let duplicate_constructors = constructors
                    .enumerate()
                    .map(
                        |(index, element)| {
                            if index == 0 {
                                None
                            } else {
                                Some(element)
                            }
                        },
                    )
                    .filter(|el| el.is_some());

                for generic_defining in &self.generic_definings {
                    let _used_variables = self.body.iter().find_map(|item| match item {
                        Processors::Variable(variable) => {
                            if prime
                                .parameters
                                .iter()
                                .filter_map(|x| Some(&x.name))
                                .collect::<Vec<_>>()
                                .contains(&&variable.data.name)
                            {
                                if variable
                                    .data
                                    .rtype
                                    .definer_type
                                    .is_generic_used_in_defining(&generic_defining.name)
                                {
                                    Some(variable.data.name.clone())
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                        _ => None,
                    });
                }

                for constructor in duplicate_constructors {
                    let mut err = error::error_list::ERROR_S30.clone().build_with_path(
                        vec![],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        path.clone(),
                        constructor.unwrap().pos,
                    );
                    err.reference_block = Some((prime.pos, page.path.clone()));
                    err.reference_message = "Prime is here".to_owned();
                    err.semi_assist = true;
                    options.parser.informations.push(&err);
                }
            } else {
                // If no constructor exists build one.

                let mut inside_code = Vec::new();
                let mut parameters = Vec::new();

                let mut constructor = Processors::Constructor(
                    Constructor {
                        parameters: todo!(),
                        inside_code: todo!(),
                        name_pos: todo!(),
                        parameters_pos: todo!(),
                        brace_count: todo!(),
                        iterator: todo!(),
                        pos: todo!(),
                        complete: todo!(),
                    }
                );

            }

            for (index, generic) in self.generic_definings.iter().enumerate() {
                if let Some(other_index) = self
                    .generic_definings
                    .iter()
                    .position(|g| g.name == generic.name)
                {
                    if other_index < index {
                        let mut err = error::error_list::ERROR_S10.clone().build_with_path(
                            vec![],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            path.clone(),
                            generic.pos,
                        );
                        err.reference_block =
                            Some((self.generic_definings[other_index].pos, page.path.clone()));
                        err.reference_message = "Prime is here".to_owned();
                        err.semi_assist = true;
                        options.parser.informations.push(&err);
                    }
                }
            }

            let inner_page_id: usize = ellie_core::utils::generate_hash_usize();

            for generic in self.generic_definings.clone() {
                items.push(Processors::GenericItem(
                    ellie_tokenizer::syntax::items::generic_item::GenericItem {
                        generic_name: generic.name,
                        pos: generic.pos,
                        hash: generic.hash,
                    },
                ));
            }

            let mut dependencies = vec![ellie_tokenizer::tokenizer::Dependency {
                hash: page.hash,
                processed: false,
                module: None,
                deep_link: Some(page.hash),
                public: false,
            }];
            dependencies.extend(page.dependencies.iter().map(|d| {
                let mut dep = d.clone();
                dep.deep_link = Some(page.hash);
                dep
            }));
            items.extend(self.body.clone());

            let mut inner = ellie_tokenizer::tokenizer::Page {
                hash: inner_page_id,
                inner: Some(page.hash),
                path: page.path.clone(),
                items,
                dependents: vec![],
                dependencies,
                page_type: PageType::ClassBody(ClassPageType {
                    name: self.name.clone(),
                    hash: self.hash,
                    pos: self.pos,
                    page_hash: options.page_hash,
                }),
                unreachable: false,
                unreachable_range: defs::Cursor::default(),
                processed: false,
                module: false,
            };

            inner
                .items
                .push(Processors::ClassInstance(inner.generate_instance()));

            options.parser.pages.push_page(inner);
            let processed_page = options
                .parser
                .processed_pages
                .nth_mut(options.processed_page_idx)
                .unwrap();

            let processed = ellie_core::definite::items::Collecting::Class(
                ellie_core::definite::items::class::Class {
                    name: self.name.clone(),
                    public: self.public,
                    inner_page_id,
                    name_pos: self.name_pos,
                    pos: self.pos,
                    hash: self.hash,
                    file_keys: processed_page.unassigned_file_keys.clone(),
                    generic_definings: self
                        .generic_definings
                        .iter()
                        .map(|x| ellie_core::definite::items::class::GenericDefining {
                            name: x.name.clone(),
                            hash: x.hash,
                            pos: x.pos,
                        })
                        .collect::<Vec<_>>(),
                },
            );

            processed_page.unassigned_file_keys = vec![];
            processed_page.items.push(processed);
        }
        true
    }
}
