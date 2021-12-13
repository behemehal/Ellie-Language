use alloc::{borrow::ToOwned, vec, vec::Vec};
use ellie_core::{definite::items::Collecting, defs, error, warning};
use ellie_tokenizer::syntax::items::class::Class;

impl super::Processor for Class {
    fn process(self, parser: &mut super::Parser, page_id: u64) {
        let (duplicate, found) =
            parser.is_duplicate(page_id, self.name.clone(), self.hash.clone(), self.pos);
        if duplicate {
            if let Some((page, cursor_pos)) = found {
                let mut err = error::error_list::ERROR_S24.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: self.name,
                    }],
                    "pcls_0x16".to_owned(),
                    parser.find_page(page_id).unwrap().path.clone(),
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
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: self.name,
                        }],
                        "pcls_0x31".to_owned(),
                        parser.find_page(page_id).unwrap().path.clone(),
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
                            parser.find_page(page_id).unwrap().path.clone(),
                            self.name_pos,
                        ))
                }
            }

            let page = parser.find_page(page_id).unwrap().clone();
            let mut has_constructor: Option<defs::Cursor> = None;

            for element in self.body.clone() {
                match element {
                    ellie_tokenizer::processors::items::Processors::Constructor(x) => {
                        if has_constructor.is_none() {
                            has_constructor = Some(x.pos);
                        } else {
                            let mut err = error::error_list::ERROR_S30.clone().build_with_path(
                                vec![],
                                "pcls_0x74".to_owned(),
                                parser.find_page(page_id).unwrap().path.clone(),
                                self.name_pos,
                            );
                            err.reference_block =
                                Some((has_constructor.unwrap(), page.path.clone()));
                            err.reference_message = "Prime is here".to_owned();
                            err.semi_assist = true;
                            parser.informations.push(&err);
                        }
                    }
                    ellie_tokenizer::processors::items::Processors::Variable(_) => (),
                    ellie_tokenizer::processors::items::Processors::Function(_) => (),
                    ellie_tokenizer::processors::items::Processors::SelfItem(_) => (),
                    ellie_tokenizer::processors::items::Processors::GenericItem(_) => (),
                    e => {
                        let mut err = error::error_list::ERROR_S22.clone().build_with_path(
                            vec![],
                            "pcls_0x92".to_owned(),
                            parser.find_page(page_id).unwrap().path.clone(),
                            e.get_pos(),
                        );
                        err.full_assist = true;
                        parser.informations.push(&err);
                    }
                };
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
                            "pcls_0x74".to_owned(),
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

            let mut inner_dependencies = page.dependencies.clone();
            inner_dependencies.push(ellie_tokenizer::tokenizer::Dependency {
                hash: page.hash.clone(),
                public: false,
            });

            let inner_page_id: u64 = ellie_core::utils::generate_hash_u64();

            let mut items = self.body;

            for generic in self.generic_definings {
                items.push(ellie_tokenizer::processors::items::Processors::GenericItem(
                    ellie_tokenizer::syntax::items::generic_item::GenericItem {
                        generic_name: generic.name,
                        pos: generic.pos,
                    },
                ));
            }

            items.push(ellie_tokenizer::processors::items::Processors::SelfItem(
                ellie_tokenizer::syntax::items::self_item::SelfItem {
                    class_page: page_id,
                    class_hash: self.hash.clone(),
                },
            ));

            let inner = ellie_tokenizer::tokenizer::Page {
                hash: inner_page_id,
                inner: Some(page.hash),
                path: page.path.clone(),
                generics_allowed: true,
                items,
                dependents: vec![],
                dependencies: inner_dependencies,
            };
            parser.pages.push(inner);
            parser.process_page(inner_page_id);

            let processed_page = parser.find_processed_page(inner_page_id).unwrap();

            let processed = ellie_core::definite::items::Collecting::Class(
                ellie_core::definite::items::class::Class {
                    name: self.name,
                    public: self.public,
                    inner_page_id,
                    constructor: processed_page
                        .items
                        .iter()
                        .find_map(|f| match f {
                            Collecting::Constructor(c) => Some(Some(c.clone())),
                            _ => None,
                        })
                        .unwrap_or(None),
                    generic_definings: processed_page
                        .items
                        .iter()
                        .filter_map(|f| match f {
                            Collecting::Generic(g) => {
                                Some(ellie_core::definite::items::class::GenericDefining {
                                    name: g.name.clone(),
                                    pos: g.pos,
                                })
                            }
                            _ => None,
                        })
                        .collect::<Vec<_>>(),
                    properties: processed_page
                        .items
                        .clone()
                        .into_iter()
                        .filter_map(|f| match f {
                            Collecting::Variable(p) => Some(p.clone()),
                            _ => None,
                        })
                        .collect::<Vec<_>>(),
                    getters: Vec::new(),
                    setters: Vec::new(),
                    methods: processed_page
                        .items
                        .clone()
                        .into_iter()
                        .filter_map(|f| match f {
                            Collecting::Function(p) => Some(p.clone()),
                            _ => None,
                        })
                        .collect::<Vec<_>>(),
                    name_pos: self.name_pos,
                    pos: self.pos,
                    hash: self.hash,
                },
            );
            parser
                .find_processed_page(page_id)
                .unwrap()
                .items
                .push(processed)
        }
    }
}
