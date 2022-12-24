use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::types::class_call_type::ClassCall;

impl CodeRenderer for ClassCall {
    fn render(&self, _: &State, _: &FormatterOptions) -> String {
        todo!()
    }
}
