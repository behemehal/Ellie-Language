use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::definers::CollectiveType;

impl CodeRenderer for CollectiveType {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space();
        let state_ending_token = &state.ending_token;

        let key_type = self.key.render(&State::empty_state(), options);
        let value_type = self.value.render(&State::empty_state(), options);

        let comment = if options.leave_space_after_comma {
            ", "
        } else {
            ","
        };
        //{[type], [size]}
        format!(
            "{state_scope_length}{{{key_type}{comment}{value_type}}}{state_ending_token}"
        )
    }
}
