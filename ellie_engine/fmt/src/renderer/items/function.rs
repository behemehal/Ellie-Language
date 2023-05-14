use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::function::Function;

impl CodeRenderer for Function {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);
        let is_public = if self.public { "pub " } else { "" };

        let params = {
            let mut params = format!("(");
            for (index, param) in self.parameters.iter().enumerate() {
                let mut _param = format!(
                    "{}{}",
                    param.name,
                    if param.multi_capture { "*" } else { "" }
                );

                if options.space_before_type_colon {
                    _param += " : "
                } else {
                    _param += ": "
                }

                _param += &param
                    .rtype
                    .definer_type
                    .render(&State::empty_state(), options);

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

        let rtype = if self.no_return {
            String::new()
        } else {
            let rtype = self
                .return_type
                .definer_type
                .render(&State::empty_state(), options);
            if options.space_before_type_colon {
                format!(" : {}", rtype)
            } else {
                format!(": {}", rtype)
            }
        };

        let mut fn_input = format!(
            "{state_scope_length}{is_public}fn {fn_name}{params}{rtype}",
            fn_name = &self.name
        );

        if self.defining {
            fn_input += ";";
            return fn_input;
        } else if self.body.len() == 0 {
            fn_input += " {}";
            return fn_input;
        }

        if options.render_brace_next_line {
            fn_input += &format!(
                "{line_ending}{state_scope_length}{{{line_ending}",
                line_ending = options.render_line_ending()
            );
        } else {
            fn_input += &format!(
                " {{{line_ending}",
                line_ending = options.render_line_ending()
            );
        }

        let mut child_state = state.clone();
        child_state.scope_length += 1;
        child_state.ending_token = options.render_line_ending();
        for line in self.body.iter() {
            fn_input += &line.render(&child_state, options);
        }
        fn_input += format!(
            "{state_scope_length}}}{line_ending}",
            line_ending = options.render_line_ending()
        )
        .as_str();
        fn_input
    }
}
