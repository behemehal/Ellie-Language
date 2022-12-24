use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::self_item::SelfItem;

impl CodeRenderer for SelfItem {
    //Renderer Options
    fn render(&self, _state: &State, _options: &FormatterOptions) -> String {
        todo!()
    }
}
