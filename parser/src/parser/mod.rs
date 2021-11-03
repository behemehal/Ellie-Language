use core::panic;

use enum_as_inner::EnumAsInner;
use serde::{Deserialize, Serialize};

pub mod iterator;
pub mod scope;

use crate::alloc::borrow::ToOwned;
use crate::alloc::boxed::Box;
use crate::alloc::string::{String, ToString};
use crate::alloc::vec;
use crate::alloc::vec::Vec;

use crate::syntax::{
    caller, class, condition, constructor, definers, enum_type, file_key, for_loop, function,
    getter, import, import_item, native_function, ret, setter, types, variable,
};
use ellie_core::{com, definite, defs, error};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Parsed {
    pub name: String,
    pub items: Vec<Collecting>,
}

impl Parsed {
    pub fn to_definite(self) -> definite::DefiniteParsed {
        definite::DefiniteParsed {
            name: self.name,
            items: self.items.into_iter().map(|x| x.to_definite()).collect(),
        }
    }

    pub fn from_definite(self, from: definite::DefiniteParsed) -> Self {
        Parsed {
            name: from.name,
            items: from
                .items
                .into_iter()
                .map(|x| Collecting::default().from_definite(x))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParserResponse {
    pub parsed: Parsed,
    pub syntax_errors: Vec<error::Error>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Collecting {
    ImportItem(import_item::ImportItem),
    Variable(variable::VariableCollector),
    Function(function::FunctionCollector),
    ForLoop(for_loop::ForLoopCollector),
    Condition(condition::ConditionCollector),
    Class(class::ClassCollector),
    Ret(ret::Ret),
    Constructor(constructor::ConstructorCollector),
    Caller(caller::Caller),
    Import(import::Import),
    FileKey(file_key::FileKeyCollector),
    Getter(getter::GetterCollector),
    Setter(setter::SetterCollector),
    NativeClass,
    ValueCall(types::Types),
    Enum(enum_type::EnumTypeCollector),
    NativeFunction(native_function::NativeFunction),
    None,
}

impl Collecting {
    pub fn to_definite(self) -> definite::items::Collecting {
        match self {
            Collecting::ImportItem(e) => definite::items::Collecting::ImportItem(e.to_definite()),
            Collecting::Variable(e) => definite::items::Collecting::Variable(e.to_definite()),
            Collecting::Function(e) => definite::items::Collecting::Function(e.to_definite()),
            Collecting::ForLoop(e) => definite::items::Collecting::ForLoop(e.to_definite()),
            Collecting::Condition(e) => definite::items::Collecting::Condition(e.to_definite()),
            Collecting::Class(e) => definite::items::Collecting::Class(e.to_definite()),
            Collecting::Ret(e) => definite::items::Collecting::Ret(e.to_definite()),
            Collecting::Constructor(e) => definite::items::Collecting::Constructor(e.to_definite()),
            Collecting::Caller(e) => definite::items::Collecting::Caller(e.to_definite()),
            Collecting::Import(e) => definite::items::Collecting::Import(e.to_definite()),
            Collecting::FileKey(e) => definite::items::Collecting::FileKey(e.to_definite()),
            Collecting::Getter(e) => definite::items::Collecting::Getter(e.to_definite()),
            Collecting::Setter(e) => definite::items::Collecting::Setter(e.to_definite()),
            Collecting::NativeClass => definite::items::Collecting::NativeClass,
            Collecting::ValueCall(e) => definite::items::Collecting::ValueCall(e.to_definite()),
            Collecting::Enum(e) => definite::items::Collecting::Enum(e.to_definite()),
            Collecting::NativeFunction(e) => {
                definite::items::Collecting::NativeFunction(e.to_definite())
            }
            Collecting::None => definite::items::Collecting::None,
        }
    }

    pub fn from_definite(self, from: definite::items::Collecting) -> Self {
        match from {
            definite::items::Collecting::ImportItem(e) => {
                Collecting::ImportItem(import_item::ImportItem::default().from_definite(e))
            }
            definite::items::Collecting::Variable(e) => {
                Collecting::Variable(variable::VariableCollector::default().from_definite(e))
            }
            definite::items::Collecting::Function(e) => {
                Collecting::Function(function::FunctionCollector::default().from_definite(e))
            }
            definite::items::Collecting::ForLoop(e) => {
                Collecting::ForLoop(for_loop::ForLoopCollector::default().from_definite(e))
            }
            definite::items::Collecting::Condition(e) => {
                Collecting::Condition(condition::ConditionCollector::default().from_definite(e))
            }
            definite::items::Collecting::Class(e) => {
                Collecting::Class(class::ClassCollector::default().from_definite(e))
            }
            definite::items::Collecting::Ret(e) => {
                Collecting::Ret(ret::Ret::default().from_definite(e))
            }
            definite::items::Collecting::Constructor(e) => Collecting::Constructor(
                constructor::ConstructorCollector::default().from_definite(e),
            ),
            definite::items::Collecting::Caller(e) => {
                Collecting::Caller(caller::Caller::default().from_definite(e))
            }
            definite::items::Collecting::Import(e) => {
                Collecting::Import(import::Import::default().from_definite(e))
            }
            definite::items::Collecting::FileKey(e) => {
                Collecting::FileKey(file_key::FileKeyCollector::default().from_definite(e))
            }
            definite::items::Collecting::Getter(e) => {
                Collecting::Getter(getter::GetterCollector::default().from_definite(e))
            }
            definite::items::Collecting::Setter(e) => {
                Collecting::Setter(setter::SetterCollector::default().from_definite(e))
            }
            definite::items::Collecting::Enum(e) => {
                Collecting::Enum(enum_type::EnumTypeCollector::default().from_definite(e))
            }
            definite::items::Collecting::NativeFunction(e) => Collecting::NativeFunction(
                native_function::NativeFunction::default().from_definite(e),
            ),
            definite::items::Collecting::NativeClass => Collecting::NativeClass,
            definite::items::Collecting::ValueCall(e) => {
                Collecting::ValueCall(types::Types::default().from_definite(e))
            }
            definite::items::Collecting::None => todo!(),
        }
    }
}

impl Default for Collecting {
    fn default() -> Self {
        Collecting::None
    }
}

#[derive(EnumAsInner, Debug)]
pub enum NameCheckResponseType {
    Variable(variable::VariableCollector),
    Getter(getter::GetterCollector),
    Setter(setter::SetterCollector),
    Function(function::FunctionCollector),
    NativeFunction(native_function::NativeFunction),
    Class(class::ClassCollector),
    None,
}

#[derive(PartialEq, Debug)]
pub enum DeepCallResponse {
    TypeResponse(types::Types),
    ElementResponse(Collecting),
    NoElement,
}

#[derive(Debug)]
pub struct NameCheckResponse {
    pub found: bool,
    pub found_type: NameCheckResponseType,
}

//This is a clone of parser that implements serialize and deserialize
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct RawParser {
    pub scope: Box<scope::Scope>,
    pub code: String,
    pub options: defs::ParserOptions,
    pub collected: Vec<Collecting>,
    pub generic_variables: Vec<class::GenericDefining>,
    pub pos: defs::CursorPosition,
    pub on_comment: bool,
    pub on_line_comment: bool,
    pub ignore_line: bool,
    pub current: Collecting,
    pub keyword_pos: defs::Cursor,
    pub keyword_catch: String,
    pub keyword_cache: variable::VariableCollector,
    pub keyword_errors: Vec<error::Error>,
}

impl RawParser {
    pub fn to_parser(
        self,
        _resolver: fn(defs::ParserOptions, String) -> ResolvedImport,
    ) -> Parser<
        impl FnMut(com::Message) + Clone + Copy + Sized,
        impl FnMut(ellie_core::defs::ParserOptions, String, bool) -> ResolvedImport + Clone + Copy + Sized,
    > {
        Parser {
            scope: self.scope,
            resolver: |_, _, _| ResolvedImport::default(),
            emit_message: |_| {},
            code: self.code,
            options: self.options,
            collected: self.collected,
            generic_variables: self.generic_variables,
            pos: self.pos,
            on_comment: self.on_comment,
            on_line_comment: self.on_line_comment,
            ignore_line: self.ignore_line,
            current: self.current,
            keyword_pos: self.keyword_pos,
            keyword_catch: self.keyword_catch,
            keyword_cache: self.keyword_cache,
            keyword_errors: self.keyword_errors,
        }
    }

    pub fn to_no_resolver_parser(
        self,
    ) -> Parser<
        impl FnMut(com::Message) + Clone + Copy + Sized,
        impl FnMut(ellie_core::defs::ParserOptions, String, bool) -> ResolvedImport + Clone + Copy + Sized,
    > {
        Parser {
            scope: self.scope,
            resolver: |_, _, _| ResolvedImport::default(),
            emit_message: |_| {},
            code: self.code,
            options: self.options,
            collected: self.collected,
            generic_variables: self.generic_variables,
            pos: self.pos,
            on_comment: self.on_comment,
            on_line_comment: self.on_line_comment,
            ignore_line: self.ignore_line,
            current: self.current,
            keyword_pos: self.keyword_pos,
            keyword_catch: self.keyword_catch,
            keyword_cache: self.keyword_cache,
            keyword_errors: self.keyword_errors,
        }
    }
}

#[derive(Clone)]
pub struct Parser<F, E> {
    pub scope: Box<scope::Scope>,
    pub resolver: E,
    pub emit_message: F,
    pub code: String,
    pub options: defs::ParserOptions,
    pub collected: Vec<Collecting>,
    pub generic_variables: Vec<class::GenericDefining>,
    pub pos: defs::CursorPosition,
    pub on_comment: bool,
    pub on_line_comment: bool,
    pub ignore_line: bool,
    pub current: Collecting,
    pub keyword_pos: defs::Cursor,
    pub keyword_catch: String,
    pub keyword_cache: variable::VariableCollector,
    pub keyword_errors: Vec<error::Error>,
}

#[derive(Debug, Clone)]
pub enum ResolvedFileContent {
    PreBuilt(Vec<ellie_core::definite::items::Collecting>),
    Raw(String),
}

impl Default for ResolvedFileContent {
    fn default() -> Self {
        ResolvedFileContent::Raw("".to_owned())
    }
}

#[derive(Default, Debug, Clone)]
pub struct ResolvedImport {
    pub found: bool,
    pub resolve_error: String,
    pub resolved_path: String,
    pub resolution_id: u64,
    pub id: u64,
    pub file_content: ResolvedFileContent,
}

impl<F, E> Parser<F, E>
where
    F: FnMut(com::Message) + Clone + Copy + Sized,
    E: FnMut(ellie_core::defs::ParserOptions, String, bool) -> ResolvedImport
        + Clone
        + Copy
        + Sized,
{
    pub fn new(
        code: String,
        resolve_import: E,
        com: F,
        options: defs::ParserOptions,
    ) -> Parser<F, E> {
        Parser {
            scope: Box::new(scope::Scope::default()),
            resolver: resolve_import,
            emit_message: com,
            code,
            options,
            collected: Vec::new(),
            generic_variables: Vec::new(),
            pos: defs::CursorPosition(0, 0),
            keyword_pos: defs::Cursor::default(),
            ignore_line: false,
            on_comment: false,
            on_line_comment: false,
            current: Collecting::None,
            keyword_catch: String::new(),
            keyword_cache: variable::VariableCollector::default(),
            keyword_errors: Vec::new(),
        }
    }

    pub fn read_module(self, code: String, path: String) -> ParserResponse {
        let mut new_options = self.options.clone();
        new_options.path = path;
        new_options.parser_type = ellie_core::defs::ParserType::RawParser;
        Parser::new(code, self.resolver, self.emit_message, new_options).map()
    }

    pub fn read_native_header(self, code: String, path: String) -> ParserResponse {
        let mut new_options = self.options.clone();
        new_options.path = path;
        new_options.parser_type = ellie_core::defs::ParserType::HeaderParser;
        Parser::new(code, self.resolver, self.emit_message, new_options).map()
    }

    pub fn to_raw(self) -> RawParser {
        RawParser {
            scope: self.scope,
            code: self.code,
            options: self.options,
            collected: self.collected,
            generic_variables: self.generic_variables,
            pos: self.pos,
            on_comment: self.on_comment,
            on_line_comment: self.on_line_comment,
            ignore_line: self.ignore_line,
            current: self.current,
            keyword_pos: self.keyword_pos,
            keyword_catch: self.keyword_catch,
            keyword_cache: self.keyword_cache,
            keyword_errors: self.keyword_errors,
        }
    }

    pub fn map(mut self) -> ParserResponse {
        if self.options.import_std {
            let build_std: ellie_core::definite::DefiniteParsed =
                serde_json::from_str(ellie_core::builded_libraries::ELLIE_STANDARD_LIBRARY)
                    .unwrap();
            self.collected = Parsed::default().from_definite(build_std).items;
        }

        let mut errors: Vec<error::Error> = Vec::new();
        let code = self.code.clone();
        let mut content = code.split("").collect::<Vec<_>>();
        content.remove(0);
        content.remove(content.len() - 1);
        for i in 0..content.len() {
            let char = content[i].chars().nth(0).unwrap_or('\0');
            let letter_char = content[i];
            let last_char = if i == 0 { "" } else { content[i - 1] };
            let next_char = if i + 1 > content.len() - 1 {
                ""
            } else {
                content[i + 1]
            };

            if self.pos.1 == 1 {
                (self.emit_message)(ellie_core::com::Message {
                    id: ellie_core::utils::generate_hash(),
                    message_type: ellie_core::com::MessageType::ParserLineExec,
                    from: self.options.path.clone(),
                    from_chain: None,
                    message_data: alloc::format!("{:?}", self.pos.clone()),
                });
            }

            if char != '\n'
                && char != '\r'
                && char != '\t'
                && (letter_char != "/" || next_char != "/")
                && !self.on_line_comment
            {
                iterator::iter(&mut self, &mut errors, letter_char, next_char, last_char);
                self.pos.1 += 1;
            } else if letter_char == "/"
                && next_char == "/"
                && !self.on_comment
                && !self.on_line_comment
            {
                self.on_line_comment = true;
                self.pos.1 += 1;
            } else if last_char == "\r" || letter_char == "\n" {
                self.pos.0 += 1;
                self.on_line_comment = false;
                self.pos.1 = 0;
            }
        }

        errors.extend(self.keyword_errors.clone());
        if self.current != Collecting::None || !self.keyword_catch.trim().is_empty() {
            errors.push(error::Error {
                path: self.options.path.clone(),
                scope: "definer_processor".to_owned(),
                debug_message: "bbf28ccedc74d19be808fad8f20ea341".to_owned(),
                title: error::errorList::error_s26.title.clone(),
                code: error::errorList::error_s26.code,
                message: error::errorList::error_s26.message.clone(),
                builded_message: error::BuildedError::build_from_string(
                    error::errorList::error_s26.message.clone(),
                ),
                pos: self.keyword_pos,
            });
        }
        (self.emit_message)(ellie_core::com::Message {
            id: ellie_core::utils::generate_hash(),
            message_type: ellie_core::com::MessageType::ParseComplete,
            from: self.options.path.clone(),
            from_chain: None,
            message_data: alloc::format!(""),
        });
        ParserResponse {
            parsed: Parsed {
                name: self.scope.scope_name,
                items: self.collected.clone(),
            },
            syntax_errors: errors,
        }
    }

    pub fn type_is_iterable(&self, target_type: types::Types) -> bool {
        match target_type {
            types::Types::Integer(_) => true,
            types::Types::String(_) => true,
            types::Types::Array(_) => true,
            types::Types::Collective(_) => todo!(),
            types::Types::Reference(_) => todo!(),
            types::Types::FunctionCall(_) => todo!(),
            types::Types::NullResolver(_) => todo!(),
            types::Types::VariableType(_) => todo!(),
            _ => false,
        }
    }

    pub fn is_iterable(&self, target: DeepCallResponse) -> Option<i8> {
        match target {
            DeepCallResponse::TypeResponse(e) => match e {
                types::Types::Cloak(cloak) => {
                    if cloak.data.collective.is_empty() || cloak.data.collective.len() != 2 {
                        Some(1)
                    } else {
                        let first_entry = cloak.data.collective[0].clone();
                        let second_entry = cloak.data.collective[1].clone();

                        if let types::Types::VariableType(_) = *first_entry.value {
                            if self.type_is_iterable(*second_entry.value) {
                                None
                            } else {
                                Some(0)
                            }
                        } else {
                            Some(1)
                        }
                    }
                }
                _ => Some(1),
            },
            _ => Some(1),
        }
    }

    pub fn resolve_deep_call(&self, target: types::Types) -> DeepCallResponse {
        match target.clone() {
            types::Types::Integer(_) => DeepCallResponse::TypeResponse(target),
            types::Types::Float(_) => DeepCallResponse::TypeResponse(target),
            types::Types::Bool(_) => DeepCallResponse::TypeResponse(target),
            types::Types::String(_) => DeepCallResponse::TypeResponse(target),
            types::Types::Char(_) => DeepCallResponse::TypeResponse(target),
            types::Types::Null => DeepCallResponse::TypeResponse(target),
            types::Types::Void => DeepCallResponse::TypeResponse(target),
            types::Types::Collective(_) => DeepCallResponse::TypeResponse(target),
            types::Types::Array(_) => DeepCallResponse::TypeResponse(target),
            types::Types::Cloak(_) => DeepCallResponse::TypeResponse(target),
            types::Types::Reference(_) => todo!(),
            types::Types::BracketReference(_) => todo!(),
            types::Types::Operator(_) => todo!(),
            types::Types::ArrowFunction(_) => todo!(),
            types::Types::ConstructedClass(e) => {
                let name = match *e.data.value {
                    types::Types::Integer(_) => "int".to_owned(),
                    types::Types::Float(_) => "float".to_owned(),
                    types::Types::Bool(_) => "bool".to_owned(),
                    types::Types::String(_) => "string".to_owned(),
                    types::Types::Char(_) => "char".to_owned(),
                    types::Types::Collective(_) => "collective".to_owned(),
                    types::Types::Reference(_) => todo!(),
                    types::Types::BracketReference(_) => todo!(),
                    types::Types::Operator(_) => todo!(),
                    types::Types::Cloak(_) => todo!(),
                    types::Types::Array(_) => todo!(),
                    types::Types::ArrowFunction(_) => todo!(),
                    types::Types::ConstructedClass(_) => todo!(),
                    types::Types::FunctionCall(_) => todo!(),
                    types::Types::Void => "void".to_owned(),
                    types::Types::NullResolver(_) => todo!(),
                    types::Types::Negative(_) => "bool".to_owned(),
                    types::Types::VariableType(e) => e.data.value,
                    types::Types::Null => "null".to_owned(),
                };
                let vr_found = self.check_keyword(name, false, false);
                if vr_found.found {
                    if let NameCheckResponseType::Variable(v_data) = vr_found.found_type {
                        self.resolve_deep_call(v_data.data.value)
                    } else if let NameCheckResponseType::Function(f_data) = vr_found.found_type {
                        DeepCallResponse::ElementResponse(Collecting::Function(f_data))
                    } else if let NameCheckResponseType::NativeFunction(f_data) =
                        vr_found.found_type
                    {
                        DeepCallResponse::ElementResponse(Collecting::NativeFunction(f_data))
                    } else if let NameCheckResponseType::Class(c_data) = vr_found.found_type {
                        DeepCallResponse::ElementResponse(Collecting::Class(c_data))
                    } else {
                        DeepCallResponse::NoElement
                    }
                } else {
                    DeepCallResponse::NoElement
                }
            }
            types::Types::FunctionCall(_) => todo!(),
            types::Types::NullResolver(_) => todo!(),
            types::Types::Negative(_) => todo!(),
            types::Types::VariableType(e) => {
                let vr_found = self.check_keyword(e.data.value, false, false);

                if vr_found.found {
                    if let NameCheckResponseType::Variable(v_data) = vr_found.found_type {
                        self.resolve_deep_call(v_data.data.value)
                    } else if let NameCheckResponseType::Function(f_data) = vr_found.found_type {
                        DeepCallResponse::ElementResponse(Collecting::Function(f_data))
                    } else if let NameCheckResponseType::NativeFunction(f_data) =
                        vr_found.found_type
                    {
                        DeepCallResponse::ElementResponse(Collecting::NativeFunction(f_data))
                    } else if let NameCheckResponseType::Class(c_data) = vr_found.found_type {
                        DeepCallResponse::ElementResponse(Collecting::Class(c_data))
                    } else {
                        DeepCallResponse::NoElement
                    }
                } else {
                    DeepCallResponse::NoElement
                }
            } // self.resolve_variable(target)
        }
    }

    pub fn resolve_variable(
        &self,
        target: types::Types,
        contain_private: bool,
    ) -> Result<definers::DefinerCollecting, Vec<ellie_core::error::Error>> {
        match target.clone() {
            types::Types::ConstructedClass(constructed_class) => {
                if let Ok(resolved) = self.resolve_new_call(constructed_class.clone()) {
                    if let Collecting::Class(class_collector) = resolved {
                        Ok(definers::DefinerCollecting::Generic(
                            definers::GenericType {
                                rtype: class_collector.data.name,
                                hash: constructed_class.data.class_hash(),
                            },
                        ))
                    } else {
                        panic!("Unexpected parser behaviour");
                    }
                } else {
                    Err(vec![error::Error {
                        path: self.options.path.clone(),
                        scope: self.scope.scope_name.clone(),
                        debug_message: "edfe21ce811657666a2cd92a370b29c7".to_owned(),
                        title: error::errorList::error_s38.title.clone(),
                        code: error::errorList::error_s38.code,
                        message: error::errorList::error_s38.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s38.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: constructed_class.data.clone().class_name(),
                            }],
                        ),
                        pos: constructed_class.data.value_pos,
                    }])
                }
            }
            types::Types::FunctionCall(e) => {
                let fn_found = self.check_keyword(e.data.name.clone(), contain_private, false);

                if fn_found.found.clone() {
                    if let NameCheckResponseType::Function(_) = fn_found.found_type {
                        Ok(target.to_definer())
                    } else if let NameCheckResponseType::NativeFunction(_) = fn_found.found_type {
                        Ok(target.to_definer())
                    } else if let NameCheckResponseType::Variable(v_data) = fn_found.found_type {
                        if let definers::DefinerCollecting::Function(_) = v_data.data.rtype {
                            Ok(v_data.data.rtype)
                        } else {
                            Err(Vec::new())
                        }
                    } else {
                        Err(Vec::new())
                    }
                } else {
                    Err(vec![error::Error {
                        path: self.options.path.clone(),
                        scope: self.scope.scope_name.clone(),
                        debug_message: "0e318bdd9177ad3435e8c8119fcad4c8".to_owned(),
                        title: error::errorList::error_s38.title.clone(),
                        code: error::errorList::error_s38.code,
                        message: error::errorList::error_s38.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s38.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: e.data.name,
                            }],
                        ),
                        pos: e.data.name_pos,
                    }])
                }
            }
            types::Types::VariableType(e) => {
                let vr_found = self.check_keyword(e.data.value.clone(), contain_private, false);

                if vr_found.found {
                    match vr_found.found_type {
                        NameCheckResponseType::Variable(v_data) => Ok(v_data.data.rtype),
                        NameCheckResponseType::Getter(g_data) => Ok(g_data.data.rtype),
                        NameCheckResponseType::Setter(s_data) => Ok(s_data.data.rtype),
                        NameCheckResponseType::Function(f_data) => Ok(
                            definers::DefinerCollecting::Function(definers::FunctionType {
                                returning: Box::new(f_data.data.return_type),
                                params: f_data
                                    .data
                                    .parameters
                                    .into_iter()
                                    .map(|x| x.rtype)
                                    .collect::<Vec<_>>(),
                                ..Default::default()
                            }),
                        ),
                        NameCheckResponseType::NativeFunction(f_data) => Ok(
                            definers::DefinerCollecting::Function(definers::FunctionType {
                                returning: Box::new(f_data.return_type),
                                params: f_data
                                    .parameters
                                    .into_iter()
                                    .map(|x| x.rtype)
                                    .collect::<Vec<_>>(),
                                ..Default::default()
                            }),
                        ),
                        NameCheckResponseType::Class(c_data) => Ok(
                            definers::DefinerCollecting::Generic(definers::GenericType {
                                rtype: c_data.data.name,
                                hash: c_data.data.hash,
                            }),
                        ),
                        NameCheckResponseType::None => panic!("Unexpected parser behaviour"),
                    }
                } else {
                    Err(vec![error::Error {
                        path: self.options.path.clone(),
                        scope: self.scope.scope_name.clone(),
                        debug_message: "76947034039960f1b99adae5ac4d1c0b".to_owned(),
                        title: error::errorList::error_s38.title.clone(),
                        code: error::errorList::error_s38.code,
                        message: error::errorList::error_s38.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s38.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: e.data.value,
                            }],
                        ),
                        pos: e.data.pos,
                    }])
                }
            }
            e => Ok(e.to_definer()),
        }
    }

    pub fn resolve_reference_tree(
        self,
        _reference: DeepCallResponse,
        _chain: Vec<types::reference_type::Chain>,
    ) {
    }

    pub fn resolve_reference_call(
        self,
        reference_data_collector: types::reference_type::ReferenceTypeCollector,
    ) -> Result<definers::DefinerCollecting, Vec<ellie_core::error::Error>> {
        let chain_len = reference_data_collector.data.chain.len() - 1;
        let chain = &reference_data_collector.data.chain[chain_len];
        let mut found = definers::DefinerCollecting::Dynamic;
        let mut errors = Vec::new();

        let generic = match reference_data_collector.last_entry.clone() {
            definers::DefinerCollecting::Array(_) => "array".to_owned(),
            definers::DefinerCollecting::Cloak(_) => "cloak".to_owned(),
            definers::DefinerCollecting::Future(_) => "future".to_owned(),
            definers::DefinerCollecting::Nullable(_) => "nullable".to_owned(),
            definers::DefinerCollecting::GrowableArray(_) => "growableArray".to_owned(),
            definers::DefinerCollecting::Generic(e) => e.rtype,
            definers::DefinerCollecting::Function(_) => "function".to_owned(),
            definers::DefinerCollecting::Collective(_) => "collective".to_owned(),
            definers::DefinerCollecting::Dynamic => todo!(),
            definers::DefinerCollecting::Error(i) => alloc::format!("unexpectedBehavior({})", i),
        };

        fn resolve_reference_chain(
            target: Collecting,
            chain: String,
        ) -> Result<definers::DefinerCollecting, (String, i8)> {
            /*
                Error codes:
                0 => cannot used as referencer,
                1 => not exists as parameter
            */

            match target {
                Collecting::Variable(_) => todo!(),
                Collecting::Function(_) => todo!(),
                Collecting::Class(targeted_item) => {
                    let properties = targeted_item
                        .data
                        .properties
                        .into_iter()
                        .map(|x| {
                            (
                                x.name,
                                if x.dynamic {
                                    x.value.to_definer()
                                } else {
                                    x.rtype
                                },
                            )
                        })
                        .collect::<Vec<_>>();
                    let getters = targeted_item
                        .data
                        .getters
                        .into_iter()
                        .map(|x| (x.name, x.rtype))
                        .collect::<Vec<_>>();
                    let setters = targeted_item
                        .data
                        .setters
                        .into_iter()
                        .map(|x| (x.name, x.rtype))
                        .collect::<Vec<_>>();
                    let methods = targeted_item
                        .data
                        .methods
                        .into_iter()
                        .map(|x| {
                            (
                                x.name,
                                types::Types::ArrowFunction(
                                    types::arrow_function::ArrowFunctionCollector {
                                        data: types::arrow_function::ArrowFunction {
                                            parameters: x.parameters,
                                            return_type: x.return_type,
                                            inside_code: x.inside_code,
                                            return_pos: x.return_pos,
                                        },
                                        ..Default::default()
                                    },
                                )
                                .to_definer(),
                            )
                        })
                        .collect::<Vec<_>>();
                    let mut all_properties = vec![];
                    all_properties.extend(properties);
                    all_properties.extend(getters);
                    all_properties.extend(setters);
                    all_properties.extend(methods);

                    let found_property = all_properties.into_iter().find(|q| q.0 == chain);
                    match found_property {
                        Some(q) => Ok(q.1),
                        None => Err((chain, 1)),
                    }
                }
                Collecting::Caller(_) => todo!(),
                Collecting::Getter(_) => todo!(),
                Collecting::Setter(_) => todo!(),
                Collecting::NativeClass => todo!(),
                Collecting::Enum(_) => todo!(),
                Collecting::NativeFunction(_) => todo!(),
                Collecting::None => todo!(),
                _ => panic!("Unexpected parser behaviour: {:?}", target),
            }
        }

        let found_generic = self.resolve_reference_proto(generic.clone(), false);
        if found_generic.found {
            match found_generic.found_type {
                NameCheckResponseType::Class(target) => {
                    let resolved_reference_chain = resolve_reference_chain(
                        Collecting::Class(target.clone()), //Remove clone
                        chain.value.clone(),
                    );
                    match resolved_reference_chain {
                        Ok(e) => {
                            found = e;
                        }
                        Err(e) => {
                            found = definers::DefinerCollecting::Generic(definers::GenericType {
                                rtype: "null".to_owned(),
                                hash: "ellie_null_hash".to_owned(),
                            });
                            if e.1 == 0 {
                                errors.push(error::Error {
                                    path: self.options.path.clone(),
                                    scope: self.scope.scope_name.clone(),
                                    debug_message: "baf37e59278fcd4591c85721b2c63680".to_owned(),
                                    title: error::errorList::error_s37.title.clone(),
                                    code: error::errorList::error_s37.code,
                                    message: error::errorList::error_s37.message.clone(),
                                    builded_message: error::Error::build(
                                        error::errorList::error_s37.message.clone(),
                                        vec![error::ErrorBuildField {
                                            key: "token".to_owned(),
                                            value: e.0,
                                        }],
                                    ),
                                    pos: chain.pos,
                                });
                            } else {
                                errors.push(error::Error {
                                    path: self.options.path.clone(),
                                    scope: self.scope.scope_name.clone(),
                                    debug_message: "03335d8d511b815dfd1f39bc3775d069".to_owned(),
                                    title: error::errorList::error_s42.title.clone(),
                                    code: error::errorList::error_s42.code,
                                    message: error::errorList::error_s42.message.clone(),
                                    builded_message: error::Error::build(
                                        error::errorList::error_s42.message.clone(),
                                        vec![
                                            error::ErrorBuildField {
                                                key: "token".to_owned(),
                                                value: e.0,
                                            },
                                            error::ErrorBuildField {
                                                key: "token1".to_owned(),
                                                value: target.data.name,
                                            },
                                        ],
                                    ),
                                    pos: chain.pos,
                                });
                            }
                        }
                    }
                }
                _ => panic!("Unexpected behaviour"),
            }
        } else {
            errors.push(error::Error {
                path: self.options.path.clone(),
                scope: "function_call_processor".to_owned(),
                debug_message: "2d8594b7c8fc5ddcd25ed97360d9e4d5".to_owned(),
                title: error::errorList::error_s6.title.clone(),
                code: error::errorList::error_s6.code,
                message: error::errorList::error_s6.message.clone(),
                builded_message: error::Error::build(
                    error::errorList::error_s6.message.clone(),
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: (*reference_data_collector.data.reference.get_type()).to_string(),
                    }],
                ),
                pos: reference_data_collector.data.reference_pos,
            });
        }

        if errors.is_empty() {
            Ok(found)
        } else {
            Err(errors)
        }
    }

    pub fn look_up_for_item(
        self,
        item: Collecting,
        target: String,
        ignore_private: bool,
    ) -> Option<Collecting> {
        let mut found_item: Option<Collecting> = None;

        match item.clone() {
            Collecting::Variable(c) => {
                if target == c.data.name && (c.data.public || ignore_private) {
                    found_item = Some(item);
                }
            }
            Collecting::Function(c) => {
                if target == c.data.name && (c.data.public || ignore_private) {
                    found_item = Some(item);
                }
            }
            Collecting::Class(c) => {
                if target == c.data.name && (c.data.public || ignore_private) {
                    found_item = Some(item);
                }
            }
            Collecting::Constructor(c) => {
                if target == c.data.name {
                    found_item = Some(item);
                }
            }
            Collecting::ImportItem(c) => {
                if c.public || ignore_private {
                    found_item = self.look_up_for_item(*c.item, target, false);
                }
            }
            _ => found_item = None,
        }

        found_item
    }

    pub fn resolve_new_call(
        &self,
        caller_data: types::constructed_class::ConstructedClassCollector,
    ) -> Result<Collecting, Vec<ellie_core::error::Error>> {
        let mut found = false;
        let mut found_item = Collecting::None;
        let mut errors = Vec::new();

        match *caller_data.data.value {
            types::Types::VariableType(e) => {
                for item in self.collected.clone() {
                    if let Some(looked_up) =
                        self.clone()
                            .look_up_for_item(item, e.data.value.clone(), true)
                    {
                        match looked_up.clone() {
                            Collecting::Variable(c) => {
                                found = true;
                                let resolved_deep_call = self.resolve_deep_call(c.data.value);

                                match resolved_deep_call {
                                    DeepCallResponse::ElementResponse(response) => {
                                        match response.clone() {
                                            Collecting::Class(_e) => {
                                                found = true;
                                                found_item = response;
                                            }
                                            _ => {
                                                errors.push(error::Error {
                                                    path: self.options.path.clone(),
                                                    scope: "function_call_processor".to_owned(),
                                                    debug_message:
                                                        "cd5e55a2e9b088bbd6f453d7593d6d94"
                                                            .to_owned(),
                                                    title: error::errorList::error_s31
                                                        .title
                                                        .clone(),
                                                    code: error::errorList::error_s31.code,
                                                    message: error::errorList::error_s31
                                                        .message
                                                        .clone(),
                                                    builded_message: error::Error::build(
                                                        error::errorList::error_s31.message.clone(),
                                                        vec![error::ErrorBuildField {
                                                            key: "token".to_owned(),
                                                            value: caller_data.raw_value.clone(),
                                                        }],
                                                    ),
                                                    pos: caller_data.data.value_pos,
                                                });
                                            }
                                        }
                                    }
                                    _ => {
                                        errors.push(error::Error {
                                            path: self.options.path.clone(),
                                            scope: "function_call_processor".to_owned(),
                                            debug_message: "a204f5e330b6e4e18603de9fe311de00"
                                                .to_owned(),
                                            title: error::errorList::error_s31.title.clone(),
                                            code: error::errorList::error_s31.code,
                                            message: error::errorList::error_s31.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s31.message.clone(),
                                                vec![error::ErrorBuildField {
                                                    key: "token".to_owned(),
                                                    value: caller_data.raw_value.clone(),
                                                }],
                                            ),
                                            pos: caller_data.data.value_pos,
                                        });
                                    }
                                }
                            }
                            Collecting::Class(_) => {
                                found = true;
                                found_item = looked_up.clone();
                            }
                            _ => {
                                panic!("Failed to resolve this type")
                            }
                        }
                    }
                }
            }
            _ => {
                found = true;
                errors.push(error::Error {
                    path: self.options.path.clone(),
                    scope: "function_call_processor".to_owned(),
                    debug_message: "371f18ac376171e1faacf30b36f19d92".to_owned(),
                    title: error::errorList::error_s31.title.clone(),
                    code: error::errorList::error_s31.code,
                    message: error::errorList::error_s31.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s31.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: caller_data.raw_value.clone(),
                        }],
                    ),
                    pos: caller_data.data.value_pos,
                });
            }
        }

        if !found {
            errors.push(error::Error {
                path: self.options.path.clone(),
                scope: self.scope.scope_name.clone(),
                debug_message: "5462f7ee57ccafbfbf98b2cd70809a82".to_owned(),
                title: error::errorList::error_s6.title.clone(),
                code: error::errorList::error_s6.code,
                message: error::errorList::error_s6.message.clone(),
                builded_message: error::Error::build(
                    error::errorList::error_s6.message.clone(),
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: caller_data.raw_value.clone(),
                    }],
                ),
                pos: caller_data.data.value_pos,
            });
        }

        if errors.is_empty() {
            Ok(found_item)
        } else {
            Err(errors)
        }
    }

    pub fn deep_function_call_resolver(
        &self,
        caller_data: types::function_call::FunctionCallCollector,
        item: Collecting,
        errors: &mut Vec<ellie_core::error::Error>,
        layer: usize,
    ) -> Option<definers::DefinerCollecting> {
        let mut found = false;
        let mut found_type = definers::DefinerCollecting::Dynamic;
        match item {
            Collecting::Variable(e) => {
                if e.data.name == caller_data.data.name && (e.data.public || layer == 0) {
                    if let definers::DefinerCollecting::Function(fn_type) = e.data.rtype {
                        found_type = *fn_type.returning;
                        found = true;
                        if caller_data.data.params.len() != fn_type.params.len() {
                            errors.push(error::Error {
                                path: self.options.path.clone(),
                                scope: self.scope.scope_name.clone(),
                                debug_message: "5d609a1609b1eb9403d080c5e6713e26".to_owned(),
                                title: error::errorList::error_s7.title.clone(),
                                code: error::errorList::error_s7.code,
                                message: error::errorList::error_s7.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s7.message.clone(),
                                    vec![
                                        error::ErrorBuildField {
                                            key: "name".to_owned(),
                                            value: "Function".to_owned(),
                                        },
                                        error::ErrorBuildField {
                                            key: "token".to_owned(),
                                            value: fn_type.params.len().to_string(),
                                        },
                                        error::ErrorBuildField {
                                            key: "token2".to_owned(),
                                            value: caller_data.data.params.len().to_string(),
                                        },
                                    ],
                                ),
                                pos: caller_data.data.name_pos,
                            });
                        } else {
                            for (index, caller_param) in
                                caller_data.data.params.into_iter().enumerate()
                            {
                                match fn_type.params[index].clone() {
                                    definers::DefinerCollecting::Array(_) => {
                                        panic!("Definer Resolving on 'Array' is not supported");
                                    }
                                    definers::DefinerCollecting::Future(_) => {
                                        panic!("Definer Resolving on 'Future' is not supported");
                                    }
                                    definers::DefinerCollecting::GrowableArray(_) => {
                                        panic!(
                                            "Definer Resolving on 'GrowableArray' is not supported"
                                        );
                                    }
                                    definers::DefinerCollecting::Nullable(_) => {
                                        panic!("Definer Resolving on 'Nullable' is not supported");
                                    }
                                    definers::DefinerCollecting::Generic(e) => {
                                        let resolved_type_option =
                                            self.resolve_variable(caller_param.value, true);
                                        if let Ok(resolved_type) = resolved_type_option {
                                            if !resolved_type.is_generic()
                                                || resolved_type.is_generic()
                                                    && resolved_type.as_generic().unwrap().rtype
                                                        != e.rtype
                                            {
                                                errors.push(error::Error {
                                                    path: self.options.path.clone(),
                                                    scope: self.scope.scope_name.clone(),
                                                    debug_message:
                                                        "d4824ea474c0c2675d16029f0708f38f"
                                                            .to_owned(),
                                                    title: error::errorList::error_s3.title.clone(),
                                                    code: error::errorList::error_s3.code,
                                                    message: error::errorList::error_s3
                                                        .message
                                                        .clone(),
                                                    builded_message: error::Error::build(
                                                        error::errorList::error_s3.message.clone(),
                                                        vec![
                                                            error::ErrorBuildField {
                                                                key: "token1".to_owned(),
                                                                value: e.rtype,
                                                            },
                                                            error::ErrorBuildField {
                                                                key: "token2".to_owned(),
                                                                value: resolved_type
                                                                    .raw_name_with_extensions(),
                                                            },
                                                        ],
                                                    ),
                                                    pos: caller_data.data.name_pos,
                                                });
                                            }
                                        } else {
                                            panic!("Unexpected parser error");
                                        }
                                    }
                                    definers::DefinerCollecting::Function(_) => {
                                        panic!("Definer Resolving on 'Function' is not supported");
                                    }
                                    definers::DefinerCollecting::Cloak(_) => {
                                        panic!("Definer Resolving on 'Cloak' is not supported");
                                    }
                                    definers::DefinerCollecting::Collective(_) => {
                                        panic!(
                                            "Definer Resolving on 'Collective' is not supported"
                                        );
                                    }
                                    definers::DefinerCollecting::Dynamic => {
                                        panic!("Definer Resolving on 'Dynamic' is not supported");
                                    }
                                    definers::DefinerCollecting::Error(i) => {
                                        panic!("unexpectedBehavior({})", i)
                                    }
                                }
                            }
                        }
                    } else {
                        errors.push(error::Error {
                            path: self.options.path.clone(),
                            scope: self.scope.scope_name.clone(),
                            debug_message: "8f192a4ce1f6f67e458f0c24fe62118f".to_owned(),
                            title: error::errorList::error_s25.title.clone(),
                            code: error::errorList::error_s25.code,
                            message: error::errorList::error_s25.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s25.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: caller_data.data.name.clone(),
                                }],
                            ),
                            pos: caller_data.data.name_pos,
                        });
                    }
                }
            }
            Collecting::Function(e) => {
                if e.data.name == caller_data.data.name && (e.data.public || layer == 0) {
                    found_type = e.data.return_type;
                    found = true;
                    if caller_data.data.params.len() != e.data.parameters.len()
                        && (caller_data.data.params.len() > 0
                            && !e.data.parameters[e.data.parameters.len() - 1].multi_capture)
                    {
                        errors.push(error::Error {
                            path: self.options.path.clone(),
                            scope: self.scope.scope_name.clone(),
                            debug_message: "dac6c699296c761edf53f81c0b584eb3".to_owned(),
                            title: error::errorList::error_s7.title.clone(),
                            code: error::errorList::error_s7.code,
                            message: error::errorList::error_s7.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s7.message.clone(),
                                vec![
                                    error::ErrorBuildField {
                                        key: "name".to_owned(),
                                        value: "Function".to_owned(),
                                    },
                                    error::ErrorBuildField {
                                        key: "token".to_owned(),
                                        value: e.data.parameters.len().to_string(),
                                    },
                                    error::ErrorBuildField {
                                        key: "token2".to_owned(),
                                        value: caller_data.data.params.len().to_string(),
                                    },
                                ],
                            ),
                            pos: caller_data.data.name_pos,
                        });
                    } else {
                        for (index, caller_param) in
                            caller_data.data.params.clone().into_iter().enumerate()
                        {
                            let to_match = if index > e.data.parameters.len() - 1 {
                                //If multi capturer function
                                e.data.parameters[e.data.parameters.len() - 1].rtype.clone()
                            } else {
                                e.data.parameters[index].rtype.clone()
                            };

                            match to_match {
                                definers::DefinerCollecting::Array(_) => {
                                    panic!("Definer Resolving on 'Array' is not supported");
                                }
                                definers::DefinerCollecting::GrowableArray(_) => {
                                    panic!("Definer Resolving on 'GrowableArray' is not supported");
                                }
                                definers::DefinerCollecting::Nullable(_) => {
                                    panic!("Definer Resolving on 'Nullable' is not supported");
                                }
                                definers::DefinerCollecting::Future(_) => {
                                    panic!("Definer Resolving on 'Future' is not supported");
                                }
                                definers::DefinerCollecting::Generic(e) => {
                                    let resolved_type_option =
                                        self.resolve_variable(caller_param.value, false);

                                    if let Ok(resolved_type) = resolved_type_option {
                                        if !resolved_type.is_generic()
                                            && resolved_type.as_generic().unwrap().rtype != e.rtype
                                        {
                                            errors.push(error::Error {
                                                path: self.options.path.clone(),
                                                scope: self.scope.scope_name.clone(),
                                                debug_message: "706148eeaf39215e37a0651a15a4298d"
                                                    .to_owned(),
                                                title: error::errorList::error_s3.title.clone(),
                                                code: error::errorList::error_s3.code,
                                                message: error::errorList::error_s3.message.clone(),
                                                builded_message: error::Error::build(
                                                    error::errorList::error_s3.message.clone(),
                                                    vec![
                                                        error::ErrorBuildField {
                                                            key: "token1".to_owned(),
                                                            value: e.rtype,
                                                        },
                                                        error::ErrorBuildField {
                                                            key: "token2".to_owned(),
                                                            value: resolved_type
                                                                .raw_name_with_extensions(),
                                                        },
                                                    ],
                                                ),
                                                pos: caller_param.pos,
                                            });
                                        }
                                    } else {
                                        panic!("Unexpected parser error");
                                    }
                                }
                                definers::DefinerCollecting::Function(_) => {
                                    panic!("Definer Resolving on 'Function' is not supported");
                                }
                                definers::DefinerCollecting::Cloak(_) => {
                                    panic!("Definer Resolving on 'Cloak' is not supported");
                                }
                                definers::DefinerCollecting::Collective(_) => {
                                    panic!("Definer Resolving on 'Cloak' is not supported");
                                }
                                definers::DefinerCollecting::Dynamic => {
                                    #[cfg(feature = "std")]
                                    std::println!("\u{001b}[33m[Experimental]\u{001b}[0m: Resolving type as dynamic");
                                    let resolved_type_option =
                                        self.resolve_variable(caller_param.value.clone(), false);

                                    if let Ok(_) = resolved_type_option {
                                        if caller_param.value.clone().get_type() == "variable" {
                                            errors.push(error::Error {
                                                path: self.options.path.clone(),
                                                scope: self.scope.scope_name.clone(),
                                                debug_message: "edd2c44a76338b064960e53360a5f47d"
                                                    .to_owned(),
                                                title: error::errorList::error_s4.title.clone(),
                                                code: error::errorList::error_s4.code,
                                                message: error::errorList::error_s4.message.clone(),
                                                builded_message: error::Error::build(
                                                    error::errorList::error_s4.message.clone(),
                                                    vec![error::ErrorBuildField {
                                                        key: "token".to_owned(),
                                                        value: caller_param
                                                            .value
                                                            .as_variable_type()
                                                            .unwrap()
                                                            .data
                                                            .value
                                                            .clone(),
                                                    }],
                                                ),
                                                pos: caller_param.pos,
                                            });
                                        }
                                    } else {
                                        panic!("Unexpected parser error");
                                    }
                                }
                                definers::DefinerCollecting::Error(i) => {
                                    panic!("unexpectedBehavior({})", i)
                                }
                            }
                        }
                    }
                }
            }
            Collecting::Class(e) => {
                if e.data.name == caller_data.data.name && (e.data.public || layer == 0) {
                    errors.push(error::Error {
                        path: self.options.path.clone(),
                        scope: self.scope.scope_name.clone(),
                        debug_message: "d10e63735c35f3c1d42b093f4645f827".to_owned(),
                        title: error::errorList::error_s25.title.clone(),
                        code: error::errorList::error_s25.code,
                        message: error::errorList::error_s25.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s25.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: caller_data.data.name.clone(),
                            }],
                        ),
                        pos: caller_data.data.name_pos,
                    });
                }
            }
            Collecting::NativeFunction(e) => {
                if e.name == caller_data.data.name && (e.public || layer == 0) {
                    found_type = e.return_type;
                    found = true;
                    if caller_data.data.params.len() != e.parameters.len()
                        && (caller_data.data.params.len() > 0
                            && !e.parameters[e.parameters.len() - 1].multi_capture)
                    {
                        errors.push(error::Error {
                            path: self.options.path.clone(),
                            scope: self.scope.scope_name.clone(),
                            debug_message: "2e8f868082db0d261ec89dae914fd1dc".to_owned(),
                            title: error::errorList::error_s7.title.clone(),
                            code: error::errorList::error_s7.code,
                            message: error::errorList::error_s7.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s7.message.clone(),
                                vec![
                                    error::ErrorBuildField {
                                        key: "name".to_owned(),
                                        value: "Function".to_owned(),
                                    },
                                    error::ErrorBuildField {
                                        key: "token".to_owned(),
                                        value: e.parameters.len().to_string(),
                                    },
                                    error::ErrorBuildField {
                                        key: "token2".to_owned(),
                                        value: caller_data.data.params.len().to_string(),
                                    },
                                ],
                            ),
                            pos: caller_data.data.name_pos,
                        });
                    } else {
                        for (index, caller_param) in
                            caller_data.data.params.clone().into_iter().enumerate()
                        {
                            let to_match = if index > e.parameters.len() - 1 {
                                //If multi capturer function
                                e.parameters[e.parameters.len() - 1].rtype.clone()
                            } else {
                                e.parameters[index].rtype.clone()
                            };

                            match to_match {
                                definers::DefinerCollecting::Array(_) => {
                                    panic!("Definer Resolving on 'Array' is not supported");
                                }
                                definers::DefinerCollecting::GrowableArray(_) => {
                                    panic!("Definer Resolving on 'GrowableArray' is not supported");
                                }
                                definers::DefinerCollecting::Nullable(_) => {
                                    panic!("Definer Resolving on 'Nullable' is not supported");
                                }
                                definers::DefinerCollecting::Future(_) => {
                                    panic!("Definer Resolving on 'Future' is not supported");
                                }
                                definers::DefinerCollecting::Generic(e) => {
                                    let resolved_type_option =
                                        self.resolve_variable(caller_param.value.clone(), false);

                                    match resolved_type_option {
                                        Ok(resolved_type) => {
                                            if !resolved_type.is_generic()
                                                && resolved_type.as_generic().unwrap().rtype
                                                    != e.rtype
                                            {
                                                errors.push(error::Error {
                                                    path: self.options.path.clone(),
                                                    scope: self.scope.scope_name.clone(),
                                                    debug_message:
                                                        "de2ef18362c26ce3204dac5a07ee4c31"
                                                            .to_owned(),
                                                    title: error::errorList::error_s3.title.clone(),
                                                    code: error::errorList::error_s3.code,
                                                    message: error::errorList::error_s3
                                                        .message
                                                        .clone(),
                                                    builded_message: error::Error::build(
                                                        error::errorList::error_s3.message.clone(),
                                                        vec![
                                                            error::ErrorBuildField {
                                                                key: "token1".to_owned(),
                                                                value: e.rtype,
                                                            },
                                                            error::ErrorBuildField {
                                                                key: "token2".to_owned(),
                                                                value: resolved_type
                                                                    .raw_name_with_extensions(),
                                                            },
                                                        ],
                                                    ),
                                                    pos: caller_param.pos,
                                                });
                                            }
                                        }
                                        Err(found_errors) => errors.extend(found_errors),
                                    }
                                }
                                definers::DefinerCollecting::Function(_) => {
                                    panic!("Definer Resolving on 'Function' is not supported");
                                }
                                definers::DefinerCollecting::Cloak(_) => {
                                    panic!("Definer Resolving on 'Cloak' is not supported");
                                }
                                definers::DefinerCollecting::Collective(_) => {
                                    panic!("Definer Resolving on 'Cloak' is not supported");
                                }
                                definers::DefinerCollecting::Dynamic => {
                                    #[cfg(feature = "std")]
                                    std::println!("\u{001b}[33m[Experimental]\u{001b}[0m: Resolving type as dynamic");
                                    let resolved_type_option =
                                        self.resolve_variable(caller_param.value.clone(), false);

                                    if let Ok(_) = resolved_type_option {
                                        if caller_param.value.clone().get_type() == "variable" {
                                            errors.push(error::Error {
                                                path: self.options.path.clone(),
                                                scope: self.scope.scope_name.clone(),
                                                debug_message: "26f27fca82b0628a00997a31dc9802a0"
                                                    .to_owned(),
                                                title: error::errorList::error_s4.title.clone(),
                                                code: error::errorList::error_s4.code,
                                                message: error::errorList::error_s4.message.clone(),
                                                builded_message: error::Error::build(
                                                    error::errorList::error_s4.message.clone(),
                                                    vec![error::ErrorBuildField {
                                                        key: "token".to_owned(),
                                                        value: caller_param
                                                            .value
                                                            .as_variable_type()
                                                            .unwrap()
                                                            .data
                                                            .value
                                                            .clone(),
                                                    }],
                                                ),
                                                pos: caller_param.pos,
                                            });
                                        }
                                    } else {
                                        panic!("Unexpected parser error");
                                    }
                                }
                                definers::DefinerCollecting::Error(i) => {
                                    panic!("unexpectedBehavior({})", i)
                                }
                            }
                        }
                    }
                }
            }
            Collecting::ImportItem(e) => {
                match self.deep_function_call_resolver(
                    caller_data.clone(),
                    *e.item,
                    errors,
                    if e.from_path == "<temporary>" {
                        0
                    } else {
                        layer + 1
                    },
                ) {
                    Some(e) => {
                        found_type = e;
                        found = true;
                    }
                    _ => (),
                }
            }
            _ => (),
        };
        if found {
            Some(found_type)
        } else {
            None
        }
    }

    pub fn resolve_function_call(
        &self,
        caller_data: types::function_call::FunctionCallCollector,
    ) -> Result<definers::DefinerCollecting, Vec<ellie_core::error::Error>> {
        let mut found_type = definers::DefinerCollecting::Dynamic;
        let mut found = false;
        let mut errors = Vec::new();
        'scp: for item in self.collected.clone() {
            match self.deep_function_call_resolver(caller_data.clone(), item, &mut errors, 0) {
                Some(e) => {
                    found_type = e;
                    found = true;
                    break 'scp;
                }
                _ => (),
            }
        }

        if !found {
            errors.push(error::Error {
                path: self.options.path.clone(),
                scope: self.scope.scope_name.clone(),
                debug_message: "42ff0a261d16264d03d996399fa76651".to_owned(),
                title: error::errorList::error_s6.title.clone(),
                code: error::errorList::error_s6.code,
                message: error::errorList::error_s6.message.clone(),
                builded_message: error::Error::build(
                    error::errorList::error_s6.message.clone(),
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: caller_data.data.name.clone(),
                    }],
                ),
                pos: caller_data.data.name_pos,
            });
        }

        if errors.is_empty() {
            Ok(found_type)
        } else {
            Err(errors)
        }
    }

    pub fn generic_type_exists(&self, name: String) -> bool {
        let mut found = false;
        for item in self.generic_variables.clone() {
            if item.name == name {
                found = true;
                break;
            }
        }
        found
    }

    pub fn type_exists(&self, name: String) -> bool {
        let mut found = false;
        for item in self.collected.clone() {
            if let Collecting::Class(ref e) = item {
                if e.data.name == name {
                    found = e.data.name == name;
                    break;
                }
            } else if let Collecting::ImportItem(ref e) = item {
                if let Collecting::Class(class) = *e.item.clone() {
                    if class.data.name == name && class.data.public {
                        found = class.data.name == name;
                        break;
                    }
                }
            }
        }
        found
    }

    pub fn check_key_keyword(&self, name: String) -> bool {
        let mut found = false;
        for item in self.collected.clone() {
            match item.clone() {
                Collecting::FileKey(e) => {
                    if e.data.key_name == name {
                        found = true;
                        break;
                    }
                }
                _ => (),
            }
        }
        found
    }

    pub fn deep_check_keyword(
        &self,
        item: Collecting,
        name: String,
        contain_private: bool,
        contain_setter: bool,
        layer: usize,
    ) -> (bool, Collecting) {
        let mut found = false;
        let mut found_item: Collecting = Collecting::None;
        match item.clone() {
            Collecting::Variable(e)
                if e.data.name == name && (e.data.public || contain_private || layer == 0) =>
            {
                if let types::Types::VariableType(referenced_var) = e.data.value {
                    let deep_check =
                        self.check_keyword(referenced_var.data.value, contain_private, false);
                    if deep_check.found {
                        found = true;
                        found_item = match deep_check.found_type {
                            NameCheckResponseType::Variable(e) => Collecting::Variable(e),
                            NameCheckResponseType::Getter(e) => Collecting::Getter(e),
                            NameCheckResponseType::Setter(e) => Collecting::Setter(e),
                            NameCheckResponseType::Function(e) => Collecting::Function(e),
                            NameCheckResponseType::NativeFunction(e) => {
                                Collecting::NativeFunction(e)
                            }
                            NameCheckResponseType::Class(e) => Collecting::Class(e),
                            NameCheckResponseType::None => panic!("Unexpected parser behaviour"),
                        };
                    } else {
                        found = true;
                        found_item = item;
                    }
                } else {
                    found = true;
                    found_item = item;
                }
            }
            Collecting::Setter(e) => {
                if e.data.name == name
                    && (e.data.public || contain_private || layer == 0 || contain_setter)
                {
                    found = true;
                    found_item = item;
                }
            }
            Collecting::Getter(e) => {
                if e.data.name == name && (e.data.public || contain_private || layer == 0) {
                    found = true;
                    found_item = item;
                }
            }
            Collecting::Function(e) => {
                if e.data.name == name && (e.data.public || contain_private || layer == 0) {
                    found = true;
                    found_item = item;
                }
            }
            Collecting::NativeFunction(e) => {
                if e.name == name && (e.public || contain_private || layer == 0) {
                    found = true;
                    found_item = item;
                }
            }
            Collecting::Class(e) => {
                if e.data.name == name && (e.data.public || contain_private || layer == 0) {
                    found = true;
                    found_item = item;
                }
            }
            Collecting::FileKey(e) => {
                if e.data.key_name == name {
                    found = true;
                    found_item = item;
                }
            }
            Collecting::ImportItem(e) => {
                if e.public || contain_private {
                    let (is_found, found_item_target) = self.deep_check_keyword(
                        *e.item,
                        name,
                        contain_private,
                        false,
                        if e.from_path == "<temporary>" {
                            0
                        } else {
                            layer + 1
                        },
                    );
                    found = is_found;
                    found_item = found_item_target;
                }
            }
            _ => (),
        };
        (found, found_item)
    }

    pub fn deep_resolve_reference_proto(
        &self,
        item: Collecting,
        name: String,
        contain_private: bool,
        layer: usize,
    ) -> (bool, Collecting) {
        let mut found = false;
        let mut found_item: Collecting = Collecting::None;
        match item.clone() {
            Collecting::Variable(e)
                if e.data.name == name && (e.data.public || contain_private || layer == 0) =>
            {
                let class = match e.data.value {
                    types::Types::Integer(_) => {
                        let layer_check =
                            self.resolve_reference_proto("integer".to_owned(), contain_private);
                        if layer_check.found {
                            layer_check.found_type.as_class().unwrap().clone()
                        } else {
                            //TODO handle error
                            panic!("Error: Failed to resolve generic");
                        }
                    }
                    types::Types::Float(_) => {
                        let layer_check =
                            self.resolve_reference_proto("float".to_owned(), contain_private);
                        if layer_check.found {
                            layer_check.found_type.as_class().unwrap().clone()
                        } else {
                            //TODO handle error
                            panic!("Error: Failed to resolve generic");
                        }
                    }
                    types::Types::Bool(_) => {
                        let layer_check =
                            self.resolve_reference_proto("bool".to_owned(), contain_private);
                        if layer_check.found {
                            layer_check.found_type.as_class().unwrap().clone()
                        } else {
                            //TODO handle error
                            panic!("Error: Failed to resolve generic");
                        }
                    }
                    types::Types::String(_) => {
                        let layer_check =
                            self.resolve_reference_proto("string".to_owned(), contain_private);
                        if layer_check.found {
                            layer_check.found_type.as_class().unwrap().clone()
                        } else {
                            //TODO handle error
                            panic!("Error: Failed to resolve generic");
                        }
                    }
                    types::Types::Char(_) => {
                        let layer_check =
                            self.resolve_reference_proto("char".to_owned(), contain_private);
                        if layer_check.found {
                            layer_check.found_type.as_class().unwrap().clone()
                        } else {
                            //TODO handle error
                            panic!("Error: Failed to resolve generic");
                        }
                    }
                    types::Types::Collective(_) => {
                        let layer_check =
                            self.resolve_reference_proto("collective".to_owned(), contain_private);
                        if layer_check.found {
                            layer_check.found_type.as_class().unwrap().clone()
                        } else {
                            //TODO handle error
                            panic!("Error: Failed to resolve generic");
                        }
                    }
                    types::Types::Reference(e) => {
                        let layer_check =
                            self.resolve_reference_proto(e.last_entry.raw_name(), contain_private);
                        if layer_check.found {
                            layer_check.found_type.as_class().unwrap().clone()
                        } else {
                            //TODO handle error
                            panic!("Error: Failed to resolve generic");
                        }
                    }
                    types::Types::BracketReference(e) => {
                        //let layer_check =
                        //    self.resolve_reference_proto(e.last_entry.raw_name(), contain_private);
                        //if layer_check.found {
                        //    layer_check.found_type.as_class().unwrap().clone()
                        //} else {
                        //    //TODO handle error
                        //    panic!("Error: Failed to resolve generic");
                        //}
                        todo!("NOT IMPLEMENTED")
                    }
                    types::Types::Operator(e) => {
                        let generic = match e.data.operator {
                            types::operator_type::Operators::ComparisonType(_) => {
                                crate::syntax::definers::DefinerCollecting::Generic(
                                    crate::syntax::definers::GenericType {
                                        rtype: "bool".to_owned(),
                                        hash: "ellie_bool_hash".to_owned(),
                                    },
                                )
                            }
                            types::operator_type::Operators::LogicalType(op) => match op {
                                types::logical_type::LogicalOperators::And => {
                                    e.data.second.to_definer()
                                }
                                types::logical_type::LogicalOperators::Or => {
                                    e.data.first.to_definer()
                                }
                                types::logical_type::LogicalOperators::Null => {
                                    panic!("Unexpected parser behaviour")
                                }
                            },
                            types::operator_type::Operators::ArithmeticType(_) => {
                                e.data.first.to_definer()
                            }
                            types::operator_type::Operators::AssignmentType(_) => {
                                e.data.first.to_definer()
                            }
                            types::operator_type::Operators::Null => {
                                panic!("Unexpected parser behaviour")
                            }
                        }
                        .raw_name();
                        let layer_check = self.resolve_reference_proto(generic, contain_private);
                        if layer_check.found {
                            layer_check.found_type.as_class().unwrap().clone()
                        } else {
                            //TODO handle error
                            panic!("Error: Failed to resolve generic");
                        }
                    }
                    types::Types::Cloak(e) => {
                        let layer_check = self.resolve_reference_proto(
                            if e.data.collective.len() == 1 {
                                e.data.collective[e.data.collective.len() - 1]
                                    .value
                                    .clone()
                                    .to_definer()
                                    .raw_name()
                            } else {
                                "cloak".to_owned()
                            },
                            contain_private,
                        );
                        if layer_check.found {
                            layer_check.found_type.as_class().unwrap().clone()
                        } else {
                            //TODO handle error
                            panic!("Error: Failed to resolve generic");
                        }
                    }
                    types::Types::Array(_) => {
                        let layer_check =
                            self.resolve_reference_proto(e.data.rtype.raw_name(), contain_private);
                        if layer_check.found {
                            layer_check.found_type.as_class().unwrap().clone()
                        } else {
                            //TODO handle error
                            panic!("Error: Failed to resolve generic");
                        }
                    }
                    types::Types::ArrowFunction(_) => {
                        let layer_check =
                            self.resolve_reference_proto("function".to_owned(), contain_private);
                        if layer_check.found {
                            layer_check.found_type.as_class().unwrap().clone()
                        } else {
                            //TODO handle error
                            panic!("Error: Failed to resolve generic");
                        }
                    }
                    types::Types::ConstructedClass(e) => {
                        let layer_check =
                            self.resolve_reference_proto(e.data.class_name(), contain_private);
                        if layer_check.found {
                            layer_check.found_type.as_class().unwrap().clone()
                        } else {
                            //TODO handle error
                            panic!("Error: Failed to resolve generic");
                        }
                    }
                    types::Types::FunctionCall(e) => {
                        let layer_check =
                            self.resolve_reference_proto(e.return_type.raw_name(), contain_private);
                        if layer_check.found {
                            layer_check.found_type.as_class().unwrap().clone()
                        } else {
                            //TODO handle error
                            panic!("Error: Failed to resolve generic");
                        }
                    }
                    types::Types::Void => {
                        let layer_check =
                            self.resolve_reference_proto("void".to_owned(), contain_private);
                        if layer_check.found {
                            layer_check.found_type.as_class().unwrap().clone()
                        } else {
                            //TODO handle error
                            panic!("Error: Failed to resolve generic");
                        }
                    }
                    types::Types::NullResolver(_) => {
                        let layer_check = self
                            .resolve_reference_proto("nullResolver".to_owned(), contain_private);
                        if layer_check.found {
                            layer_check.found_type.as_class().unwrap().clone()
                        } else {
                            //TODO handle error
                            panic!("Error: Failed to resolve generic");
                        }
                    }
                    types::Types::Negative(_) => {
                        let layer_check =
                            self.resolve_reference_proto("bool".to_owned(), contain_private);
                        if layer_check.found {
                            layer_check.found_type.as_class().unwrap().clone()
                        } else {
                            //TODO handle error
                            panic!("Error: Failed to resolve generic");
                        }
                    }
                    types::Types::VariableType(referenced_var) => {
                        let deep_check = self
                            .resolve_reference_proto(referenced_var.data.value, contain_private);

                        if deep_check.found {
                            let found_generic = match deep_check.found_type {
                                NameCheckResponseType::Variable(e) => {
                                    let layer_check = self.resolve_reference_proto(
                                        e.data.value.to_definer().raw_name(),
                                        contain_private,
                                    );
                                    if layer_check.found {
                                        layer_check.found_type.as_class().unwrap().clone()
                                    } else {
                                        //TODO handle error
                                        panic!("Error: Failed to resolve generic");
                                    }
                                }
                                NameCheckResponseType::Getter(e) => {
                                    let layer_check = self.resolve_reference_proto(
                                        e.data.rtype.raw_name(),
                                        contain_private,
                                    );
                                    if layer_check.found {
                                        layer_check.found_type.as_class().unwrap().clone()
                                    } else {
                                        //TODO handle error
                                        panic!("Error: Failed to resolve generic");
                                    }
                                }
                                NameCheckResponseType::Function(_) => {
                                    let layer_check = self.resolve_reference_proto(
                                        "function".to_owned(),
                                        contain_private,
                                    );
                                    if layer_check.found {
                                        layer_check.found_type.as_class().unwrap().clone()
                                    } else {
                                        //TODO handle error
                                        panic!("Error: Failed to resolve generic");
                                    }
                                }
                                NameCheckResponseType::NativeFunction(_) => {
                                    let layer_check = self.resolve_reference_proto(
                                        "function".to_owned(),
                                        contain_private,
                                    );
                                    if layer_check.found {
                                        layer_check.found_type.as_class().unwrap().clone()
                                    } else {
                                        //TODO handle error
                                        panic!("Error: Failed to resolve generic");
                                    }
                                }
                                NameCheckResponseType::Class(e) => e,
                                _ => panic!("Unexpected parser behaviour"),
                            };
                            found_generic
                        } else {
                            //TODO handle error
                            panic!("Error: Failed to resolve generic");
                        }
                    }
                    types::Types::Null => todo!(),
                };
                found = true;
                found_item = Collecting::Class(class);
            }
            Collecting::Getter(e) => {
                if e.data.name == name && (e.data.public || contain_private || layer == 0) {
                    let layer_check =
                        self.resolve_reference_proto(e.data.rtype.raw_name(), contain_private);
                    if layer_check.found {
                        found = true;
                        found_item =
                            Collecting::Class(layer_check.found_type.as_class().unwrap().clone());
                    } else {
                        //TODO handle error
                        panic!("Error: Failed to resolve generic");
                    }
                }
            }
            Collecting::Function(e) => {
                if e.data.name == name && (e.data.public || contain_private || layer == 0) {
                    let layer_check =
                        self.resolve_reference_proto("function".to_owned(), contain_private);
                    if layer_check.found {
                        found = true;
                        found_item =
                            Collecting::Class(layer_check.found_type.as_class().unwrap().clone());
                    } else {
                        //TODO handle error
                        panic!("Error: Failed to resolve generic");
                    }
                }
            }
            Collecting::NativeFunction(e) => {
                if e.name == name && (e.public || contain_private || layer == 0) {
                    let layer_check =
                        self.resolve_reference_proto("function".to_owned(), contain_private);
                    if layer_check.found {
                        found = true;
                        found_item =
                            Collecting::Class(layer_check.found_type.as_class().unwrap().clone());
                    } else {
                        //TODO handle error
                        panic!("Error: Failed to resolve generic");
                    }
                }
            }
            Collecting::Class(e) => {
                if e.data.name == name && (e.data.public || contain_private || layer == 0) {
                    if name == "class" {
                        found = true;
                        found_item = item;
                    } else {
                        let layer_check =
                            self.resolve_reference_proto("class".to_owned(), contain_private);

                        if layer_check.found {
                            found = true;
                            found_item = Collecting::Class(
                                layer_check.found_type.as_class().unwrap().clone(),
                            );
                        } else {
                            //TODO handle error
                            panic!("Error: Failed to resolve generic");
                        }
                    }
                }
            }
            Collecting::ImportItem(e) => {
                if e.public || contain_private {
                    let (is_found, found_item_target) = self.deep_resolve_reference_proto(
                        *e.item,
                        name,
                        contain_private,
                        if e.from_path == "<temporary>" {
                            0
                        } else {
                            layer + 1
                        },
                    );
                    found = is_found;
                    found_item = found_item_target;
                }
            }
            _ => (),
        };
        (found, found_item)
    }

    pub fn resolve_reference_proto(
        &self,
        name: String,
        contain_private: bool,
    ) -> NameCheckResponse {
        let mut found = false;
        let mut found_item: Collecting = Collecting::None;

        for item in self.collected.clone() {
            let (is_found, found_item_target) =
                self.deep_resolve_reference_proto(item, name.clone(), contain_private, 0);
            found = is_found;
            found_item = found_item_target;
            if found {
                break;
            }
        }

        match found_item {
            Collecting::Class(e) => NameCheckResponse {
                found,
                found_type: NameCheckResponseType::Class(e),
            },
            _ => panic!(
                "Unexpected behaviour: {:#?} found: {}, target: {}",
                found_item, found, name
            ),
        }
    }

    pub fn check_keyword(
        &self,
        name: String,
        contain_private: bool,
        contain_setter: bool,
    ) -> NameCheckResponse {
        let mut found = false;
        let mut found_item: Collecting = Collecting::None;

        for item in self.collected.clone() {
            let (is_found, found_item_target) =
                self.deep_check_keyword(item, name.clone(), contain_private, contain_setter, 0);
            found = is_found;
            found_item = found_item_target;
            if found {
                break;
            }
        }

        match found_item {
            Collecting::Variable(e) => NameCheckResponse {
                found,
                found_type: NameCheckResponseType::Variable(e),
            },
            Collecting::Getter(e) => NameCheckResponse {
                found,
                found_type: NameCheckResponseType::Getter(e),
            },
            Collecting::Setter(e) => NameCheckResponse {
                found,
                found_type: NameCheckResponseType::Setter(e),
            },
            Collecting::Function(e) => NameCheckResponse {
                found,
                found_type: NameCheckResponseType::Function(e),
            },
            Collecting::NativeFunction(e) => NameCheckResponse {
                found,
                found_type: NameCheckResponseType::NativeFunction(e),
            },
            Collecting::Class(e) => NameCheckResponse {
                found,
                found_type: NameCheckResponseType::Class(e),
            },
            _ => NameCheckResponse {
                found: found,
                found_type: NameCheckResponseType::None,
            },
        }
    }

    pub fn import_exists(self, path_name: &str) -> bool {
        let mut found = false;
        for i in self.collected {
            if let Collecting::ImportItem(import_item) = i {
                if import_item.from_path == path_name {
                    found = true;
                    break;
                }
            }
        }
        found
    }
}
