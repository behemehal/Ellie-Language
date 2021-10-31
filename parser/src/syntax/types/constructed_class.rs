use crate::syntax::types;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{borrow::ToOwned, boxed::Box};
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct ConstructedClassParameter {
    pub value: types::Types,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct ConstructedClass {
    pub value: Box<types::Types>,
    pub keyword_pos: defs::Cursor,
    pub value_pos: defs::Cursor,
    pub params: Vec<ConstructedClassParameter>,
}

impl ConstructedClass {
    pub fn class_name(self) -> String {
        match *self.value {
            types::Types::Integer(_) => "int".to_owned(),
            types::Types::Float(_) => "float".to_owned(),
            types::Types::Bool(_) => "bool".to_owned(),
            types::Types::String(_) => "string".to_owned(),
            types::Types::Char(_) => "char".to_owned(),
            types::Types::Collective(_) => "collective".to_owned(),
            types::Types::Reference(_) => todo!("Not implemented"),
            types::Types::BracketReference(_) => todo!("Not implemented"),
            types::Types::Operator(_) => panic!("UNEXPECTED BEHAVIOUR"),
            types::Types::Cloak(_) => "cloak".to_owned(),
            types::Types::Array(_) => "array".to_owned(),
            types::Types::ArrowFunction(_) => "function".to_owned(),
            types::Types::ConstructedClass(e) => e.data.class_name(),
            types::Types::FunctionCall(e) => e.return_type.raw_name(),
            types::Types::Void => "void".to_owned(),
            types::Types::NullResolver(_) => "nullResolver".to_owned(),
            types::Types::Negative(_) => "bool".to_owned(),
            types::Types::VariableType(e) => e.data.value,
            types::Types::Null => "null".to_owned(),
        }
    }
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct ConstructedClassCollector {
    pub data: ConstructedClass,
    pub keyword_collected: bool,
    pub keyword_index: i8,
    pub raw_value: String,
    pub value_collected: bool,
    pub comma: bool,
    pub complete: bool,
}

impl ConstructedClassCollector {
    pub fn to_definite(self) -> definite::types::constructed_class::ConstructedClass {
        definite::types::constructed_class::ConstructedClass {
            value: Box::new(self.data.value.to_definite()),
            keyword_pos: self.data.keyword_pos,
            value_pos: self.data.value_pos,
            params: self
                .data
                .params
                .into_iter()
                .map(
                    |x| definite::types::constructed_class::ConstructedClassParameter {
                        value: x.value.to_definite(),
                        pos: x.pos,
                    },
                )
                .collect(),
        }
    }

    pub fn from_definite(self, from: definite::types::constructed_class::ConstructedClass) -> Self {
        ConstructedClassCollector {
            data: ConstructedClass {
                value: Box::new(types::Types::default().from_definite(*from.value)),
                keyword_pos: from.keyword_pos,
                value_pos: from.value_pos,
                params: from
                    .params
                    .into_iter()
                    .map(|x| ConstructedClassParameter {
                        value: types::Types::default().from_definite(x.value),
                        pos: x.pos,
                    })
                    .collect(),
            },
            complete: true,
            ..Default::default()
        }
    }
}
