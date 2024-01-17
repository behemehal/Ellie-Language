use alloc::vec;
use alloc::vec::Vec;
use ellie_core::{definite::types, error};
use ellie_tokenizer::syntax::types::cloak_type;

impl super::TypeParserProcessor for cloak_type::CloakTypeCollector {
    fn process(
        &self,
        options: &mut super::TypeParserProcessorOptions,
    ) -> Result<types::Types, Vec<error::Error>> {
        let mut errors = Vec::new();

        if self.data.collective.len() == 1 {
            let first_entry = self.data.collective.first().unwrap();
            let mut _options =
                super::TypeParserProcessorOptions::new(options.parser, options.page_id);

            first_entry.value.process(
                _options
                    .dont_exclude_getter()
                    .dont_include_setter()
                    .dont_ignore_type()
                    .build(),
            )
        } else {
            let mut collective = vec![];

            for i in &self.data.collective {
                let mut _options =
                    super::TypeParserProcessorOptions::new(options.parser, options.page_id);
                let response = i
                    .value
                    .process(_options.dont_exclude_getter().dont_include_setter().build());

                if response.is_err() {
                    errors.append(&mut response.unwrap_err());
                } else {
                    collective.push(types::cloak::CloakEntry {
                        value: response.unwrap(),
                        location: i.location,
                    });
                }
            }

            if errors.is_empty() {
                //TODO: Type helper
                //if collective.len() == 0 && !ignore_type {
                //    errors.push(error::error_list::ERROR_S55.clone().build_with_path(
                //        vec![],
                //        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                //        parser.find_page(page_id).unwrap().path.clone(),
                //        from.get_pos(),
                //    ));
                //    return Err(errors);
                //}

                Ok(types::Types::Cloak(types::cloak::CloakType {
                    collective,
                    pos: self.data.pos,
                }))
            } else {
                Err(errors)
            }
        }
    }
}
