use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::types::brace_reference_type::BraceReferenceType;

impl CodeRenderer for BraceReferenceType {
    fn render(&self, _: &State, _: &FormatterOptions) -> String {
        todo!()
    }
}
