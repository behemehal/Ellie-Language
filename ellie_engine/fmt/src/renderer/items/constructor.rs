use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::constructor::Constructor;

impl CodeRenderer for Constructor {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);

        let params = {
            let mut params = format!("(");
            for (index, param) in self.parameters.iter().enumerate() {
                let mut _param = format!("{}", param.name,);

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

        if self.inside_code.len() > 0 {
            todo!()
        } else {
            constructor_input += &format!(";{}", options.render_line_ending());
        }
        constructor_input
    }
}
