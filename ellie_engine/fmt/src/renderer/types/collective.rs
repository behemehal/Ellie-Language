use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::types::collective_type::CollectiveType;

impl CodeRenderer for CollectiveType {
    fn render(&self, _: &State, _: &FormatterOptions) -> String {
        todo!()
    }
}
