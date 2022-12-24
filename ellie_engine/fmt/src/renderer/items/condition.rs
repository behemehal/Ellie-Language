use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::condition::Condition;
use ellie_tokenizer::syntax::items::condition::ConditionChain;

impl CodeRenderer for Condition {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);

        let render_chain = |chain: &ConditionChain| -> String {
            let mut chain_str = format!("{state_scope_length}");
            match chain.rtype {
                ellie_tokenizer::syntax::items::condition::ConditionType::If => {
                    chain_str += &format!(
                        "if {}",
                        chain
                            .condition
                            .current
                            .render(&State::empty_state(), options)
                    );
                }
                ellie_tokenizer::syntax::items::condition::ConditionType::ElseIf => {
                    chain_str += format!(
                        "else if {}",
                        chain
                            .condition
                            .current
                            .render(&State::empty_state(), options)
                    )
                    .as_str();
                }
                ellie_tokenizer::syntax::items::condition::ConditionType::Else => {
                    chain_str += "else";
                }
            };

            if options.render_brace_next_line {
                chain_str += format!(
                    "{line_ending}{state_scope_length}{{{line_ending}",
                    line_ending = options.render_line_ending()
                )
                .as_str();
            } else {
                chain_str += format!(
                    " {{{line_ending}",
                    line_ending = options.render_line_ending()
                )
                .as_str();
            }

            let mut child_state = state.clone();
            child_state.scope_length += 1;
            child_state.ending_token = options.render_line_ending();
            for line in chain.code.iter() {
                chain_str += &line.render(&child_state, options);
            }
            chain_str
        };

        let mut condition_str = String::new();

        for (idx, chain) in self.chains.iter().enumerate() {
            condition_str += &render_chain(&chain);
            if idx == self.chains.len() - 1 {
                condition_str += &format!(
                    "{state_scope_length}}}{line_ending}",
                    line_ending = options.render_line_ending()
                );
            } else {
                if options.render_brace_next_line {
                    condition_str += &format!(
                        "{state_scope_length}}}{line_ending}",
                        line_ending = options.render_line_ending()
                    );
                } else {
                    condition_str += &format!("{state_scope_length}}}",);
                }
                //condition_str += &format!("{state_scope_length}}} ");
            }
        }

        condition_str
    }
}
