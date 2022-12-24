use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::file_key::FileKey;

impl CodeRenderer for FileKey {
    //Renderer Options
    fn render(&self, _state: &State, _options: &FormatterOptions) -> String {
        todo!()
    }
}
