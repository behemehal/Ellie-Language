use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::types::as_keyword::AsKeyword;

impl CodeRenderer for AsKeyword {
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);
        let state_ending_token = &state.ending_token;
        let value = self.target.render(&State::empty_state(), options);
        let type_value = self
            .rtype
            .definer_type
            .render(&State::empty_state(), options);
        format!("{state_scope_length}{value} as {type_value}{state_ending_token}")
    }
}
