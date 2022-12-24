use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::types::null_resolver::NullResolver;

impl CodeRenderer for NullResolver {
    fn render(&self, _: &State, _: &FormatterOptions) -> String {
        todo!()
    }
}
