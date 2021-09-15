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
            Collecting::Enum(e) => definite::items::Collecting::Enum(e.to_definite()),
            Collecting::NativeFunction(e) => {
                definite::items::Collecting::NativeFunction(e.to_definite())
            }
            Collecting::None => definite::items::Collecting::None,
        }
    }
}

impl Default for Collecting {
    fn default() -> Self {
        Collecting::None
    }
}

#[derive(EnumAsInner)]
pub enum NameCheckResponseType {
    Variable(variable::VariableCollector),
    Getter(getter::GetterCollector),
    Setter(setter::SetterCollector),
    Function(function::FunctionCollector),
    Class(class::ClassCollector),
    None,
}

#[derive(PartialEq, Debug)]
pub enum DeepCallResponse {
    TypeResponse(types::Types),
    ElementResponse(Collecting),
    NoElement,
}

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
}

impl RawParser {
    pub fn to_parser(
        self,
        _resolver: fn(defs::ParserOptions, String) -> ResolvedImport,
    ) -> Parser<impl FnMut(com::Message) + Clone + Sized> {
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
        }
    }

    pub fn to_no_resolver_parser(self) -> Parser<impl FnMut(com::Message) + Clone + Sized> {
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
        }
    }
}

#[derive(Clone)]
pub struct Parser<F> {
    pub scope: Box<scope::Scope>,
    pub resolver: fn(ellie_core::defs::ParserOptions, String, bool) -> ResolvedImport,
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
}

#[derive(Debug)]
pub enum ResolvedFileContent {
    PreBuilt(Vec<ellie_core::definite::items::Collecting>),
    Raw(String),
}

impl Default for ResolvedFileContent {
    fn default() -> Self {
        ResolvedFileContent::Raw("".to_owned())
    }
}

#[derive(Default, Debug)]
pub struct ResolvedImport {
    pub found: bool,
    pub resolve_error: String,
    pub resolved_path: String,
    pub file_content: ResolvedFileContent,
}

