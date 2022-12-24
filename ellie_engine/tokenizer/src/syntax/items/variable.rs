use crate::processors::types::{Processors, TypeProcessor};
use crate::syntax::items::definers::{DefinerCollector, DefinerTypes};
use ellie_core::definite::Converter;
use ellie_core::defs;
use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub constant: bool,
    pub public: bool,
    pub has_type: bool,
    pub has_value: bool,
    pub value: Processors,
    pub pos: defs::Cursor,
    pub name_pos: defs::Cursor,
    pub value_pos: defs::Cursor,
    pub type_pos: defs::Cursor,
    pub hash: usize,
    pub rtype: crate::syntax::items::definers::DefinerCollector,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct VariableCollector {
    pub data: Variable,
    pub type_collected: bool,
    pub value_collected: bool,
    pub name_collected: bool,
    pub value_cache: TypeProcessor,
    pub type_cache: crate::syntax::items::definers::DefinerCollector,
    pub complete: bool,
}

impl Converter<VariableCollector, ellie_core::definite::items::variable::Variable>
    for VariableCollector
{
    fn to_definite(self) -> ellie_core::definite::items::variable::Variable {
        ellie_core::definite::items::variable::Variable {
            name: self.data.name,
            constant: self.data.constant,
            public: self.data.public,
            value: self.data.value.to_definite(),
            pos: self.data.pos,
            name_pos: self.data.name_pos,
            value_pos: self.data.value_pos,
            type_pos: self.data.type_pos,
            rtype: self.data.rtype.definer_type.to_definite(),
            file_keys: Vec::new(),
            hash: self.data.hash,
            has_type: self.data.has_type,
            has_value: self.data.has_value,
        }
    }

    fn from_definite(
        self,
        from: ellie_core::definite::items::variable::Variable,
    ) -> VariableCollector {
        VariableCollector {
            data: Variable {
                name: from.name,
                constant: from.constant,
                public: from.public,
                value: Processors::default().from_definite(from.value),
                pos: from.pos,
                name_pos: from.name_pos,
                value_pos: from.value_pos,
                hash: from.hash,
                type_pos: from.type_pos,
                rtype: DefinerCollector {
                    definer_type: DefinerTypes::default().from_definite(from.rtype),
                    complete: true,
                },
                has_type: from.has_type,
                has_value: from.has_value,
            },
            ..Default::default()
        }
    }
}
