use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::brk::Brk;

impl CodeRenderer for Brk {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);
        let line_ending = &options.render_line_ending();
        format!("{state_scope_length}brk;{line_ending}")
    }
}
