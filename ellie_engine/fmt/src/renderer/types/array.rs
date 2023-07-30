use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::types::array_type::ArrayType;

impl CodeRenderer for ArrayType {
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);
        let state_ending_token = &state.ending_token;
        if options.extend_array {
            todo!()
        } else {
            let mut array = format!("{state_scope_length}[");
            for (index, item) in self.collective.iter().enumerate() {
                let value = item.value.render(
                    &State {
                        scope_length: 0,
                        ending_token: String::new(),
                    },
                    options,
                );

                if index != self.collective.len() - 1 {
                    array.push_str(&format!("{}, ", value));
                } else {
                    array.push_str(&value.to_string());
                }
            }
            array.push_str(&format!("]{state_ending_token}"));
            array
        }
    }
}
