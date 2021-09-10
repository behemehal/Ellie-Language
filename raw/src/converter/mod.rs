use crate::alloc::format;
use ellie_parser::parser;

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

    pub fn convert(&mut self, parsed: parser::Parsed) {
        for (index, element) in parsed.items.iter().enumerate() {
            match element {
                parser::Collecting::ImportItem(_) => todo!(),
                parser::Collecting::Variable(e) => {
                    if let Some(scope_pos) = self.clone().scope_pos(parsed.name.clone()).clone() {
                        let mut scope = &mut self.scopes[scope_pos];
                        scope.headers.push((index, e.data.name.clone()));
                        if e.collected_value != "" {
                            scope.initials.push((
                                index,
                                e.data.value.get_type(),
                                e.collected_value.clone(),
                            ));
                        }
                        scope.items.push(crate::Item {
                            rtype: if e.data.public {
                                crate::Commands::PV
                            } else {
                                crate::Commands::RV
                            },
                            name: index,
                            has_initial: e.collected_value != "",
                            initial_data: e.collected_value.clone(),
                            ..Default::default()
                        })
                    }
                }
                parser::Collecting::Function(e) => {
                    if let Some(scope_pos) = self.clone().scope_pos(parsed.name.clone()).clone() {
                        let mut inner_scope =
                            Converter::new(index.to_string(), self.options.clone(), true);

                        inner_scope.convert(parser::Parsed {
                            name: index.to_string(),
                            items: e.data.code.collected.clone(),
                        });

                        self.scopes.extend(inner_scope.scopes);

                        let mut scope = &mut self.scopes[scope_pos];
                        scope.headers.push((index, e.data.name.clone()));

                        scope.items.push(crate::Item {
                            rtype: if e.data.public {
                                crate::Commands::PF
                            } else {
                                crate::Commands::RF
                            },
                            name: index,
                            has_initial: false,
                            initial_data: "".to_owned(),
                            has_inner_scope: true,
                            inner_scope_name: index,
                            ..Default::default()
                        })
                    }
                }
                parser::Collecting::ForLoop(_) => todo!(),
                parser::Collecting::Condition(_) => todo!(),
                parser::Collecting::Class(e) => {
                    if let Some(scope_pos) = self.clone().scope_pos(parsed.name.clone()).clone() {
                        let mut constructor_inner_scope =
                            Converter::new(index.to_string(), self.options.clone(), true);

                        let mut scope = &mut self.scopes[scope_pos];
                        scope.headers.push((index, e.data.name.clone()));
                        scope.items.push(crate::Item {
                            rtype: if e.data.public {
                                crate::Commands::PC
                            } else {
                                crate::Commands::RC
                            },
                            name: index,
                            has_inner_scope: true,
                            inner_scope_name: index,
                            has_generics: true,
                            generics: e.data.generic_definings.clone().into_iter().map(|x| x.name).collect(),
                            ..Default::default()
                        })
                    }
                }
                parser::Collecting::Ret(_) => todo!(),
                parser::Collecting::Constructor(_) => todo!(),
                parser::Collecting::Caller(_) => todo!(),
                parser::Collecting::Import(_) => todo!(),
                parser::Collecting::FileKey(_) => todo!(),
                parser::Collecting::Getter => todo!(),
                parser::Collecting::Setter => todo!(),
                parser::Collecting::NativeClass => todo!(),
                parser::Collecting::NativeFunction(e) => {
                    if let Some(scope_pos) = self.clone().scope_pos(parsed.name.clone()).clone() {
                        let mut scope = &mut self.scopes[scope_pos];
                        scope.headers.push((index, e.name.clone()));
                        scope.items.push(crate::Item {
                            rtype: if e.public {
                                crate::Commands::PU
                            } else {
                                crate::Commands::RU
                            },
                            name: index,
                            ..Default::default()
                        })
                    }
                }
                parser::Collecting::None => todo!(),
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
                let get_com_rest = match item.rtype {
                    crate::Commands::RI => todo!(),
                    crate::Commands::PI => todo!(),
                    crate::Commands::RN => todo!(),
                    crate::Commands::PN => todo!(),
                    crate::Commands::RV => format!(
                        "{} : {}\n",
                        if item.has_type {
                            " ".to_owned() + &item.type_id.to_string()
                        } else {
                            "".to_owned()
                        },
                        item.name
                    ),
                    crate::Commands::PV => format!(
                        "{} : {}\n",
                        if item.has_type {
                            " ".to_owned() + &item.type_id.to_string()
                        } else {
                            "".to_owned()
                        },
                        item.name
                    ),
                    crate::Commands::RT => todo!(),
                    crate::Commands::PT => todo!(),
                    crate::Commands::RD => todo!(),
                    crate::Commands::PD => todo!(),
                    crate::Commands::RF => todo!(),
                    crate::Commands::PF => todo!(),
                    crate::Commands::RU => todo!(),
                    crate::Commands::PU => todo!(),
                    crate::Commands::RC => format!(
                        "<{:?}>",
                        item.generics
                    ),
                    crate::Commands::PC => {
                        todo!()
                    }
                    crate::Commands::RL => todo!(),
                    crate::Commands::PL => todo!(),
                    crate::Commands::IF => todo!(),
                    crate::Commands::EF => todo!(),
                    crate::Commands::EL => todo!(),
                    crate::Commands::FO => todo!(),
                    crate::Commands::RE => todo!(),
                };

                raw_scope +=
                    &format!("\t\t{:?} {} {}\n", item.rtype, item.name, get_com_rest).to_owned();
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
