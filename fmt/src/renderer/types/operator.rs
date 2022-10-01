use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::types::operator_type::OperatorType;

impl CodeRenderer for OperatorType {
    fn render(&self, _: &State, _: &FormatterOptions) -> String {
        todo!()
    }
}
