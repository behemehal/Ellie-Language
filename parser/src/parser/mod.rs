use enum_as_inner::EnumAsInner;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Serialize as Serd;

pub mod iterator;
pub mod scope;

use crate::alloc::boxed::Box;
use crate::alloc::string::{String, ToString};
use crate::alloc::vec;
use crate::alloc::vec::Vec;
use crate::syntax::{
    caller, class, condition, constructor, definers, forloop, function, import, import_item, ret,
    types, variable,
};
use ellie_core::{defs, error, utils};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Parsed {
    pub name: String,
    pub items: Vec<Collecting>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParserResponse {
    pub parsed: Parsed,
    pub syntax_errors: Vec<error::Error>,
}

#[derive(PartialEq, Debug, Clone, Serd)]
pub enum Collecting {
    ImportItem(import_item::ImportItem),
    Variable(variable::VariableCollector),
    Function(function::FunctionCollector),
    Forloop(forloop::ForloopCollector),
    Condition(condition::ConditionCollector),
    Class(class::ClassCollector),
    Ret(ret::Ret),
    Constructor(constructor::ConstructorCollector),
    Caller(caller::Caller),
    Import(import::Import),
    Getter,
    Setter,
    None,
}

#[derive(EnumAsInner)]
pub enum NameCheckResponseType {
    Variable(variable::VariableCollector),
    Function(function::FunctionCollector),
    Class(class::ClassCollector),
    None,
}

pub struct NameCheckResponse {
    pub found: bool,
    pub found_type: NameCheckResponseType,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Parser {
    pub scope: Box<scope::Scope>,
    pub resolver: fn(String) -> ResolvedImport,
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

impl Serialize for Parser {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Color", 3)?;
        state.serialize_field("scope", &self.scope)?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("options", &self.options)?;
        state.serialize_field("collected", &self.collected)?;
        state.serialize_field("generic_variables", &self.generic_variables)?;
        state.serialize_field("pos", &self.pos)?;
        state.serialize_field("on_comment", &self.on_comment)?;
        state.serialize_field("on_line_comment", &self.on_line_comment)?;
        state.serialize_field("ignore_line", &self.ignore_line)?;
        state.serialize_field("current", &self.current)?;
        state.serialize_field("keyword_pos", &self.keyword_pos)?;
        state.serialize_field("keyword_catch", &self.keyword_catch)?;
        state.serialize_field("keyword_cache", &self.keyword_cache)?;
        state.end()
    }
}

impl Default for Parser {
    fn default() -> Self {
        Parser {
            scope: Box::new(scope::Scope::default()),
            resolver: |_| ResolvedImport::default(),
            code: "".to_string(),
            options: defs::ParserOptions::default(),
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
}

#[derive(Default)]
pub struct ResolvedImport {
    pub found: bool,
    pub file_content: Parsed,
}

impl Parser {
    pub fn new(
        code: String,
        resolve_import: fn(String) -> ResolvedImport,
        options: defs::ParserOptions,
    ) -> Self {
        Parser {
            scope: Box::new(scope::Scope::default()),
            resolver: resolve_import,
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

    pub fn read_module(mut self, code: String) -> ParserResponse {
        self.code = code;
        self.map()
    }

    pub fn map(mut self) -> ParserResponse {
        let mut errors: Vec<error::Error> = Vec::new();

        for (index, char) in self.code.clone().chars().enumerate() {
            let letter_char = &char.to_string();
            let last_char =
                &utils::get_letter(self.code.clone().to_string(), index, false).to_string();
            let next_char =
                &utils::get_letter(self.code.clone().to_string(), index, true).to_string();

            if char != '\n'
                && char != '\r'
                && char != '\t'
                && (letter_char != "/" || next_char != "/")
                && !self.on_line_comment
            {
                iterator::iter(
                    &mut self,
                    &mut errors,
                    letter_char,
                    next_char.to_string(),
                    last_char.to_string(),
                );
                self.pos.1 += 1;
            } else if letter_char == "/" || next_char == "/" {
                if !self.on_comment && !self.on_line_comment {
                    self.on_line_comment = true
                }
            } else if last_char == "\r" || letter_char == "\n" {
                self.pos.0 += 1;
                self.on_line_comment = false;
                self.pos.1 = 0;
            }
        }
        if self.current != Collecting::None || !self.keyword_catch.is_empty() {
            std::println!("{:#?}", self.current);
            errors.push(error::Error {
                scope: "definer_processor".to_string(),
                debug_message: "96910a324dae7459f3f1e063b3477aa7".to_string(),
                title: error::errorList::error_s26.title.clone(),
                code: error::errorList::error_s26.code,
                message: error::errorList::error_s26.message.clone(),
                builded_message: error::BuildedError::build_from_string(
                    error::errorList::error_s26.message.clone(),
                ),
                pos: defs::Cursor {
                    range_start: defs::CursorPosition::default(),
                    range_end: self.pos.skip_char(1),
                },
            });
        }
        ParserResponse {
            parsed: Parsed {
                name: self.scope.scope_name,
                items: self.collected.clone(),
            },
            syntax_errors: errors,
        }
    }

    pub fn resolve_variable(&self, target: types::Types) -> String {
        match target {
            types::Types::Integer(_) => "int".to_string(),
            types::Types::Float(_) => "float".to_string(),
            types::Types::String(_) => "string".to_string(),
            types::Types::Char(_) => "char".to_string(),
            types::Types::Collective => {
                #[cfg(feature = "std")]
                std::println!("Not implemented for: types {:#?}", target);
                "".to_string()
            }
            types::Types::Refference(_) => {
                #[cfg(feature = "std")]
                std::println!("Not implemented for: types {:#?}", target);
                "".to_string()
            }
            types::Types::Operator(_) => {
                #[cfg(feature = "std")]
                std::println!("Not implemented for: types {:#?}", target);
                "".to_string()
            }
            types::Types::Cloak(_) => {
                #[cfg(feature = "std")]
                std::println!("Not implemented for: types {:#?}", target);
                "".to_string()
            }
            types::Types::Array(_) => {
                #[cfg(feature = "std")]
                std::println!("Not implemented for: types {:#?}", target);
                "".to_string()
            }
            types::Types::ArrowFunction(_) => "function".to_string(),
            types::Types::ClassCall(_) => {
                #[cfg(feature = "std")]
                std::println!("Not implemented for: types {:#?}", target);
                "".to_string()
            }
            types::Types::FunctionCall(e) => {
                let fn_found = self.check_keyword(e.data.name);

                if fn_found.found {
                    if let NameCheckResponseType::Function(fn_data) = fn_found.found_type {
                        fn_data.data.return_type.raw_name()
                    } else if let NameCheckResponseType::Variable(v_data) = fn_found.found_type {
                        if let definers::DefinerCollecting::Function(fntype) = v_data.data.rtype {
                            fntype.returning.raw_name()
                        } else {
                            "nen".to_string()
                        }
                    } else {
                        "nen".to_string()
                    }
                } else {
                    "nen".to_string()
                }
            }
            types::Types::Void => "void".to_string(),
            types::Types::VariableType(e) => {
                let fn_found = self.check_keyword(e.value);

                if fn_found.found {
                    if let NameCheckResponseType::Variable(v_data) = fn_found.found_type {
                        v_data.data.rtype.raw_name()
                    } else if let NameCheckResponseType::Function(_) = fn_found.found_type {
                        "function".to_string()
                    } else if let NameCheckResponseType::Class(_) = fn_found.found_type {
                        "class".to_string()
                    } else {
                        "nen".to_string()
                    }
                } else {
                    "nen".to_string()
                }
            }
            types::Types::Null => "null".to_string(),
            _ => "".to_string(),
        }
    }

    pub fn resolve_refference_function_call(
        &self,
        refference_data: types::refference_type::RefferenceType,
        caller_data: types::function_call::FunctionCallCollector,
    ) -> Option<Vec<ellie_core::error::Error>> {
        let found = false;
        let mut errors = Vec::new();

        let targeted_var = self.check_keyword(self.resolve_variable(*refference_data.refference));

        if !targeted_var.found {
            errors.push(error::Error {
                scope: self.scope.scope_name.clone(),
                debug_message: "8bc8af3429bc3dc74b3344a70ff95264".to_string(),
                title: error::errorList::error_s6.title.clone(),
                code: error::errorList::error_s6.code,
                message: error::errorList::error_s6.message.clone(),
                builded_message: error::Error::build(
                    error::errorList::error_s6.message.clone(),
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: caller_data.data.name.clone(),
                    }],
                ),
                pos: caller_data.data.name_pos,
            });
        } else {
            //targeted_var.found_type
        }

        //panic!("?? : {:#?} \n\n\n, {:#?}", refference_data.clone(), caller_data);

        if !found {
            errors.push(error::Error {
                scope: self.scope.scope_name.clone(),
                debug_message: "14a11e35c7d5dcd3210eb70e60f5f188".to_string(),
                title: error::errorList::error_s6.title.clone(),
                code: error::errorList::error_s6.code,
                message: error::errorList::error_s6.message.clone(),
                builded_message: error::Error::build(
                    error::errorList::error_s6.message.clone(),
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: caller_data.data.name.clone(),
                    }],
                ),
                pos: caller_data.data.name_pos,
            });
        }

        if errors.is_empty() {
            None
        } else {
            Some(errors)
        }
    }

    pub fn resolve_function_call(
        &self,
        caller_data: types::function_call::FunctionCallCollector,
    ) -> Option<Vec<ellie_core::error::Error>> {
        let mut found = false;
        let mut errors = Vec::new();
        for item in self.collected.clone() {
            match item {
                Collecting::Variable(e) => {
                    if e.data.name == caller_data.data.name {
                        found = true;
                        if let definers::DefinerCollecting::Function(fntype) = e.data.rtype {
                            if caller_data.data.params.len() != fntype.params.len() {
                                errors.push(error::Error {
                                    scope: self.scope.scope_name.clone(),
                                    debug_message: "4ab7796f07cdb9d4bfe79637134525e7".to_string(),
                                    title: error::errorList::error_s7.title.clone(),
                                    code: error::errorList::error_s7.code,
                                    message: error::errorList::error_s7.message.clone(),
                                    builded_message: error::Error::build(
                                        error::errorList::error_s7.message.clone(),
                                        vec![
                                            error::ErrorBuildField {
                                                key: "name".to_string(),
                                                value: "Function".to_string(),
                                            },
                                            error::ErrorBuildField {
                                                key: "token".to_string(),
                                                value: fntype.params.len().to_string(),
                                            },
                                            error::ErrorBuildField {
                                                key: "token2".to_string(),
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
                                    match fntype.params[index].clone() {
                                        definers::DefinerCollecting::Array(_) => {
                                            panic!("Definer Resolving on 'Array' is not supported");
                                        }
                                        definers::DefinerCollecting::GrowableArray(_) => {
                                            panic!("Definer Resolving on 'GrowableArray' is not supported");
                                        }
                                        definers::DefinerCollecting::Generic(e) => {
                                            let resolved_type =
                                                self.resolve_variable(caller_param.value);
                                            if resolved_type != e.rtype {
                                                errors.push(error::Error {
                                                    scope: self.scope.scope_name.clone(),
                                                    debug_message:
                                                        "d4824ea474c0c2675d16029f0708f38f"
                                                            .to_string(),
                                                    title: error::errorList::error_s3.title.clone(),
                                                    code: error::errorList::error_s3.code,
                                                    message: error::errorList::error_s3
                                                        .message
                                                        .clone(),
                                                    builded_message: error::Error::build(
                                                        error::errorList::error_s3.message.clone(),
                                                        vec![
                                                            error::ErrorBuildField {
                                                                key: "token1".to_string(),
                                                                value: e.rtype,
                                                            },
                                                            error::ErrorBuildField {
                                                                key: "token2".to_string(),
                                                                value: resolved_type,
                                                            },
                                                        ],
                                                    ),
                                                    pos: caller_data.data.name_pos,
                                                });
                                            }
                                        }
                                        definers::DefinerCollecting::Function(_) => {
                                            panic!(
                                                "Definer Resolving on 'Function' is not supported"
                                            );
                                        }
                                        definers::DefinerCollecting::Cloak(_) => {
                                            panic!("Definer Resolving on 'Cloak' is not supported");
                                        }
                                        definers::DefinerCollecting::Dynamic => {
                                            panic!(
                                                "Definer Resolving on 'Dynamic' is not supported"
                                            );
                                        }
                                    }
                                }
                            }
                        } else {
                            errors.push(error::Error {
                                scope: self.scope.scope_name.clone(),
                                debug_message: "67eed67c3f984fc5913bb60f19fe826f".to_string(),
                                title: error::errorList::error_s25.title.clone(),
                                code: error::errorList::error_s25.code,
                                message: error::errorList::error_s25.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s25.message.clone(),
                                    vec![error::ErrorBuildField {
                                        key: "token".to_string(),
                                        value: caller_data.data.name.clone(),
                                    }],
                                ),
                                pos: caller_data.data.name_pos,
                            });
                        }
                        break;
                    }
                }
                Collecting::Function(e) => {
                    if e.data.name == caller_data.data.name {
                        found = true;
                        if caller_data.data.params.len() != e.data.parameters.len() {
                            errors.push(error::Error {
                                scope: self.scope.scope_name.clone(),
                                debug_message: "a3775631da7e38263f762e1bcf726c18".to_string(),
                                title: error::errorList::error_s7.title.clone(),
                                code: error::errorList::error_s7.code,
                                message: error::errorList::error_s7.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s7.message.clone(),
                                    vec![
                                        error::ErrorBuildField {
                                            key: "name".to_string(),
                                            value: "Function".to_string(),
                                        },
                                        error::ErrorBuildField {
                                            key: "token".to_string(),
                                            value: e.data.parameters.len().to_string(),
                                        },
                                        error::ErrorBuildField {
                                            key: "token2".to_string(),
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
                                match e.data.parameters[index].data.rtype.clone() {
                                    definers::DefinerCollecting::Array(_) => {
                                        panic!("Definer Resolving on 'Array' is not supported");
                                    }
                                    definers::DefinerCollecting::GrowableArray(_) => {
                                        panic!(
                                            "Definer Resolving on 'GrowableArray' is not supported"
                                        );
                                    }
                                    definers::DefinerCollecting::Generic(e) => {
                                        let resolved_type =
                                            self.resolve_variable(caller_param.value);

                                        if resolved_type != e.rtype {
                                            errors.push(error::Error {
                                                scope: self.scope.scope_name.clone(),
                                                debug_message: "885d700cf36e2e2594ce683d7fea49f5"
                                                    .to_string(),
                                                title: error::errorList::error_s3.title.clone(),
                                                code: error::errorList::error_s3.code,
                                                message: error::errorList::error_s3.message.clone(),
                                                builded_message: error::Error::build(
                                                    error::errorList::error_s3.message.clone(),
                                                    vec![
                                                        error::ErrorBuildField {
                                                            key: "token1".to_string(),
                                                            value: e.rtype,
                                                        },
                                                        error::ErrorBuildField {
                                                            key: "token2".to_string(),
                                                            value: resolved_type,
                                                        },
                                                    ],
                                                ),
                                                pos: caller_param.pos,
                                            });
                                        }
                                    }
                                    definers::DefinerCollecting::Function(_) => {
                                        panic!("Definer Resolving on 'Function' is not supported");
                                    }
                                    definers::DefinerCollecting::Cloak(_) => {
                                        panic!("Definer Resolving on 'Cloak' is not supported");
                                    }
                                    definers::DefinerCollecting::Dynamic => {
                                        panic!("Definer Resolving on 'Dynamic' is not supported");
                                    }
                                }
                            }
                        }
                    }
                }
                Collecting::Class(e) => {
                    if e.data.name == caller_data.data.name {
                        found = true;
                        errors.push(error::Error {
                            scope: self.scope.scope_name.clone(),
                            debug_message: "4e25530ff8ed09a8887ffaef460912ef".to_string(),
                            title: error::errorList::error_s25.title.clone(),
                            code: error::errorList::error_s25.code,
                            message: error::errorList::error_s25.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s25.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: caller_data.data.name.clone(),
                                }],
                            ),
                            pos: caller_data.data.name_pos,
                        });
                    }
                }
                _ => (),
            }
        }

        if !found {
            errors.push(error::Error {
                scope: self.scope.scope_name.clone(),
                debug_message: "fc2e9d0b2ba89054836612c7e6bcaf6a".to_string(),
                title: error::errorList::error_s6.title.clone(),
                code: error::errorList::error_s6.code,
                message: error::errorList::error_s6.message.clone(),
                builded_message: error::Error::build(
                    error::errorList::error_s6.message.clone(),
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: caller_data.data.name.clone(),
                    }],
                ),
                pos: caller_data.data.name_pos,
            });
        }

        if errors.is_empty() {
            None
        } else {
            Some(errors)
        }
    }

    pub fn resolve_class_call(
        &self,
        caller_data: types::class_call::ClassCallCollector,
    ) -> Option<Vec<ellie_core::error::Error>> {
        let mut found = false;
        let mut errors = Vec::new();
        for item in self.collected.clone() {
            match item {
                Collecting::Variable(e) => {
                    if e.data.name == caller_data.data.name {
                        found = true;
                        if let definers::DefinerCollecting::Function(fntype) = e.data.rtype {
                            if caller_data.data.params.len() != fntype.params.len() {
                                errors.push(error::Error {
                                    scope: self.scope.scope_name.clone(),
                                    debug_message: "b7dffa3f72d2257f1486e7ab12cf2e9e".to_string(),
                                    title: error::errorList::error_s7.title.clone(),
                                    code: error::errorList::error_s7.code,
                                    message: error::errorList::error_s7.message.clone(),
                                    builded_message: error::Error::build(
                                        error::errorList::error_s7.message.clone(),
                                        vec![
                                            error::ErrorBuildField {
                                                key: "name".to_string(),
                                                value: "Function".to_string(),
                                            },
                                            error::ErrorBuildField {
                                                key: "token".to_string(),
                                                value: fntype.params.len().to_string(),
                                            },
                                            error::ErrorBuildField {
                                                key: "token2".to_string(),
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
                                    match fntype.params[index].clone() {
                                        definers::DefinerCollecting::Array(_) => {
                                            panic!("Definer Resolving on 'Array' is not supported");
                                        }
                                        definers::DefinerCollecting::GrowableArray(_) => {
                                            panic!("Definer Resolving on 'GrowableArray' is not supported");
                                        }
                                        definers::DefinerCollecting::Generic(e) => {
                                            let resolved_type =
                                                self.resolve_variable(caller_param.value);
                                            if resolved_type != e.rtype {
                                                errors.push(error::Error {
                                                    scope: self.scope.scope_name.clone(),
                                                    debug_message:
                                                        "d4824ea474c0c2675d16029f0708f38f"
                                                            .to_string(),
                                                    title: error::errorList::error_s3.title.clone(),
                                                    code: error::errorList::error_s3.code,
                                                    message: error::errorList::error_s3
                                                        .message
                                                        .clone(),
                                                    builded_message: error::Error::build(
                                                        error::errorList::error_s3.message.clone(),
                                                        vec![
                                                            error::ErrorBuildField {
                                                                key: "token1".to_string(),
                                                                value: e.rtype,
                                                            },
                                                            error::ErrorBuildField {
                                                                key: "token2".to_string(),
                                                                value: resolved_type,
                                                            },
                                                        ],
                                                    ),
                                                    pos: caller_data.data.name_pos,
                                                });
                                            }
                                        }
                                        definers::DefinerCollecting::Function(_) => {
                                            panic!(
                                                "Definer Resolving on 'Function' is not supported"
                                            );
                                        }
                                        definers::DefinerCollecting::Cloak(_) => {
                                            panic!("Definer Resolving on 'Cloak' is not supported");
                                        }
                                        definers::DefinerCollecting::Dynamic => {
                                            panic!(
                                                "Definer Resolving on 'Dynamic' is not supported"
                                            );
                                        }
                                    }
                                }
                            }
                        } else {
                            errors.push(error::Error {
                                scope: self.scope.scope_name.clone(),
                                debug_message: "1acc7faa536befc32845f0d7024568d2".to_string(),
                                title: error::errorList::error_s25.title.clone(),
                                code: error::errorList::error_s25.code,
                                message: error::errorList::error_s25.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s25.message.clone(),
                                    vec![error::ErrorBuildField {
                                        key: "token".to_string(),
                                        value: caller_data.data.name.clone(),
                                    }],
                                ),
                                pos: caller_data.data.name_pos,
                            });
                        }
                        break;
                    }
                }
                Collecting::Function(e) => {
                    if e.data.name == caller_data.data.name {
                        found = true;
                        if caller_data.data.params.len() != e.data.parameters.len() {
                            errors.push(error::Error {
                                scope: self.scope.scope_name.clone(),
                                debug_message: "a0a1f254aab3c3b3b8f48bd587cb2bf3".to_string(),
                                title: error::errorList::error_s7.title.clone(),
                                code: error::errorList::error_s7.code,
                                message: error::errorList::error_s7.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s7.message.clone(),
                                    vec![
                                        error::ErrorBuildField {
                                            key: "name".to_string(),
                                            value: "Function".to_string(),
                                        },
                                        error::ErrorBuildField {
                                            key: "token".to_string(),
                                            value: e.data.parameters.len().to_string(),
                                        },
                                        error::ErrorBuildField {
                                            key: "token2".to_string(),
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
                                match e.data.parameters[index].data.rtype.clone() {
                                    definers::DefinerCollecting::Array(_) => {
                                        panic!("Definer Resolving on 'Array' is not supported");
                                    }
                                    definers::DefinerCollecting::GrowableArray(_) => {
                                        panic!(
                                            "Definer Resolving on 'GrowableArray' is not supported"
                                        );
                                    }
                                    definers::DefinerCollecting::Generic(e) => {
                                        let resolved_type =
                                            self.resolve_variable(caller_param.value);

                                        if resolved_type != e.rtype {
                                            errors.push(error::Error {
                                                scope: self.scope.scope_name.clone(),
                                                debug_message: "66bbdb187f3adc2e7ff741c9b1e35bd3"
                                                    .to_string(),
                                                title: error::errorList::error_s3.title.clone(),
                                                code: error::errorList::error_s3.code,
                                                message: error::errorList::error_s3.message.clone(),
                                                builded_message: error::Error::build(
                                                    error::errorList::error_s3.message.clone(),
                                                    vec![
                                                        error::ErrorBuildField {
                                                            key: "token1".to_string(),
                                                            value: e.rtype,
                                                        },
                                                        error::ErrorBuildField {
                                                            key: "token2".to_string(),
                                                            value: resolved_type,
                                                        },
                                                    ],
                                                ),
                                                pos: caller_param.pos,
                                            });
                                        }
                                    }
                                    definers::DefinerCollecting::Function(_) => {
                                        panic!("Definer Resolving on 'Function' is not supported");
                                    }
                                    definers::DefinerCollecting::Cloak(_) => {
                                        panic!("Definer Resolving on 'Cloak' is not supported");
                                    }
                                    definers::DefinerCollecting::Dynamic => {
                                        panic!("Definer Resolving on 'Dynamic' is not supported");
                                    }
                                }
                            }
                        }
                    }
                }
                _ => (),
            }
        }

        if !found {
            errors.push(error::Error {
                scope: self.scope.scope_name.clone(),
                debug_message: "2406faaa88a575cf683fd7562f5910ce".to_string(),
                title: error::errorList::error_s6.title.clone(),
                code: error::errorList::error_s6.code,
                message: error::errorList::error_s6.message.clone(),
                builded_message: error::Error::build(
                    error::errorList::error_s6.message.clone(),
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: caller_data.data.name.clone(),
                    }],
                ),
                pos: caller_data.data.name_pos,
            });
        }

        if errors.is_empty() {
            None
        } else {
            Some(errors)
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
            std::println!("iştem: {:#?}", item);
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

    pub fn check_keyword(&self, name: String) -> NameCheckResponse {
        let mut found = false;
        let mut found_item: Collecting = Collecting::None;

        for item in self.collected.clone() {
            match item {
                Collecting::Variable(ref e) => {
                    if e.data.name == name {
                        found = e.data.name == name;
                        found_item = item;
                        break;
                    }
                }
                Collecting::Function(ref e) => {
                    if e.data.name == name {
                        found = e.data.name == name;
                        found_item = item;
                        break;
                    }
                }
                Collecting::Class(ref e) => {
                    if e.data.name == name {
                        found = e.data.name == name;
                        found_item = item;
                        break;
                    }
                }
                _ => (),
            }
        }

        match found_item {
            Collecting::Variable(e) => NameCheckResponse {
                found,
                found_type: NameCheckResponseType::Variable(e),
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
                found: false,
                found_type: NameCheckResponseType::None,
            },
        }
    }
}
