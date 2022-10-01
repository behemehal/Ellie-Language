use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::types::reference_type::ReferenceType;

impl CodeRenderer for ReferenceType {
    fn render(&self, _: &State, _: &FormatterOptions) -> String {
        todo!()
    }
}
