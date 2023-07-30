use crate::deep_search_extensions::{deep_search_hash, resolve_type};
use alloc::{borrow::ToOwned, vec, vec::Vec};
use ellie_core::{definite::Converter, error};
use ellie_tokenizer::syntax::{
    items::setter_call::SetterCall,
    types::operator_type::{AssignmentOperators, Operators},
};

impl super::Processor for SetterCall {
    fn process(
        &self,
        parser: &mut super::Parser,
        page_idx: usize,
        processed_page_idx: usize,
        page_hash: usize,
    ) -> bool {
        let current_page = parser
            .pages
            .nth(page_idx)
            .unwrap_or_else(|| panic!("Failed to find page"))
            .clone();
        match super::type_processor::process(
            self.target.clone(),
            parser,
            page_hash,
            None,
            true,
            true,
            false,
            Some(self.target_pos),
        ) {
            Ok(target) => match target.clone() {
                ellie_core::definite::types::Types::Reference(_) => {
                    match super::type_processor::process(
                        self.value.clone(),
                        parser,
                        page_hash,
                        None,
                        false,
                        true,
                        false,
                        Some(self.target_pos),
                    ) {
                        Ok(processed_value_type) => {
                            let mut errors = Vec::new();
                            let target_type = match resolve_type(
                                target.clone(),
                                page_hash,
                                parser,
                                &mut errors,
                                Some(self.target_pos),
                            ) {
                                Some(e) => e,
                                None => {
                                    parser.informations.extend(&errors);
                                    return false;
                                }
                            };

                            let value_defining = match resolve_type(
                                processed_value_type.clone(),
                                page_hash,
                                parser,
                                &mut errors,
                                Some(self.target_pos),
                            ) {
                                Some(e) => e,
                                None => {
                                    parser.informations.extend(&errors);
                                    return false;
                                }
                            };

                            let first = target_type.to_string();
                            let second = value_defining.to_string();

                            match ellie_core::utils::operator_control(
                                Operators::AssignmentType(self.operator.clone()).to_definite(),
                                first,
                                second,
                                current_page.path,
                                self.value_pos,
                            ) {
                                Some(e) => {
                                    parser.informations.push(&e);
                                    return false;
                                }
                                None => {
                                    let page =
                                        parser.processed_pages.nth_mut(processed_page_idx).unwrap();
                                    page.items.push(ellie_core::definite::items::Collecting::SetterCall(
                                            ellie_core::definite::items::setter_call::SetterCall {
                                                target,
                                                hash: self.hash,
                                                value: processed_value_type,
                                                target_pos: self.target_pos,
                                                value_pos: self.value_pos,
                                                operator: match self.operator  {
                                                    AssignmentOperators::Assignment => ellie_core::definite::types::operator::AssignmentOperators::Assignment,
                                                    AssignmentOperators::AdditionAssignment => ellie_core::definite::types::operator::AssignmentOperators::AdditionAssignment,
                                                    AssignmentOperators::SubtractionAssignment => ellie_core::definite::types::operator::AssignmentOperators::SubtractionAssignment,
                                                    AssignmentOperators::MultiplicationAssignment => ellie_core::definite::types::operator::AssignmentOperators::MultiplicationAssignment,
                                                    AssignmentOperators::DivisionAssignment => ellie_core::definite::types::operator::AssignmentOperators::DivisionAssignment,
                                                    AssignmentOperators::ModulusAssignment => ellie_core::definite::types::operator::AssignmentOperators::ModulusAssignment,
                                                    AssignmentOperators::ExponentiationAssignment => ellie_core::definite::types::operator::AssignmentOperators::ExponentiationAssignment,
                                                    AssignmentOperators::Null => ellie_core::definite::types::operator::AssignmentOperators::Null,
                                                },
                                            },
                                        ));
                                }
                            }
                        }
                        Err(e) => {
                            parser.informations.extend(&e);
                        }
                    };
                }
                ellie_core::definite::types::Types::BraceReference(_) => {
                    match super::type_processor::process(
                        self.value.clone(),
                        parser,
                        page_hash,
                        None,
                        false,
                        false,
                        false,
                        Some(self.value_pos),
                    ) {
                        Ok(processed_value_type) => {
                            let mut errors = Vec::new();
                            let mut target_type = match resolve_type(
                                processed_value_type.clone(),
                                page_hash,
                                parser,
                                &mut errors,
                                Some(self.target_pos),
                            ) {
                                Some(e) => e,
                                None => {
                                    parser.informations.extend(&errors);
                                    return true;
                                }
                            };

                            if matches!(target_type.clone(), ellie_core::definite::definers::DefinerCollecting::ParentGeneric(e) if e.rtype == "nullAble")
                            {
                                target_type = target_type.as_parent_generic().unwrap().generics[0]
                                    .clone()
                                    .value;
                            }

                            let value_defining = match resolve_type(
                                processed_value_type.clone(),
                                page_hash,
                                parser,
                                &mut errors,
                                Some(self.target_pos),
                            ) {
                                Some(e) => e,
                                None => {
                                    parser.informations.extend(&errors);
                                    return false;
                                }
                            };

                            let first = target_type.to_string();
                            let second = value_defining.to_string();

                            match ellie_core::utils::operator_control(
                                Operators::AssignmentType(self.operator.clone()).to_definite(),
                                first,
                                second,
                                current_page.path.clone(),
                                self.value_pos,
                            ) {
                                Some(e) => {
                                    errors.push(e);
                                    parser.informations.extend(&errors);
                                    return false;
                                }
                                None => {
                                    let page =
                                        parser.processed_pages.nth_mut(processed_page_idx).unwrap();
                                    page.items.push(ellie_core::definite::items::Collecting::SetterCall(
                                            ellie_core::definite::items::setter_call::SetterCall {
                                                target,
                                                hash: self.hash,
                                                value: processed_value_type,
                                                target_pos: self.target_pos,
                                                value_pos: self.value_pos,
                                                operator: match self.operator  {
                                                    AssignmentOperators::Assignment => ellie_core::definite::types::operator::AssignmentOperators::Assignment,
                                                    AssignmentOperators::AdditionAssignment => ellie_core::definite::types::operator::AssignmentOperators::AdditionAssignment,
                                                    AssignmentOperators::SubtractionAssignment => ellie_core::definite::types::operator::AssignmentOperators::SubtractionAssignment,
                                                    AssignmentOperators::MultiplicationAssignment => ellie_core::definite::types::operator::AssignmentOperators::MultiplicationAssignment,
                                                    AssignmentOperators::DivisionAssignment => ellie_core::definite::types::operator::AssignmentOperators::DivisionAssignment,
                                                    AssignmentOperators::ModulusAssignment => ellie_core::definite::types::operator::AssignmentOperators::ModulusAssignment,
                                                    AssignmentOperators::ExponentiationAssignment => ellie_core::definite::types::operator::AssignmentOperators::ExponentiationAssignment,
                                                    AssignmentOperators::Null => ellie_core::definite::types::operator::AssignmentOperators::Null,
                                                },
                                            },
                                        ));
                                }
                            }
                        }
                        Err(e) => {
                            parser.informations.extend(&e);
                        }
                    };
                }
                ellie_core::definite::types::Types::VariableType(variable_reference) => {
                    let deep_type = deep_search_hash(
                        parser,
                        page_hash,
                        variable_reference.reference,
                        vec![],
                        0,
                    );
                    if deep_type.found {
                        match deep_type.found_item {
                            crate::deep_search_extensions::ProcessedDeepSearchItems::Variable(
                                variable,
                            ) => {
                                if variable.constant {
                                    parser.informations.push(
                                        &ellie_core::error::error_list::ERROR_S18
                                            .clone()
                                            .build_with_path(
                                                vec![],
                                                alloc::format!(
                                                    "{}:{}:{}",
                                                    file!().to_owned(),
                                                    line!(),
                                                    column!()
                                                ),
                                                current_page.path.clone(),
                                                self.target_pos,
                                            ),
                                    );
                                } else {
                                    match super::type_processor::process(
                                        self.value.clone(),
                                        parser,
                                        page_hash,
                                        None,
                                        false,
                                        false,
                                        false,
                                        Some(self.value_pos),
                                    ) {
                                        Ok(processed_value_type) => {
                                            let mut errors = Vec::new();

                                            let target_type = if variable.has_type {
                                                variable.rtype
                                            } else {
                                                match resolve_type(
                                                    variable.value,
                                                    page_hash,
                                                    parser,
                                                    &mut errors,
                                                    Some(variable.value_pos),
                                                ) {
                                                    Some(e) => e,
                                                    None => {
                                                        parser.informations.extend(&errors);
                                                        return false;
                                                    }
                                                }
                                            };

                                            let value_defining = match resolve_type(
                                                processed_value_type.clone(),
                                                page_hash,
                                                parser,
                                                &mut errors,
                                                Some(self.target_pos),
                                            ) {
                                                Some(e) => e,
                                                None => {
                                                    parser.informations.extend(&errors);
                                                    return false;
                                                }
                                            };

                                            let first = target_type.to_string();
                                            let second = value_defining.to_string();

                                            match ellie_core::utils::operator_control(
                                                Operators::AssignmentType(self.operator.clone())
                                                    .to_definite(),
                                                first,
                                                second,
                                                current_page.path.clone(),
                                                self.value_pos,
                                            ) {
                                                Some(e) => {
                                                    errors.push(e);
                                                    parser.informations.extend(&errors);
                                                    return false;
                                                }
                                                None => {
                                                    let page = parser
                                                        .processed_pages
                                                        .nth_mut(processed_page_idx)
                                                        .unwrap();
                                                    page.items.push(ellie_core::definite::items::Collecting::SetterCall(
                                                            ellie_core::definite::items::setter_call::SetterCall {
                                                                target,
                                                                hash: self.hash,
                                                                value: processed_value_type,
                                                                target_pos: self.target_pos,
                                                                value_pos: self.value_pos,
                                                                operator: match self.operator  {
                                                                    AssignmentOperators::Assignment => ellie_core::definite::types::operator::AssignmentOperators::Assignment,
                                                                    AssignmentOperators::AdditionAssignment => ellie_core::definite::types::operator::AssignmentOperators::AdditionAssignment,
                                                                    AssignmentOperators::SubtractionAssignment => ellie_core::definite::types::operator::AssignmentOperators::SubtractionAssignment,
                                                                    AssignmentOperators::MultiplicationAssignment => ellie_core::definite::types::operator::AssignmentOperators::MultiplicationAssignment,
                                                                    AssignmentOperators::DivisionAssignment => ellie_core::definite::types::operator::AssignmentOperators::DivisionAssignment,
                                                                    AssignmentOperators::ModulusAssignment => ellie_core::definite::types::operator::AssignmentOperators::ModulusAssignment,
                                                                    AssignmentOperators::ExponentiationAssignment => ellie_core::definite::types::operator::AssignmentOperators::ExponentiationAssignment,
                                                                    AssignmentOperators::Null => ellie_core::definite::types::operator::AssignmentOperators::Null,
                                                                },
                                                            },
                                                        ));
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            parser.informations.extend(&e);
                                        }
                                    }
                                }
                            }
                            _ => {
                                unreachable!("Parser should have prevented this");
                            }
                        }
                    } else {
                        unreachable!("Parser should have prevented this");
                    }
                }
                ellie_core::definite::types::Types::SetterCall(setter_type) => {
                    match super::type_processor::process(
                        self.value.clone(),
                        parser,
                        page_hash,
                        None,
                        false,
                        false,
                        false,
                        Some(self.value_pos),
                    ) {
                        Ok(processed_value_type) => {
                            let mut errors = Vec::new();
                            let target_type = match resolve_type(
                                target.clone(),
                                page_hash,
                                parser,
                                &mut errors,
                                Some(self.target_pos),
                            ) {
                                Some(e) => e,
                                None => {
                                    parser.informations.extend(&errors);
                                    return false;
                                }
                            };

                            let value_defining = match resolve_type(
                                processed_value_type.clone(),
                                page_hash,
                                parser,
                                &mut errors,
                                Some(self.target_pos),
                            ) {
                                Some(e) => e,
                                None => {
                                    parser.informations.extend(&errors);
                                    return false;
                                }
                            };

                            let first = target_type.to_string();
                            let second = value_defining.to_string();

                            match ellie_core::utils::operator_control(
                                Operators::AssignmentType(self.operator.clone()).to_definite(),
                                first,
                                second,
                                current_page.path,
                                self.value_pos,
                            ) {
                                Some(e) => {
                                    parser.informations.push(&e);
                                    return false;
                                }
                                None => {
                                    let page =
                                        parser.processed_pages.nth_mut(processed_page_idx).unwrap();
                                    page.items.push(ellie_core::definite::items::Collecting::SetterCall(
                                            ellie_core::definite::items::setter_call::SetterCall {
                                                target,
                                                value: processed_value_type,
                                                target_pos: self.target_pos,
                                                hash: self.hash,
                                                value_pos: self.value_pos,
                                                operator: match self.operator  {
                                                    AssignmentOperators::Assignment => ellie_core::definite::types::operator::AssignmentOperators::Assignment,
                                                    AssignmentOperators::AdditionAssignment => ellie_core::definite::types::operator::AssignmentOperators::AdditionAssignment,
                                                    AssignmentOperators::SubtractionAssignment => ellie_core::definite::types::operator::AssignmentOperators::SubtractionAssignment,
                                                    AssignmentOperators::MultiplicationAssignment => ellie_core::definite::types::operator::AssignmentOperators::MultiplicationAssignment,
                                                    AssignmentOperators::DivisionAssignment => ellie_core::definite::types::operator::AssignmentOperators::DivisionAssignment,
                                                    AssignmentOperators::ModulusAssignment => ellie_core::definite::types::operator::AssignmentOperators::ModulusAssignment,
                                                    AssignmentOperators::ExponentiationAssignment => ellie_core::definite::types::operator::AssignmentOperators::ExponentiationAssignment,
                                                    AssignmentOperators::Null => ellie_core::definite::types::operator::AssignmentOperators::Null,
                                                },
                                            },
                                        ));
                                }
                            }
                        }
                        Err(e) => {
                            parser.informations.extend(&e);
                        }
                    }
                }
                _ => {
                    parser.informations.push(
                        &ellie_core::error::error_list::ERROR_S43
                            .clone()
                            .build_with_path(
                                vec![],
                                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                                current_page.path.clone(),
                                self.target_pos,
                            ),
                    );
                }
            },
            Err(e) => {
                parser.informations.extend(&e);
            }
        }
        true
    }
}
