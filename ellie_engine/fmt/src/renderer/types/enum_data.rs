use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::types::enum_data::EnumData;

impl CodeRenderer for EnumData {
    fn render(&self, _: &State, _: &FormatterOptions) -> String {
        todo!()
    }
}
