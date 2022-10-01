use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::types::cloak_type::CloakType;

impl CodeRenderer for CloakType {
    fn render(&self, _: &State, _: &FormatterOptions) -> String {
        todo!()
    }
}
