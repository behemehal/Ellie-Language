use alloc::{borrow::ToOwned, vec, vec::Vec};
use ellie_core::{defs, error, warning};
use ellie_tokenizer::{syntax::items::class::Class, tokenizer::PageType};

impl super::Processor for Class {
    fn process(
        &self,
        parser: &mut super::Parser,
        page_idx: usize,
        _processed_page_idx: usize,
        page_hash: u64,
    ) -> bool {
        let (duplicate, found) =
            parser.is_duplicate(page_hash, self.name.clone(), self.hash.clone(), self.pos);
        let path = parser.pages.nth(page_idx).unwrap().path.clone();
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
                parser.informations.push(&err);
            } else {
                parser
                    .informations
                    .push(&error::error_list::ERROR_S24.clone().build_with_path(
                        vec![error::ErrorBuildField::new("token", &self.name)],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        path,
                        self.name_pos,
                    ))
            }
            return false;
        } else {
            #[cfg(feature = "standard_rules")]
            {
                let (is_correct, fixed) =
                    (ellie_standard_rules::rules::CLASS_NAMING_ISSUE.worker)(self.name.clone());
                if !is_correct
                    && !parser.page_has_file_key_with(page_hash, "allow", "ClassNameRule")
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

            let page = parser.pages.nth_mut(page_idx).unwrap().clone();

            let constructors = self.body.iter().filter_map(|item| match item {
                ellie_tokenizer::processors::items::Processors::Constructor(e) => Some(e),
                _ => None,
            });

            if constructors.clone().count() > 0 {
                let prime = constructors.clone().nth(0).unwrap();
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
                    parser.informations.push(&err);
                }
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
                        parser.informations.push(&err);
                    }
                }
            }

            let inner_page_id: u64 = ellie_core::utils::generate_hash_u64();

            let mut items = Vec::new();

            for generic in self.generic_definings.clone() {
                items.push(ellie_tokenizer::processors::items::Processors::GenericItem(
                    ellie_tokenizer::syntax::items::generic_item::GenericItem {
                        generic_name: generic.name,
                        pos: generic.pos,
                        hash: generic.hash,
                    },
                ));
            }

            items.push(ellie_tokenizer::processors::items::Processors::SelfItem(
                ellie_tokenizer::syntax::items::self_item::SelfItem {
                    class_page: page_hash,
                    class_hash: self.hash.clone(),
                },
            ));

            let mut dependencies = vec![ellie_tokenizer::tokenizer::Dependency {
                hash: page.hash.clone(),
                processed: false,
                module: None,
                deep_link: None,
                public: false,
            }];
            dependencies.extend(page.dependencies);
            items.extend(self.body.clone());

            let inner = ellie_tokenizer::tokenizer::Page {
                hash: inner_page_id,
                inner: Some(page.hash),
                path: page.path.clone(),
                items,
                dependents: vec![],
                dependencies,
                page_type: PageType::ClassBody,
                unreachable: false,
                unreachable_range: defs::Cursor::default(),
                processed: false,
                module: false,
            };
            parser.pages.push_page(inner);
            let processed = ellie_core::definite::items::Collecting::Class(
                ellie_core::definite::items::class::Class {
                    name: self.name.clone(),
                    public: self.public,
                    inner_page_id,
                    name_pos: self.name_pos,
                    pos: self.pos,
                    hash: self.hash,
                    generic_definings: self
                        .generic_definings
                        .iter()
                        .map(|x| ellie_core::definite::items::class::GenericDefining {
                            name: x.name.clone(),
                            hash: x.hash.clone(),
                            pos: x.pos,
                        })
                        .collect::<Vec<_>>(),
                },
            );
            parser
                .find_processed_page(page_hash)
                .unwrap()
                .items
                .push(processed);
            parser.process_page(inner_page_id);
        }
        true
    }
}
