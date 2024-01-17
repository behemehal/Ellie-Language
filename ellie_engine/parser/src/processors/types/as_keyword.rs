use crate::processors::definer::{DefinerParserProcessor, DefinerParserProcessorOptions};
use alloc::{boxed::Box, vec::Vec};
use ellie_core::{definite::types, error};
use ellie_tokenizer::syntax::types::as_keyword;

impl super::TypeParserProcessor for as_keyword::AsKeywordCollector {
    fn process(
        &self,
        options: &mut super::TypeParserProcessorOptions,
    ) -> Result<types::Types, Vec<error::Error>> {
        let mut _options = super::TypeParserProcessorOptions::new(options.parser, options.page_id);

        match self.data.target.process(
            &mut _options
                .dont_exclude_getter()
                .dont_include_setter()
                .dont_ignore_type()
                .build(),
        ) {
            Ok(resolved_types) => {
                match self.data.rtype.definer_type.process(
                    &mut DefinerParserProcessorOptions::new(options.parser, options.page_id)
                        .optional_ignore_hash(options.ignore_hash)
                        .build(),
                ) {
                    Ok(resolved_definer) => {
                        Ok(types::Types::AsKeyword(types::as_keyword::AsKeyword {
                            target: Box::new(resolved_types),
                            pos: self.data.pos,
                            target_pos: self.data.target_pos,
                            type_pos: self.data.type_pos,
                            rtype: resolved_definer,
                        }))
                    }
                    Err(type_errors) => Err(type_errors),
                }
            }
            Err(val_errors) => Err(val_errors),
        }
    }
}
