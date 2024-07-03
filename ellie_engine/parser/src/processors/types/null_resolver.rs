use alloc::{boxed::Box, vec::Vec};
use ellie_core::{definite::types, error};
use ellie_tokenizer::syntax::types::null_resolver;

impl super::TypeParserProcessor for null_resolver::NullResolver {
    fn process(
        &self,
        options: &mut super::TypeParserProcessorOptions,
    ) -> Result<types::Types, Vec<error::Error>> {
        let mut _options = super::TypeParserProcessorOptions::new(options.parser, options.page_id);
        match self.target.process(
            _options
                .dont_exclude_getter()
                .dont_include_setter()
                .dont_ignore_type()
                .build(),
        ) {
            Ok(resolved_types) => Ok(types::Types::NullResolver(
                types::null_resolver::NullResolver {
                    target: Box::new(resolved_types),
                    pos: self.pos,
                    target_pos: self.target_pos,
                },
            )),
            Err(val_errors) => Err(val_errors),
        }
    }
}
