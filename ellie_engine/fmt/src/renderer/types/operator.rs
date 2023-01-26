use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::types::operator_type::OperatorType;

impl CodeRenderer for OperatorType {
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);
        let state_ending_token = &state.ending_token;
        let first = self.first.render(state, options);
        let second = self.second.render(state, options);

        if options.space_between_operators {
            format!(
                "{state_scope_length}{first} {operator} {second}{state_ending_token}",
                first = first,
                operator = self.operator.to_defining(),
                second = second,
                state_scope_length = state_scope_length,
                state_ending_token = state_ending_token
            )
        } else {
            format!(
                "{state_scope_length}{first}{operator}{second}{state_ending_token}",
                first = first,
                operator = self.operator.to_defining(),
                second = second,
                state_scope_length = state_scope_length,
                state_ending_token = state_ending_token
            )
        }
    }
}
