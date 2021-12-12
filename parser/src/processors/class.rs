use alloc::{borrow::ToOwned, vec, vec::Vec};
use ellie_core::{definite::items::Collecting, error, warning};
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
                if !is_correct {
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
            let mut inner_dependencies = page.dependencies.clone();
            inner_dependencies.push(ellie_tokenizer::tokenizer::Dependency {
                hash: page.hash.clone(),
                public: false,
            });

            let inner_page_id: u64 = ellie_core::utils::generate_hash_u64();
            parser.pages.push(ellie_tokenizer::tokenizer::Page {
                hash: inner_page_id,
                inner: true,
                path: page.path.clone(),
                items: self.body,
                dependents: vec![],
                dependencies: inner_dependencies,
            });
            parser.process_page(inner_page_id);

            let processed_page = parser.find_processed_page(inner_page_id).unwrap();

            let processed = ellie_core::definite::items::Collecting::Class(
                ellie_core::definite::items::class::Class {
                    name: self.name,
                    public: self.public,
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