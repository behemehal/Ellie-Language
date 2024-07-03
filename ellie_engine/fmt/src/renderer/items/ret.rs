use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::ret::Ret;

impl CodeRenderer for Ret {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);
        let line_ending = &options.render_line_ending();
        let value_of_ret = self.value.current.render(
            &State {
                scope_length: 0,
                ending_token: String::new(),
            },
            options,
        );
        format!("{state_scope_length}ret {value_of_ret};{line_ending}")
    }
}
