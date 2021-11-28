use crate::{
    processors::types::{Processors, TypeProcessor},
    syntax::types::operator_type::AssignmentOperators,
};
use ellie_core::{definite::Converter, defs};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetterCall {
    pub target: Processors,
    pub value: Processors,
    pub operator: AssignmentOperators,
    pub cache: TypeProcessor,
    pub pos: defs::Cursor,
    pub complete: bool,
}

impl Converter<SetterCall, ellie_core::definite::items::setter_call::SetterCall> for SetterCall {
    fn to_definite(self) -> ellie_core::definite::items::setter_call::SetterCall {
        ellie_core::definite::items::setter_call::SetterCall {
            target: self.target.to_definite(),
            value: self.value.to_definite(),
            operator: match self.operator {
                AssignmentOperators::Assignment => ellie_core::definite::types::assignment_type::AssignmentOperators::Assignment,
                AssignmentOperators::AdditionAssignment => ellie_core::definite::types::assignment_type::AssignmentOperators::AdditionAssignment,
                AssignmentOperators::SubtractionAssignment => ellie_core::definite::types::assignment_type::AssignmentOperators::SubtractionAssignment,
                AssignmentOperators::MultiplicationAssignment => ellie_core::definite::types::assignment_type::AssignmentOperators::MultiplicationAssignment,
                AssignmentOperators::DivisionAssignment => ellie_core::definite::types::assignment_type::AssignmentOperators::DivisionAssignment,
                AssignmentOperators::ModulusAssignment => ellie_core::definite::types::assignment_type::AssignmentOperators::ModulusAssignment,
                AssignmentOperators::ExponentiationAssignment => ellie_core::definite::types::assignment_type::AssignmentOperators::ExponentiationAssignment,
                AssignmentOperators::Null => ellie_core::definite::types::assignment_type::AssignmentOperators::Null,
            },
            pos: self.pos,
        }
    }

    fn from_definite(
        self,
        from: ellie_core::definite::items::setter_call::SetterCall,
    ) -> SetterCall {
        SetterCall {
            target: Processors::default().from_definite(from.target),
            value: Processors::default().from_definite(from.value),
            operator: match from.operator {
                ellie_core::definite::types::assignment_type::AssignmentOperators::Assignment => AssignmentOperators::Assignment,
                ellie_core::definite::types::assignment_type::AssignmentOperators::AdditionAssignment => AssignmentOperators::AdditionAssignment,
                ellie_core::definite::types::assignment_type::AssignmentOperators::SubtractionAssignment => AssignmentOperators::SubtractionAssignment,
                ellie_core::definite::types::assignment_type::AssignmentOperators::MultiplicationAssignment => AssignmentOperators::MultiplicationAssignment,
                ellie_core::definite::types::assignment_type::AssignmentOperators::DivisionAssignment => AssignmentOperators::DivisionAssignment,
                ellie_core::definite::types::assignment_type::AssignmentOperators::ModulusAssignment => AssignmentOperators::ModulusAssignment,
                ellie_core::definite::types::assignment_type::AssignmentOperators::ExponentiationAssignment => AssignmentOperators::ExponentiationAssignment,
                ellie_core::definite::types::assignment_type::AssignmentOperators::Null => AssignmentOperators::Null,
            }, 
            ..Default::default()
        }
    }
}
