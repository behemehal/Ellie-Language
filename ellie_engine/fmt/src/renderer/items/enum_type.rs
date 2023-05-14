use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::enum_type::EnumType;

impl CodeRenderer for EnumType {
    //Renderer Options
    fn render(&self, _state: &State, _options: &FormatterOptions) -> String {
        todo!()
    }
}
