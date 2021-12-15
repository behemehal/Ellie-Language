use alloc::{borrow::ToOwned, vec, vec::Vec};
use ellie_core::{definite::items::Collecting, defs, error, warning};
use ellie_tokenizer::syntax::items::constructor::Constructor;

impl super::Processor for Constructor {
    fn process(self, parser: &mut crate::parser::Parser, page_id: u64) {
        let class_body_page = parser
            .find_page(page_id)
            .unwrap_or_else(|| panic!("Failed to find page"));

        //Class body should have a self which will reference us page of class and class hash
        let self_element = class_body_page
            .items
            .iter()
            .find_map(|item| match item {
                ellie_tokenizer::processors::items::Processors::SelfItem(e) => Some(e),
                _ => None,
            })
            .unwrap_or_else(|| panic!("Failed to find self"));

        //Get the page class belongs
        let class_page = parser
            .find_page(self_element.class_page)
            .unwrap_or_else(|| panic!("Failed to class page"))
            .clone();

        //Get the element future use
        let class_element = class_page
            .items
            .iter()
            .find_map(|item| match item {
                ellie_tokenizer::processors::items::Processors::Class(e) => Some(e),
                _ => None,
            })
            .unwrap_or_else(|| panic!("Failed to find class"));
        let page = parser.find_page(page_id).unwrap().clone();
        let mut items = self.inside_code;

        for (index, parameter) in self.parameters.clone().iter().enumerate() {
            let deep_search = parser.deep_search(page_id, parameter.name.clone(), None, vec![], 0);

            if let Some(other_index) = self
                .parameters
                .iter()
                .position(|g| g.name == parameter.name)
            {
                if other_index < index {
                    let mut err = error::error_list::ERROR_S10.clone().build_with_path(
                        vec![],
                        "pcls_0x74".to_owned(),
                        parser.find_page(page_id).unwrap().path.clone(),
                        parameter.pos,
                    );
                    err.reference_block =
                        Some((self.parameters[other_index].pos, page.path.clone()));
                    err.reference_message = "Prime is here".to_owned();
                    err.semi_assist = true;
                    parser.informations.push(&err);
                }
            }

            if deep_search.found {
                items.push(ellie_tokenizer::processors::items::Processors::ConstructorParameter(
                    ellie_tokenizer::syntax::items::constructor_parameter::ConstructorParameter {
                        name: parameter.name.clone(),
                        pos: parameter.pos.clone(),
                    },
                ));
                match deep_search.found_item {
                    crate::parser::DeepSearchItems::Variable(_) => (),
                    crate::parser::DeepSearchItems::BrokenPageGraph => (),
                    crate::parser::DeepSearchItems::MixUp(_) => (),
                    crate::parser::DeepSearchItems::Class(_) => (),
                    crate::parser::DeepSearchItems::ImportReference(_) => (),
                    crate::parser::DeepSearchItems::None => (),
                    _ => {
                        parser.informations.push(
                            &error::error_list::ERROR_S11.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: parameter.name.clone(),
                                }],
                                "pvr_0x23".to_owned(),
                                parser.find_page(page_id).unwrap().path.clone(),
                                parameter.pos,
                            ),
                        );
                    }
                }
            } else {
                let mut err = error::error_list::ERROR_S34.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: parameter.name.clone(),
                    }],
                    "pvr_0x23".to_owned(),
                    parser.find_page(page_id).unwrap().path.clone(),
                    parameter.pos,
                );
                err.reference_block = Some((class_element.pos, class_page.path.clone()));
                err.reference_message = "Class body is here".to_owned();
                parser.informations.push(&err);
            }
        }

        let inner_page_id: u64 = ellie_core::utils::generate_hash_u64();
        let inner = ellie_tokenizer::tokenizer::Page {
            hash: inner_page_id,
            inner: Some(page.hash),
            path: page.path.clone(),
            items,
            dependents: vec![],
            dependencies: vec![ellie_tokenizer::tokenizer::Dependency {
                hash: page.hash.clone(),
                public: false,
            }],
        };
        parser.pages.push(inner);
        parser.process_page(inner_page_id);

        let processed_page = parser.find_processed_page(inner_page_id).unwrap();

        let processed = ellie_core::definite::items::Collecting::Constructor(
            ellie_core::definite::items::constructor::Constructor {
                parameters: self
                    .parameters
                    .into_iter()
                    .map(
                        |x| ellie_core::definite::items::constructor::ConstructorParameter {
                            name: x.name,
                            pos: x.pos,
                        },
                    )
                    .collect(),
                inside_code: processed_page.items.clone(),
                name_pos: self.name_pos,
                parameters_pos: self.parameters_pos,
                pos: self.pos,
                inner_page_id,
            },
        );
        parser
            .find_processed_page(page_id)
            .unwrap()
            .items
            .push(processed)
    }
}
