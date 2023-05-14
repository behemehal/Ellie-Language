use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::{
    items::setter_call::SetterCall, types::operator_type::AssignmentOperators,
};

impl CodeRenderer for SetterCall {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);
        let line_ending = &options.render_line_ending();
        let target_value = self.target.render(&State::empty_state(), options);
        let mut operator = match self.operator {
            AssignmentOperators::Assignment => "=",
            AssignmentOperators::AdditionAssignment => "+=",
            AssignmentOperators::SubtractionAssignment => "-=",
            AssignmentOperators::MultiplicationAssignment => "*=",
            AssignmentOperators::DivisionAssignment => "/=",
            AssignmentOperators::ModulusAssignment => "%=",
            AssignmentOperators::ExponentiationAssignment => "**=",
            AssignmentOperators::Null => "",
        }
        .to_string();

        if options.space_between_operators {
            operator = format!(" {} ", operator);
        }

        let value = self.value.render(&State::empty_state(), options);

        format!("{state_scope_length}{target_value}{operator}{value};{line_ending}",)
    }
}
