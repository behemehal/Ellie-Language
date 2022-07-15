use crate::processors::type_processor::process;
use alloc::{borrow::ToOwned, vec, vec::Vec};
use ellie_core::{definite::definers::DefinerCollecting, error};
use ellie_tokenizer::{syntax::items::loop_type::Loop, tokenizer::PageType};

impl super::Processor for Loop {
    fn process(
        &self,
        parser: &mut super::Parser,
        page_idx: usize,
        processed_page_idx: usize,
        page_hash: usize,
    ) -> bool {
        let page = parser.pages.nth(page_idx).unwrap().clone();
        let path = page.path.clone();
        let condition = match process(
            self.condition.current.clone(),
            parser,
            page_hash,
            None,
            false,
            false,
            false,
        ) {
            Ok(rtype) => rtype,
            Err(e) => {
                parser.informations.extend(&e);
                return false;
            }
        };

        let mut errors: Vec<error::Error> = vec![];

        let target_condition = match crate::deep_search_extensions::resolve_type(
            condition.clone(),
            page_hash,
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

        match &target_condition {
            ellie_core::definite::definers::DefinerCollecting::Generic(e) => {
                if e.rtype != "bool" {
                    parser.informations.push(
                        &error::error_list::ERROR_S29.clone().build_with_path(
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: target_condition.to_string(),
                            }],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            path,
                            self.condition_pos,
                        ),
                    );
                }
            }
            _ => {
                parser
                    .informations
                    .push(&error::error_list::ERROR_S29.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: target_condition.to_string(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        path,
                        self.iterator_pos,
                    ));
            }
        }

        let page = parser.pages.nth(page_idx).unwrap().clone();
        let path = page.path.clone();

        let inner_page_id: usize = ellie_core::utils::generate_hash_usize();
        let mut dependencies = vec![ellie_tokenizer::tokenizer::Dependency {
            hash: page.hash.clone(),
            processed: false,
            module: None,
            deep_link: Some(page.hash.clone()),
            public: false,
        }];

        dependencies.extend(page.dependencies);
        let inner = ellie_tokenizer::tokenizer::Page {
            hash: inner_page_id,
            inner: Some(page.hash),
            path: page.path.clone(),
            page_type: PageType::LoopBody,
            items: self.body.clone(),
            dependents: vec![],
            dependencies,
            ..Default::default()
        };
        parser.pages.push_page(inner);
        let processed_page = parser.processed_pages.nth_mut(processed_page_idx).unwrap();
        processed_page
            .items
            .push(ellie_core::definite::items::Collecting::Loop(
                ellie_core::definite::items::loop_type::Loop {
                    body_pos: self.body_pos,
                    inner_page_id,
                    pos: self.pos,
                    condition,
                    hash: self.hash,
                },
            ));
        parser.process_page(inner_page_id);
        true
    }
}
