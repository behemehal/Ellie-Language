use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::getter_call::GetterCall;

impl CodeRenderer for GetterCall {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);
        let line_ending = &options.render_line_ending();
        let type_value = self.data.render(&State::empty_state(), options);
        format!("{state_scope_length}{type_value};{line_ending}")
    }
}