impl<F> Parser<F>
where
    F: FnMut(com::Message) + Clone + Sized,
{
    pub fn new(
        code: String,
        resolve_import: fn(ellie_core::defs::ParserOptions, String, bool) -> ResolvedImport,
        com: F,
        options: defs::ParserOptions,
    ) -> Parser<F> {
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
        }
    }

    pub fn map(mut self) -> ParserResponse {
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

        if self.current != Collecting::None || !self.keyword_catch.trim().is_empty() {
            errors.push(error::Error {
                path: self.options.path.clone(),
                scope: "definer_processor".to_owned(),
                debug_message: "ae1e06a9f420f6bff3caa473400245fe".to_owned(),
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

    pub fn is_iterable(&self, target: DeepCallResponse) -> bool {
        match target {
            DeepCallResponse::TypeResponse(e) => match e {
                types::Types::Integer(_) => true,
                types::Types::Float(_) => false,
                types::Types::Bool(_) => true,
                types::Types::String(_) => true,
                _ => todo!(),
            },
            DeepCallResponse::ElementResponse(_e) => false,
            DeepCallResponse::NoElement => false,
        }
    }

    pub fn look_up_for_item(self, item: Collecting, target: String) -> Option<Collecting> {
        let mut found_item: Option<Collecting> = None;

        match item.clone() {
            Collecting::Variable(c) => {
                if target == c.data.name {
                    found_item = Some(item);
                }
            }
            Collecting::Function(c) => {
                if target == c.data.name {
                    found_item = Some(item);
                }
            }
            Collecting::Class(c) => {
                if target == c.data.name {
                    found_item = Some(item);
                }
            }
            Collecting::Constructor(c) => {
                if target == c.data.name {
                    found_item = Some(item);
                }
            }
            Collecting::ImportItem(c) => {
                if c.public {
                    found_item = self.look_up_for_item(*c.item, target);
                }
            }
            _ => found_item = None,
        }

        found_item
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
            types::Types::Array(_) => todo!(),
            types::Types::Cloak(_) => todo!(),
            types::Types::Reference(_) => todo!(),
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
                let vr_found = self.check_keyword(name, false);
                if vr_found.found {
                    if let NameCheckResponseType::Variable(v_data) = vr_found.found_type {
                        self.resolve_deep_call(v_data.data.value)
                    } else if let NameCheckResponseType::Function(f_data) = vr_found.found_type {
                        DeepCallResponse::ElementResponse(Collecting::Function(f_data))
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
                let vr_found = self.check_keyword(e.data.value, false);

                if vr_found.found {
                    if let NameCheckResponseType::Variable(v_data) = vr_found.found_type {
                        self.resolve_deep_call(v_data.data.value)
                    } else if let NameCheckResponseType::Function(f_data) = vr_found.found_type {
                        DeepCallResponse::ElementResponse(Collecting::Function(f_data))
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
    ) -> Result<String, Vec<ellie_core::error::Error>> {
        match target {
            types::Types::Integer(_) => Ok("int".to_owned()),
            types::Types::Float(_) => Ok("float".to_owned()),
            types::Types::String(_) => Ok("string".to_owned()),
            types::Types::Char(_) => Ok("char".to_owned()),
            types::Types::Bool(_) => Ok("bool".to_owned()),
            types::Types::NullResolver(e) => Ok(e.value.get_type()),
            types::Types::Negative(_) => Ok("bool".to_owned()),
            types::Types::Collective(_) => Ok("collective".to_owned()),
            types::Types::Cloak(_) => Ok("cloak".to_owned()),
            types::Types::Array(_) => Ok("array".to_owned()),
            types::Types::Void => Ok("void".to_owned()),
            types::Types::Reference(reference_data) => {
                let mut errors = Vec::new();
                let deep_scan = self.resolve_deep_call(*reference_data.data.reference.clone());
                let mut found: String = String::new();

                match deep_scan {
                    DeepCallResponse::TypeResponse(_) => {
                        panic!("Direct type reference is not ready")
                    }
                    DeepCallResponse::ElementResponse(target) => {
                        let mut last: definers::DefinerCollecting =
                            definers::DefinerCollecting::Dynamic;

                        fn resolve_reference_chain(
                            target: Collecting,
                            chain: types::Types,
                        ) -> Result<definers::DefinerCollecting, (String, i8)>
                        {
                            /*
                                Error codes:
                                0 => cannot used as refferencer,
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
                                        .map(|x| (x.name, x.return_type))
                                        .collect::<Vec<_>>();
                                    let mut all_properties = vec![];
                                    all_properties.extend(properties);
                                    all_properties.extend(getters);
                                    all_properties.extend(setters);
                                    all_properties.extend(methods);

                                    match chain {
                                        types::Types::VariableType(chain_variable) => {
                                            let found_property = all_properties
                                                .into_iter()
                                                .find(|e| e.0 == chain_variable.data.value);
                                            match found_property {
                                                Some(e) => Ok(e.1),
                                                None => Err((chain_variable.data.value, 1)),
                                            }
                                        }
                                        e => Err((e.get_type(), 0)),
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

                        for (layer, chain) in reference_data.data.chain.into_iter().enumerate() {
                            if layer == 0 {
                                let resolved_reference_chain =
                                    resolve_reference_chain(target.clone(), chain.value);

                                match resolved_reference_chain {
                                    Ok(e) => {
                                        last = e;
                                    }
                                    Err(e) => {
                                        if e.1 == 0 {
                                            errors.push(error::Error {
                                                path: self.options.path.clone(),
                                                scope: self.scope.scope_name.clone(),
                                                debug_message: "replace_parser_537".to_owned(),
                                                title: error::errorList::error_s37.title.clone(),
                                                code: error::errorList::error_s37.code,
                                                message: error::errorList::error_s37
                                                    .message
                                                    .clone(),
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
                                                debug_message: "replace_parser_609".to_owned(),
                                                title: error::errorList::error_s34.title.clone(),
                                                code: error::errorList::error_s34.code,
                                                message: error::errorList::error_s34
                                                    .message
                                                    .clone(),
                                                builded_message: error::Error::build(
                                                    error::errorList::error_s34.message.clone(),
                                                    vec![error::ErrorBuildField {
                                                        key: "token".to_owned(),
                                                        value: e.0,
                                                    }],
                                                ),
                                                pos: chain.pos,
                                            });
                                        }
                                    }
                                }
                            } else {
                                match last.clone() {
                                    definers::DefinerCollecting::Array(_) => todo!(),
                                    definers::DefinerCollecting::Cloak(_) => todo!(),
                                    definers::DefinerCollecting::Future(_) => todo!(),
                                    definers::DefinerCollecting::Nullable(_) => todo!(),
                                    definers::DefinerCollecting::GrowableArray(_) => todo!(),
                                    definers::DefinerCollecting::Generic(e) => {
                                        let found_generic = self.check_keyword(e.rtype.clone(), true);

                                        if found_generic.found {
                                            match found_generic.found_type {
                                                NameCheckResponseType::Class(target) => {
                                                    let resolved_reference_chain =
                                                        resolve_reference_chain(
                                                            Collecting::Class(target),
                                                            chain.value,
                                                        );

                                                    match resolved_reference_chain {
                                                        Ok(e) => {
                                                            last = e;
                                                        }
                                                        Err(e) => {
                                                            if e.1 == 0 {
                                                                errors.push(error::Error {
                                                    path: self.options.path.clone(),
                                                    scope: self.scope.scope_name.clone(),
                                                    debug_message: "replace_parser_537".to_owned(),
                                                    title: error::errorList::error_s37.title.clone(),
                                                    code: error::errorList::error_s37.code,
                                                    message: error::errorList::error_s37
                                                        .message
                                                        .clone(),
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
                                                    debug_message: "replace_parser_674".to_owned(),
                                                    title: error::errorList::error_s34.title.clone(),
                                                    code: error::errorList::error_s34.code,
                                                    message: error::errorList::error_s34
                                                        .message
                                                        .clone(),
                                                    builded_message: error::Error::build(
                                                        error::errorList::error_s34.message.clone(),
                                                        vec![error::ErrorBuildField {
                                                            key: "token".to_owned(),
                                                            value: e.0,
                                                        }],
                                                    ),
                                                    pos: chain.pos,
                                                });
                                                            }
                                                        }
                                                    }
                                                }
                                                _ => panic!("Unexpected parser behaviour"),
                                            }
                                        } else {
                                            errors.push(error::Error {
                                                path: self.options.path.clone(),
                                                scope: self.scope.scope_name.clone(),
                                                debug_message: "replace_parser_700".to_owned(),
                                                title: error::errorList::error_s38.title.clone(),
                                                code: error::errorList::error_s38.code,
                                                message: error::errorList::error_s38
                                                    .message
                                                    .clone(),
                                                builded_message: error::Error::build(
                                                    error::errorList::error_s38.message.clone(),
                                                    vec![error::ErrorBuildField {
                                                        key: "token".to_owned(),
                                                        value: e.rtype,
                                                    }],
                                                ),
                                                pos: chain.pos,
                                            });
                                        }
                                    }
                                    definers::DefinerCollecting::Function(_) => todo!(),
                                    definers::DefinerCollecting::Collective(_) => todo!(),
                                    definers::DefinerCollecting::Dynamic => todo!(),
                                }
                            }
                        }
                    }
                    DeepCallResponse::NoElement => match *reference_data.data.reference.clone() {
                        types::Types::VariableType(e) => {
                            errors.push(error::Error {
                                path: self.options.path.clone(),
                                scope: self.scope.scope_name.clone(),
                                debug_message: "replace_parser_727".to_owned(),
                                title: error::errorList::error_s6.title.clone(),
                                code: error::errorList::error_s6.code,
                                message: error::errorList::error_s6.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s6.message.clone(),
                                    vec![error::ErrorBuildField {
                                        key: "token".to_owned(),
                                        value: e.data.value.to_string(),
                                    }],
                                ),
                                pos: reference_data.data.reference_pos,
                            });
                        }
                        types::Types::ConstructedClass(e) => {
                            errors.push(error::Error {
                                path: self.options.path.clone(),
                                scope: self.scope.scope_name.clone(),
                                debug_message: "replace_parser_727".to_owned(),
                                title: error::errorList::error_s6.title.clone(),
                                code: error::errorList::error_s6.code,
                                message: error::errorList::error_s6.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s6.message.clone(),
                                    vec![error::ErrorBuildField {
                                        key: "token".to_owned(),
                                        value: e.data.class_name(),
                                    }],
                                ),
                                pos: reference_data.data.reference_pos,
                            });
                        }
                        e => panic!(
                            "Unexpected parser behaviour, '{}' not supported for now",
                            e.get_type()
                        ),
                    },
                }

                if errors.len() == 0 {
                    Ok(found)
                } else {
                    Err(errors)
                }
            }
            types::Types::Operator(_) => {
                #[cfg(feature = "std")]
                std::println!("Not implemented for: types {:#?}", target);
                Ok("".to_owned())
            }
            types::Types::ArrowFunction(_) => Ok("function".to_owned()),
            types::Types::ConstructedClass(constructed_class) => {
                if let Ok(resolved) = self.resolve_new_call(constructed_class.clone()) {
                    let mut errors = vec![];
                    if let Collecting::Class(class_collector) = resolved {
                        if class_collector.data.constructor.parameters.len()
                            != constructed_class.data.params.len()
                        {
                            Err(vec![error::Error {
                                path: self.options.path.clone(),
                                scope: self.scope.scope_name.clone(),
                                debug_message: "replace_parser_577".to_owned(),
                                title: error::errorList::error_s19.title.clone(),
                                code: error::errorList::error_s19.code,
                                message: error::errorList::error_s19.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s19.message.clone(),
                                    vec![
                                        error::ErrorBuildField {
                                            key: "token".to_owned(),
                                            value: class_collector
                                                .data
                                                .constructor
                                                .parameters
                                                .len()
                                                .to_string(),
                                        },
                                        error::ErrorBuildField {
                                            key: "token2".to_owned(),
                                            value: constructed_class.data.params.len().to_string(),
                                        },
                                    ],
                                ),
                                pos: constructed_class.data.value_pos,
                            }])
                        } else {
                            let mut has_faulty_param = false;
                            for (pos, param) in
                                constructed_class.data.params.into_iter().enumerate()
                            {
                                let constructor_param =
                                    &class_collector.data.constructor.parameters[pos];
                                let properties = class_collector
                                    .data
                                    .properties
                                    .iter()
                                    .filter(|e| e.name == constructor_param.name)
                                    .collect::<Vec<&variable::Variable>>();

                                if properties.len() != 0 {
                                    let property = properties[0];
                                    if property.rtype.raw_name() != param.value.get_type() {
                                        errors.push(error::Error {
                                            path: self.options.path.clone(),
                                            scope: self.scope.scope_name.clone(),
                                            debug_message: "replace_parser_640".to_owned(),
                                            title: error::errorList::error_s3.title.clone(),
                                            code: error::errorList::error_s3.code,
                                            message: error::errorList::error_s3.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s3.message.clone(),
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "token1".to_owned(),
                                                        value: property.rtype.raw_name(),
                                                    },
                                                    error::ErrorBuildField {
                                                        key: "token2".to_owned(),
                                                        value: param.value.get_type(),
                                                    },
                                                ],
                                            ),
                                            pos: param.pos,
                                        });
                                    }
                                }
                            }

                            for param in class_collector.data.constructor.parameters {
                                let properties = class_collector
                                    .data
                                    .properties
                                    .iter()
                                    .filter(|e| e.name == param.name)
                                    .collect::<Vec<&variable::Variable>>();

                                if properties.len() != 0 {
                                    let property = properties[0];
                                }
                            }

                            if errors.is_empty() {
                                Ok(class_collector.data.name)
                            } else {
                                Err(errors)
                            }
                        }
                    } else {
                        panic!("Unexpected parser behaviour")
                    }
                } else {
                    Ok("nen".to_owned())
                }
            }
            types::Types::FunctionCall(e) => {
                let fn_found = self.check_keyword(e.data.name, false);

                if fn_found.found {
                    if let NameCheckResponseType::Function(fn_data) = fn_found.found_type {
                        Ok(fn_data.data.return_type.raw_name())
                    } else if let NameCheckResponseType::Variable(v_data) = fn_found.found_type {
                        if let definers::DefinerCollecting::Function(fn_type) = v_data.data.rtype {
                            Ok(fn_type.returning.raw_name())
                        } else {
                            Ok("nen".to_owned())
                        }
                    } else {
                        Ok("nen".to_owned())
                    }
                } else {
                    Ok("nen".to_owned())
                }
            }
            types::Types::VariableType(e) => {
                let vr_found = self.check_keyword(e.data.value, false);

                if vr_found.found {
                    if let NameCheckResponseType::Variable(v_data) = vr_found.found_type {
                        Ok(v_data.data.rtype.raw_name())
                    } else if let NameCheckResponseType::Function(_) = vr_found.found_type {
                        Ok("function".to_owned())
                    } else if let NameCheckResponseType::Class(_) = vr_found.found_type {
                        Ok("class".to_owned())
                    } else {
                        Ok("nen".to_owned())
                    }
                } else {
                    Ok("nen".to_owned())
                }
            }
            types::Types::Null => Ok("null".to_owned()),
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
    ) -> Option<Vec<ellie_core::error::Error>> {
        let mut errors = Vec::new();
        let deep_scan = self.resolve_deep_call(*reference_data_collector.data.reference.clone());
        if deep_scan == DeepCallResponse::NoElement {
            errors.push(error::Error {
                path: self.options.path.clone(),
                scope: "function_call_processor".to_owned(),
                debug_message: "replace_parser_915".to_owned(),
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
        } else {
            panic!("TODO");
        }

        /*
        let mut errors = Vec::new();
        let deep_scan = self.resolve_deep_call(*reference_data_collector.data.reference.clone());

        if !errors.is_empty() {
            Some(errors)
        } else {
            None
        }
        */
        None
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
                        self.clone().look_up_for_item(item, e.data.value.clone())
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
                                            debug_message: "19ea33c3bda3028a9591234081b2b35d"
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
                    debug_message: "5206de755371342f1106d7811596f3fb".to_owned(),
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
                debug_message: "9e23fc07c12b35a73080780b55613e4c".to_owned(),
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
    ) -> Option<definers::DefinerCollecting> {
        let mut found = false;
        let mut found_type = definers::DefinerCollecting::Dynamic;
        match item {
            Collecting::Variable(e) => {
                if e.data.name == caller_data.data.name {
                    if let definers::DefinerCollecting::Function(fn_type) = e.data.rtype {
                        found_type = *fn_type.returning;
                        found = true;
                        if caller_data.data.params.len() != fn_type.params.len() {
                            errors.push(error::Error {
                                path: self.options.path.clone(),
                                scope: self.scope.scope_name.clone(),
                                debug_message: "e19cd0910a22ef136c9fde4dee3660a6".to_owned(),
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
                                            self.resolve_variable(caller_param.value);
                                        if let Ok(resolved_type) = resolved_type_option {
                                            if resolved_type != e.rtype {
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
                                                                value: resolved_type,
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
                                }
                            }
                        }
                    } else {
                        errors.push(error::Error {
                            path: self.options.path.clone(),
                            scope: self.scope.scope_name.clone(),
                            debug_message: "5d34bb80f6e42282ba3e7b7df6a600b8".to_owned(),
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
                if e.data.name == caller_data.data.name {
                    found_type = e.data.return_type;
                    found = true;
                    if caller_data.data.params.len() != e.data.parameters.len()
                        && (caller_data.data.params.len() > 0
                            && !e.data.parameters[e.data.parameters.len() - 1].multi_capture)
                    {
                        errors.push(error::Error {
                            path: self.options.path.clone(),
                            scope: self.scope.scope_name.clone(),
                            debug_message: "a6a12b65ef67371f40c21fe42d3d2db8".to_owned(),
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
                                        self.resolve_variable(caller_param.value);

                                    if let Ok(resolved_type) = resolved_type_option {
                                        if resolved_type != e.rtype {
                                            errors.push(error::Error {
                                                path: self.options.path.clone(),
                                                scope: self.scope.scope_name.clone(),
                                                debug_message: "c19a4b727ef44a45f9ca006b0ee45fe2"
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
                                                            value: resolved_type,
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
                                        self.resolve_variable(caller_param.value.clone());

                                    if let Ok(resolved_type) = resolved_type_option {
                                        if resolved_type == "nen"
                                            && caller_param.value.clone().get_type() == "variable"
                                        {
                                            errors.push(error::Error {
                                                path: self.options.path.clone(),
                                                scope: self.scope.scope_name.clone(),
                                                debug_message: "74e76688c642c18e7bd9339e60b65bef"
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
                            }
                        }
                    }
                }
            }
            Collecting::Class(e) => {
                if e.data.name == caller_data.data.name {
                    errors.push(error::Error {
                        path: self.options.path.clone(),
                        scope: self.scope.scope_name.clone(),
                        debug_message: "974221bc69b9607dcbf0b7a3b9a3bc43".to_owned(),
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
                if e.name == caller_data.data.name {
                    found_type = e.return_type;
                    found = true;
                    if caller_data.data.params.len() != e.parameters.len()
                        && (caller_data.data.params.len() > 0
                            && !e.parameters[e.parameters.len() - 1].multi_capture)
                    {
                        errors.push(error::Error {
                            path: self.options.path.clone(),
                            scope: self.scope.scope_name.clone(),
                            debug_message: "a6a12b65ef67371f40c21fe42d3d2db8".to_owned(),
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
                                        self.resolve_variable(caller_param.value);

                                    if let Ok(resolved_type) = resolved_type_option {
                                        if resolved_type != e.rtype && resolved_type != "nen" {
                                            errors.push(error::Error {
                                                path: self.options.path.clone(),
                                                scope: self.scope.scope_name.clone(),
                                                debug_message: "c19a4b727ef44a45f9ca006b0ee45fe2"
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
                                                            value: resolved_type,
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
                                        self.resolve_variable(caller_param.value.clone());

                                    if let Ok(resolved_type) = resolved_type_option {
                                        if resolved_type == "nen"
                                            && caller_param.value.clone().get_type() == "variable"
                                        {
                                            errors.push(error::Error {
                                                path: self.options.path.clone(),
                                                scope: self.scope.scope_name.clone(),
                                                debug_message: "74e76688c642c18e7bd9339e60b65bef"
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
                            }
                        }
                    }
                }
            }
            Collecting::ImportItem(e) => {
                match self.deep_function_call_resolver(caller_data.clone(), *e.item, errors) {
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
            match self.deep_function_call_resolver(caller_data.clone(), item, &mut errors) {
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
                debug_message: "a5adad2e8801c07747287c29b3ed1f15".to_owned(),
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
                    if class.data.name == name {
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
    ) -> (bool, Collecting) {
        let mut found = false;
        let mut found_item: Collecting = Collecting::None;
        match item.clone() {
            Collecting::Variable(e) => {
                if e.data.name == name {
                    found = true;
                    found_item = item;
                }
            }
            Collecting::Setter(e) => {
                if e.data.name == name {
                    found = true;
                    found_item = item;
                }
            }
            Collecting::Getter(e) => {
                if e.data.name == name {
                    found = true;
                    found_item = item;
                }
            }
            Collecting::Function(e) => {
                if e.data.name == name {
                    found = true;
                    found_item = item;
                }
            }
            Collecting::Class(e) => {
                if e.data.name == name {
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
                    let (is_found, found_item_target) = self.deep_check_keyword(*e.item, name, contain_private);
                    found = is_found;
                    found_item = found_item_target;
                }
            }
            _ => (),
        };
        (found, found_item)
    }

    pub fn check_keyword(&self, name: String, contain_private: bool) -> NameCheckResponse {
        let mut found = false;
        let mut found_item: Collecting = Collecting::None;

        for item in self.collected.clone() {
            let (is_found, found_item_target) =
                self.deep_check_keyword(item, name.clone(), contain_private);
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
