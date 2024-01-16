use alloc::boxed::Box;
use alloc::{borrow::ToOwned, vec, vec::Vec};
use ellie_core::error;
use ellie_core::utils::generate_hash_usize;
use ellie_tokenizer::syntax::types::{
    reference_type::{Chain, ReferenceType, ReferenceTypeCollector},
    variable_type::{VariableType, VariableTypeCollector},
};
use ellie_tokenizer::{
    processors::items::Processors,
    processors::types::Processors as TypeProcessor,
    syntax::items::{
        constructor::Constructor, constructor_parameter::ConstructorParameter,
        setter_call::SetterCall,
    },
    syntax::types::operator_type::AssignmentOperators,
    tokenizer::PageType,
};

impl super::Processor for Constructor {
    fn process(
        &self,
        parser: &mut super::Parser,
        page_idx: usize,
        processed_page_idx: usize,
        page_hash: usize,
    ) -> bool {
        let class_body_page = parser
            .pages
            .nth(page_idx)
            .unwrap_or_else(|| panic!("Failed to find page"))
            .clone();
        let path = class_body_page.path.clone();

        let page = parser.pages.nth(page_idx).unwrap().clone();

        let class_page_type = match page.page_type {
            PageType::ClassBody(e) => e,
            _ => unreachable!(),
        };

        //Class body should have a self which will reference us page of class and class hash
        let class_instance_element = class_body_page
            .items
            .iter()
            .find_map(|item| match item {
                ellie_tokenizer::processors::items::Processors::ClassInstance(e) => Some(e),
                _ => None,
            })
            .unwrap_or_else(|| panic!("Failed to find self"));

        //Get the page class belongs
        let class_page = parser
            .find_page(class_instance_element.class_page)
            .unwrap_or_else(|| panic!("Failed to class page"))
            .clone();

        //Get the element future use
        let class_element = class_page
            .items
            .iter()
            .find_map(|item| match item {
                ellie_tokenizer::processors::items::Processors::Class(e) => {
                    if e.hash == class_instance_element.class_hash {
                        Some(e)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("Failed to find class"));

        let mut dependencies = vec![ellie_tokenizer::tokenizer::Dependency {
            hash: page.hash,
            processed: false,
            module: None,
            deep_link: Some(page.hash),
            public: false,
        }];
        dependencies.extend(page.dependencies);

        let mut items = Vec::new();

        items.push(
            ellie_tokenizer::processors::items::Processors::ConstructorParameter(
                ConstructorParameter {
                    name: "self".to_owned(),
                    rtype: ellie_core::definite::definers::DefinerCollecting::Generic(
                        ellie_core::definite::definers::GenericType {
                            rtype: "self".to_owned(),
                            pos: class_page_type.pos,
                            hash: class_page_type.hash,
                        },
                    ),
                    hash: generate_hash_usize(),
                    pos: self.pos,
                },
            ),
        );

        for variable in class_element
            .body
            .iter()
            .filter_map(|item| match item.as_variable() {
                Some(e) => e.data.has_value.then(|| e),
                None => None,
            })
        {
            let self_setter = Processors::SetterCall(SetterCall {
                target: TypeProcessor::Reference(ReferenceTypeCollector {
                    data: ReferenceType {
                        reference: Box::new(TypeProcessor::Variable(VariableTypeCollector {
                            data: VariableType {
                                value: "self".to_owned(),
                                ..Default::default()
                            },
                            ..Default::default()
                        })),
                        chain: vec![Chain {
                            value: variable.data.name.clone(),
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                value: variable.data.value.clone(),
                operator: AssignmentOperators::Assignment,
                hash: generate_hash_usize(),
                ..Default::default()
            });
            items.push(self_setter);
        }

        for (index, parameter) in self.parameters.clone().iter().enumerate() {
            if class_element.name == parameter.name {
                let mut err = error::error_list::ERROR_S24.clone().build_with_path(
                    vec![],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    path.clone(),
                    parameter.pos,
                );
                err.reference_block = Some((class_element.pos, path.clone()));
                err.reference_message = "Prime is here".to_owned();
                err.semi_assist = true;
                parser.informations.push(&err);
                return false;
            }

            if let Some(other_index) = self
                .parameters
                .iter()
                .position(|g| g.name == parameter.name)
            {
                if other_index < index {
                    let mut err = error::error_list::ERROR_S10.clone().build_with_path(
                        vec![],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        path.clone(),
                        parameter.pos,
                    );
                    err.reference_block = Some((self.parameters[other_index].pos, path.clone()));
                    err.reference_message = "Prime is here".to_owned();
                    err.semi_assist = true;
                    parser.informations.push(&err);
                }
            }
            let mut param_found = false;
            let mut found_is_constant_variable = None;
            let page = parser.find_page(page_hash).unwrap();
            for item in page.items.iter() {
                match item {
                    Processors::Variable(e) => {
                        if e.data.constant {
                            found_is_constant_variable = Some(parameter.pos);
                        }
                        param_found = true;
                    }
                    _ => (),
                }
            }

            if !param_found {
                let mut err = error::error_list::ERROR_S34.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: parameter.name.clone(),
                    }],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    path.clone(),
                    parameter.pos,
                );
                err.reference_block = Some((class_element.pos, class_page.path.clone()));
                err.reference_message = "Class body is here".to_owned();
                parser.informations.push(&err);
            }

            if found_is_constant_variable.is_some() {
                parser
                    .informations
                    .push(&error::error_list::ERROR_S18.clone().build_with_path(
                        vec![],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        class_body_page.path.clone(),
                        found_is_constant_variable.unwrap(),
                    ));
            }
        }
        items.extend(self.inside_code.clone());
        let inner_page_id: usize = ellie_core::utils::generate_hash_usize();
        let inner = ellie_tokenizer::tokenizer::Page {
            hash: inner_page_id,
            inner: Some(class_body_page.hash),
            path: class_body_page.path.clone(),
            items,
            page_type: PageType::ConstructorBody,
            dependents: vec![],
            dependencies,
            ..Default::default()
        };
        parser.pages.push_page(inner);

        let processed = ellie_core::definite::items::Collecting::Constructor(
            ellie_core::definite::items::constructor::Constructor {
                parameters: self
                    .parameters
                    .clone()
                    .into_iter()
                    .map(
                        |x| ellie_core::definite::items::constructor::ConstructorParameter {
                            name: x.name,
                            pos: x.pos,
                        },
                    )
                    .collect(),
                name_pos: self.name_pos,
                parameters_pos: self.parameters_pos,
                pos: self.pos,
                inner_page_id,
                class_hash: class_element.hash,
            },
        );
        parser
            .processed_pages
            .nth_mut(processed_page_idx)
            .unwrap()
            .items
            .push(processed);
        true
    }
}
