use alloc::vec::Vec;
use alloc::{borrow::ToOwned, vec};
use ellie_core::{definite::types, error};
use ellie_tokenizer::syntax::types::negative_type;

impl super::TypeParserProcessor for negative_type::Negative {
    fn process(
        &self,
        options: &mut super::TypeParserProcessorOptions,
    ) -> Result<types::Types, Vec<error::Error>> {
        match self.value.process(options) {
            Ok(value) => match value {
                types::Types::Byte(_)
                | types::Types::Integer(_)
                | types::Types::Decimal(_)
                | types::Types::Bool(_)
                | types::Types::Negative(_) => {
                    Ok(types::Types::Bool(types::bool::BoolType { value: true }))
                }
                _ => Err(vec![error::error_list::ERROR_S66.clone().build_with_path(
                    vec![],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    options
                        .parser
                        .find_page(options.page_id)
                        .unwrap()
                        .path
                        .clone(),
                    self.pos,
                )]),
            },
            Err(e) => Err(e),
        }
    }
}
