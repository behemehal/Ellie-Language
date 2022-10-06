use std::fmt::format;

use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::types::function_call_type::FunctionCall;

impl CodeRenderer for FunctionCall {
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);
        let state_ending_token = &state.ending_token;
        let target_value = self.target.render(&State::empty_state(), options);

        let mut call = format!("{state_scope_length}{target_value}(");

        for (idx, param) in self.parameters.iter().enumerate() {
            let param = param.value.render(&State::empty_state(), options);
            call += &param;
            if idx != self.parameters.len() - 1 {
                if options.leave_space_after_comma {
                    call += ", ";
                } else {
                    call += ",";
                }
            } else {
                call += ")";
            }
        }
        format!("{call}{state_ending_token}")
    }
}
