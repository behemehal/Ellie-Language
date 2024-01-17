use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::{borrow::ToOwned, vec};
use ellie_core::definite::Converter;
use ellie_core::{definite::types, error};
use ellie_tokenizer::syntax::types::operator_type;
use ellie_tokenizer::syntax::types::operator_type::Operators;

use crate::deep_search_extensions::resolve_type;

impl super::TypeParserProcessor for operator_type::OperatorTypeCollector {
    fn process(
        &self,
        options: &mut super::TypeParserProcessorOptions,
    ) -> Result<types::Types, Vec<error::Error>> {
        let mut errors = vec![];

        let mut _options = super::TypeParserProcessorOptions::new(options.parser, options.page_id);

        let processed_first_value = self.data.first.process(
            _options
                .dont_exclude_getter()
                .dont_include_setter()
                .dont_ignore_type()
                .build(),
        );

        let processed_second_value = self.data.second.process(
            _options
                .clone()
                .dont_exclude_getter()
                .dont_include_setter()
                .dont_ignore_type()
                .build(),
        );

        if processed_first_value.is_err() || processed_second_value.is_err() {
            if processed_first_value.is_err() {
                errors.append(&mut processed_first_value.unwrap_err());
            }
            if processed_second_value.is_err() {
                errors.append(&mut processed_second_value.unwrap_err());
            }
            return Err(errors);
        }

        let _first_value = match resolve_type(
            processed_first_value.clone().unwrap(),
            options.page_id,
            options.parser,
            &mut errors,
            Some(self.data.first_pos),
        ) {
            Some(e) => e,
            None => return Err(errors),
        };
        let _second_value = match resolve_type(
            processed_second_value.clone().unwrap(),
            options.page_id,
            options.parser,
            &mut errors,
            Some(self.data.second_pos),
        ) {
            Some(e) => e,
            None => return Err(errors),
        };

        let first = _first_value.to_string();
        let second = _second_value.to_string();

        if let Operators::AssignmentType(_) = self.data.operator {
            if !self.data.first.is_assignable() {
                return Err(vec![error::error_list::ERROR_S43.clone().build_with_path(
                    vec![],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    options
                        .parser
                        .find_page(options.page_id)
                        .unwrap()
                        .path
                        .clone(),
                    self.data.pos,
                )]);
            }
        }

        let page_id = options.page_id;

        match ellie_core::utils::operator_control(
            self.data.operator.clone().to_definite(),
            first,
            second,
            options.parser.find_page(page_id).unwrap().path.clone(),
            self.data.pos,
        ) {
            Some(e) => {
                errors.push(e);
                Err(errors)
            }
            None => Ok(types::Types::Operator(types::operator::OperatorType {
                cloaked: false,
                first: Box::new(processed_first_value.unwrap()),
                first_pos: self.data.first_pos,
                second_pos: self.data.second_pos,
                second: Box::new(processed_second_value.unwrap()),
                operator: self.data.operator.clone().to_definite(),
                pos: self.data.pos,
            })),
        }
    }
}
