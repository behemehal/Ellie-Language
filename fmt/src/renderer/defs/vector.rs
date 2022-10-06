use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::definers::VectorType;

impl CodeRenderer for VectorType {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);
        let state_ending_token = &state.ending_token;

        let type_of_value = self.rtype.render(&State::empty_state(), options);
        let comment = if options.leave_space_after_comma {
            ", "
        } else {
            ","
        };
        
        //[[type], *]
        format!(
            "{state_scope_length}[{type_of_value}{comment}*]{state_ending_token}"
        )
    }
}
