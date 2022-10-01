use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::types::float_type::FloatType;

impl CodeRenderer for FloatType {
    //Renderer Options
        fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space();
        let state_ending_token = &state.ending_token;
        let value = self.value;
        let starter = if self.raw.starts_with(".") || options.float_starts_with_dot {
            "."
        } else {
            "0."
        };
        format!("{state_scope_length}{starter}{value}{state_ending_token}")
    }
}
