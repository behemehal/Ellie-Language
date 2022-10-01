use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::definers::FunctionType;

impl CodeRenderer for FunctionType {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space();
        let state_ending_token = &state.ending_token;

        let comment = if options.leave_space_after_comma {
            ", "
        } else {
            ","
        };

        let mut generics = format!("@(");

        for (idx, rtype) in self.params.iter().enumerate() {
            let type_of_value = rtype.render(&State::empty_state(), options);

            if idx != self.params.len() - 1 {
                generics += &format!("{type_of_value}{comment}",);
            } else {
                generics += &format!("{type_of_value}");
            }
        }

        let return_value = self.returning.render(&State::empty_state(), options);

        format!("{state_scope_length}{generics}):{return_value}{state_ending_token}")
    }
}
