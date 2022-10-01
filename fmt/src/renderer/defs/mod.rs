pub mod array;
pub mod cloak;
pub mod collective;
pub mod function;
pub mod generic;
pub mod nullable;
pub mod parent_generic;
pub mod vector;

use super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::definers::DefinerTypes;

impl CodeRenderer for DefinerTypes {
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        match self {
            DefinerTypes::Cloak(_) => todo!(),
            DefinerTypes::Array(e) => e.render(state, options),
            DefinerTypes::Collective(e) => e.render(state, options),
            DefinerTypes::Vector(e) => e.render(state, options),
            DefinerTypes::Nullable(e) => e.render(state, options),
            DefinerTypes::ParentGeneric(e) => e.render(state, options),
            DefinerTypes::Generic(e) => e.render(state, options),
            DefinerTypes::Function(e) => e.render(state, options),
            DefinerTypes::Dynamic => unreachable!("Dynamic is not a definer type"),
        }
    }
}
