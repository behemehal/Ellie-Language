use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::types::function_call_type::FunctionCall;

impl CodeRenderer for FunctionCall {
    fn render(&self, _: &State, _: &FormatterOptions) -> String {
        todo!()
    }
}
