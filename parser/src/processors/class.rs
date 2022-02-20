use alloc::{borrow::ToOwned, vec, vec::Vec};
use ellie_core::{defs, error, warning};
use ellie_tokenizer::{syntax::items::class::Class, tokenizer::PageType};

impl super::Processor for Class {
    fn process(self, parser: &mut super::Parser, page_id: u64) -> bool {
        let (duplicate, found) =
            parser.is_duplicate(page_id, self.name.clone(), self.hash.clone(), self.pos);
        if duplicate {
            if let Some((page, cursor_pos)) = found {
                let mut err = error::error_list::ERROR_S24.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: self.name,
                    }],
                    file!().to_owned(),
                    parser.find_page(page_id).unwrap().path.clone(),
                    self.name_pos,
                );
                err.reference_block = Some((cursor_pos, page.path));
                err.reference_message = "Prime is here".to_owned();
                err.semi_assist = true;
                parser.informations.push(&err);
            } else {
                let path = parser.find_page(page_id).unwrap().path.clone();
                parser
                    .informations
                    .push(&error::error_list::ERROR_S24.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: self.name,
                        }],
                        file!().to_owned(),
                        path,
                        self.name_pos,
                    ))
            }
        } else {
            #[cfg(feature = "standard_rules")]
            {
                let (is_correct, fixed) =
                    (ellie_standard_rules::rules::CLASS_NAMING_ISSUE.worker)(self.name.clone());
                if !is_correct && !parser.page_has_file_key_with(page_id, "allow", "ClassNameRule")
                {
                    let path = parser.find_page(page_id).unwrap().path.clone();
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
                            path,
                            self.name_pos,
                        ))
                }
            }

            let page = parser.find_page(page_id).unwrap().clone();

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
                        file!().to_owned(),
                        parser.find_page(page_id).unwrap().path.clone(),
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
                            file!().to_owned(),
                            parser.find_page(page_id).unwrap().path.clone(),
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

            let mut items = self.body;

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
                    class_page: page_id,
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
            parser.pages.push(inner);
            parser.process_page(inner_page_id);
            let processed = ellie_core::definite::items::Collecting::Class(
                ellie_core::definite::items::class::Class {
                    name: self.name,
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
                .find_processed_page(page_id)
                .unwrap()
                .items
                .push(processed);
        }
        true
    }
}
