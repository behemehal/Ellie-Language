use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::types::as_keyword::AsKeyword;

impl CodeRenderer for AsKeyword {
    fn render(&self, _: &State, _: &FormatterOptions) -> String {
        todo!()
    }
}
