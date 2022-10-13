use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::definers::ArrayType;

impl CodeRenderer for ArrayType {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);
        let state_ending_token = &state.ending_token;

        let type_of_value = self.rtype.render(&State::empty_state(), options);
        let size_of_value = match &self.size {
            Some(rtype) => match &**rtype {
                ellie_core::definite::types::Types::Integer(e) => {
                    let comment = if self.has_size {
                        if options.leave_space_after_comma {
                            ", "
                        } else {
                            ","
                        }
                    } else {
                        ""
                    };
                    format!("{}{comment}", e.value)
                }
                _ => unreachable!("Array size must be integer"),
            },
            None => String::new(),
        };

        //[[type], [size]]
        //OR
        //[[type]]
        format!("{state_scope_length}[{type_of_value}{size_of_value}]{state_ending_token}")
    }
}
