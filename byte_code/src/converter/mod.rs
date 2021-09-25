use crate::alloc::format;
use ellie_core::definite::{items, DefiniteParsed};

pub struct InitialEntryCollector {
    pub key: usize,
    pub value: String,
}

pub struct HeaderEntryCollector {
    pub key: usize,
    pub value: String,
}

pub struct ImportEntryCollector {
    pub key: usize,
    pub value: String,
}

pub struct ScopeEntryCollector {
    pub type_collected: bool,
}

#[derive(Clone, Debug)]
pub struct ConverterOptions {
    pub apply_comments: bool,
    pub lib_name: String,
}

#[derive(Clone, Debug)]
pub struct Scope {
    pub inner_scope: bool,
    pub scope_name: String,
    pub initials: Vec<(usize, String, String)>,
    pub items: Vec<crate::Item>,
    pub headers: Vec<(usize, String)>,
}

#[derive(Clone, Debug)]
pub struct Converter {
    pub scopes: Vec<Scope>,
    pub options: ConverterOptions,
    pub initial_scope: String,
}

impl Converter {
    pub fn new(scope_name: String, options: ConverterOptions, is_inner: bool) -> Self {
        Converter {
            scopes: vec![Scope {
                inner_scope: is_inner,
                scope_name: scope_name.clone(),
                initials: Vec::new(),
                items: Vec::new(),
                headers: Vec::new(),
            }],
            options,
            initial_scope: scope_name,
        }
    }

    pub fn scope_pos(self, id: String) -> Option<usize> {
        self.scopes.into_iter().position(|x| x.scope_name == id)
    }

    pub fn convert(&mut self, parsed: DefiniteParsed) {
        for (index, element) in parsed.items.iter().enumerate() {
            match element {
                items::Collecting::ImportItem(_) => (),
                items::Collecting::Variable(e) => {
                    if let Some(scope_pos) = self.clone().scope_pos(parsed.name.clone()).clone() {
                        let scope = &mut self.scopes[scope_pos];
                        scope.headers.push((index, e.name.clone()));
                        scope.initials.push((
                            index,
                            ellie_parser::syntax::types::Types::default()
                                .from_definite(e.value.clone())
                                .get_type(),
                            "e.collected_value.clone()".to_string(),
                        ));

                        scope.items.push(crate::Item {
                            rtype: if e.public {
                                crate::Commands::PV
                            } else {
                                crate::Commands::RV
                            },
                            name: index,
                            has_initial: false,
                            initial_data: "e.collected_value.clone()".to_string(),
                            ..Default::default()
                        })
                    }
                }
                items::Collecting::Function(_) => todo!(),
                items::Collecting::ForLoop(_) => todo!(),
                items::Collecting::Condition(_) => todo!(),
                items::Collecting::Class(_) => todo!(),
                items::Collecting::Ret(_) => todo!(),
                items::Collecting::Constructor(_) => todo!(),
                items::Collecting::Caller(_) => todo!(),
                items::Collecting::Import(_) => (),
                items::Collecting::FileKey(_) => (),
                items::Collecting::Getter(_) => todo!(),
                items::Collecting::Setter(_) => todo!(),
                items::Collecting::NativeClass => todo!(),
                items::Collecting::ValueCall(_) => todo!(),
                items::Collecting::Enum(_) => todo!(),
                items::Collecting::NativeFunction(_) => todo!(),
                items::Collecting::None => todo!(),
            }
        }
    }

    pub fn to_string(self) -> String {
        let mut raw = String::new();

        if self.options.apply_comments {
            raw += &format!("%S={}\n", self.initial_scope);
            raw += &format!("%L={}\n", self.options.lib_name);
        }

        for scope in self.scopes {
            let mut raw_scope = String::new();

            raw_scope += &format!("{}{}:\n", "@", scope.scope_name).to_owned();
            raw_scope += &"\tT:\n".to_owned();

            for item in scope.items {
                let _get_com_rest = match item.rtype {
                    crate::Commands::RI => todo!(),
                    crate::Commands::PI => todo!(),
                    crate::Commands::RN => todo!(),
                    crate::Commands::PN => todo!(),
                    crate::Commands::RV => todo!(),
                    crate::Commands::PV => todo!(),
                    crate::Commands::RT => todo!(),
                    crate::Commands::PT => todo!(),
                    crate::Commands::RF => todo!(),
                    crate::Commands::PF => todo!(),
                    crate::Commands::RU => todo!(),
                    crate::Commands::PU => todo!(),
                    crate::Commands::RC => todo!(),
                    crate::Commands::PC => todo!(),
                    crate::Commands::RL => todo!(),
                    crate::Commands::PL => todo!(),
                    crate::Commands::RG => todo!(),
                    crate::Commands::PG => todo!(),
                    crate::Commands::RS => todo!(),
                    crate::Commands::PS => todo!(),
                    crate::Commands::RM => todo!(),
                    crate::Commands::PM => todo!(),
                    crate::Commands::CO => todo!(),
                    crate::Commands::AD => todo!(),
                    crate::Commands::SS => todo!(),
                    crate::Commands::MU => todo!(),
                    crate::Commands::EX => todo!(),
                    crate::Commands::DI => todo!(),
                    crate::Commands::MO => todo!(),
                    crate::Commands::IF => todo!(),
                    crate::Commands::EF => todo!(),
                    crate::Commands::EL => todo!(),
                    crate::Commands::FO => todo!(),
                    crate::Commands::RE => todo!(),
                    crate::Commands::FC => todo!(),
                    crate::Commands::PR => todo!(),
                };

                //raw_scope += &format!("\t\t{:?} {} {}\n", item.rtype, item.name, get_com_rest).to_owned();
            }

            raw_scope += &"\tH:\n".to_owned();

            for item in scope.headers {
                raw_scope += &format!("\t\t{} {}\n", item.0, item.1).to_owned();
            }

            raw_scope += &"\tI:\n".to_owned();
            for item in scope.initials {
                raw_scope += &format!("\t\t{} {} {}\n", item.0, item.1, item.2).to_owned();
            }

            raw += &raw_scope;
        }
        raw
    }
}
