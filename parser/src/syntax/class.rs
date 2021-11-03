use crate::syntax::{constructor, function, getter, setter, variable};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct GenericDefining {
    pub name: String,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Class {
    pub name: String,
    pub public: bool,
    pub constructor: constructor::Constructor,
    pub generic_definings: Vec<GenericDefining>,
    pub properties: Vec<variable::Variable>,
    pub getters: Vec<getter::Getter>,
    pub setters: Vec<setter::Setter>,
    pub methods: Vec<function::Function>,
    pub name_pos: defs::Cursor,
    pub pos: defs::Cursor,
    pub hash: String,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClassCollector {
    pub generic_definings_collected: bool,
    pub brace_count: usize,
    pub name_collected: bool,
    pub inside_code_string: String,
    pub generic_brace_open: bool,
    pub has_code: bool,
    pub at_comma: bool,
    pub data: Class,
    pub code: Box<crate::parser::RawParser>,
}

impl ClassCollector {
    pub fn to_definite(self) -> definite::items::class::Class {
        definite::items::class::Class {
            name: self.data.name,
            hash: self.data.hash,
            public: self.data.public,
            constructor: self.data.constructor.to_definite(),
            generic_definings: self
                .data
                .generic_definings
                .into_iter()
                .map(|x| definite::items::class::GenericDefining {
                    name: x.name,
                    pos: x.pos,
                })
                .collect(),
            properties: self
                .data
                .properties
                .into_iter()
                .map(|x| x.to_definite())
                .collect(),
            getters: self
                .data
                .getters
                .into_iter()
                .map(|x| x.to_definite())
                .collect(),
            setters: self
                .data
                .setters
                .into_iter()
                .map(|x| x.to_definite())
                .collect(),
            methods: self
                .data
                .methods
                .into_iter()
                .map(|x| x.to_definite())
                .collect(),
            name_pos: self.data.name_pos,
            pos: self.data.pos,
        }
    }

    pub fn from_definite(self, from: definite::items::class::Class) -> Self {
        ClassCollector {
            data: Class {
                name: from.name,
                hash: from.hash,
                public: from.public,
                constructor: constructor::ConstructorCollector::default()
                    .from_definite(from.constructor)
                    .data,
                generic_definings: from
                    .generic_definings
                    .into_iter()
                    .map(|x| GenericDefining {
                        name: x.name,
                        pos: x.pos,
                    })
                    .collect(),
                properties: from
                    .properties
                    .into_iter()
                    .map(|x| variable::VariableCollector::default().from_definite(x).data)
                    .collect(),
                getters: from
                    .getters
                    .into_iter()
                    .map(|x| getter::GetterCollector::default().from_definite(x).data)
                    .collect(),
                setters: from
                    .setters
                    .into_iter()
                    .map(|x| setter::SetterCollector::default().from_definite(x).data)
                    .collect(),
                methods: from
                    .methods
                    .into_iter()
                    .map(|x| function::FunctionCollector::default().from_definite(x).data)
                    .collect(),
                name_pos: from.name_pos,
                pos: from.pos,
            },
            ..Default::default()
        }
    }

    pub fn has_dedup(&self) -> bool {
        let mut existent_names: Vec<String> = Vec::with_capacity(self.data.generic_definings.len());
        let mut duplicate = false;
        for i in &self.data.generic_definings {
            if existent_names.contains(&i.name) {
                duplicate = true;
                break;
            } else {
                existent_names.push(i.name.clone())
            }
        }
        duplicate
    }
}
