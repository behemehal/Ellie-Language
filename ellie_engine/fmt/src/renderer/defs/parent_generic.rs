use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::definers::ParentGenericType;

impl CodeRenderer for ParentGenericType {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);
        let state_ending_token = &state.ending_token;

        let class_name = &self.parent;
        let comment = if options.leave_space_after_comma {
            ", "
        } else {
            ","
        };

        let mut generics = format!("{class_name}<");

        for (idx, rtype) in self.generics.iter().enumerate() {
            let type_of_value = rtype.value.render(&State::empty_state(), options);

            if idx != self.generics.len() - 1 {
                generics += &format!("{type_of_value}{comment}",);
            } else {
                generics += &type_of_value.to_string();
            }
        }
        format!("{state_scope_length}{generics}>{state_ending_token}")
    }
}
