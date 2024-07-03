use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::constructor::Constructor;

impl CodeRenderer for Constructor {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);

        let params = {
            let mut params = "(".to_string();
            for (index, param) in self.parameters.iter().enumerate() {
                let mut _param = param.name.to_string();

                if index != self.parameters.len() - 1 {
                    if options.leave_space_after_comma {
                        _param += ", "
                    } else {
                        _param += ","
                    }
                }
                params += &_param;
            }
            params += ")";
            params
        };

        let mut constructor_input = format!("{state_scope_length}co{params}",);

        if !self.inside_code.is_empty() {
            if options.render_brace_next_line {
                constructor_input += &format!(
                    "{line_ending}{state_scope_length}{{{line_ending}",
                    line_ending = options.render_line_ending()
                );
            } else {
                constructor_input += &format!(
                    " {{{line_ending}",
                    line_ending = options.render_line_ending()
                );
            }

            let mut child_state = state.clone();
            child_state.scope_length += 1;
            child_state.ending_token = options.render_line_ending();
            for line in self.inside_code.iter() {
                constructor_input += &line.render(&child_state, options);
            }

            constructor_input += format!(
                "{state_scope_length}}}{line_ending}",
                line_ending = options.render_line_ending()
            )
            .as_str();
        } else {
            constructor_input += &format!(";{}", options.render_line_ending());
        }
        constructor_input
    }
}
