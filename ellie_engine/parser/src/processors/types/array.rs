use alloc::vec::Vec;
use alloc::{borrow::ToOwned, vec};
use ellie_core::{definite::types, error};
use ellie_tokenizer::syntax::types::array_type;

impl super::TypeParserProcessor for array_type::ArrayTypeCollector {
    fn process(
        &self,
        options: &mut super::TypeParserProcessorOptions,
    ) -> Result<types::Types, Vec<error::Error>> {
        let mut errors = vec![];
        let mut collective = vec![];

        for i in &self.data.collective {
            let mut _options = super::TypeParserProcessorOptions::new(options.parser, options.page_id);
            let response = i.value.process(
                &mut _options
                    .dont_exclude_getter()
                    
                    .dont_include_setter()
                    .build(),
            );
            if response.is_err() {
                errors.append(&mut response.unwrap_err());
            } else {
                collective.push(types::array::ArrayEntry {
                    value: response.unwrap(),
                    location: i.location,
                });
            }
        }

        if errors.is_empty() {
            if collective.len() == 0 && !options.ignore_type {
                errors.push(
                    error::error_list::ERROR_S55.clone().build_with_path(
                        vec![],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        options
                            .parser
                            .find_page(options.page_id)
                            .unwrap()
                            .path
                            .clone(),
                        self.data.pos,
                    ),
                );
                return Err(errors);
            }

            Ok(types::Types::Array(types::array::ArrayType {
                collective,
                pos: self.data.pos,
            }))
        } else {
            Err(errors)
        }
    }
}
