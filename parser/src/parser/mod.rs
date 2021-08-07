use enum_as_inner::EnumAsInner;
use serde::{Deserialize, Serialize};

pub mod iterator;
pub mod scope;

use crate::alloc::boxed::Box;
use crate::alloc::string::{String, ToString};
use crate::alloc::vec;
use crate::alloc::vec::Vec;

use crate::syntax::{
    caller, class, condition, constructor, definers, file_key, for_loop, function, import,
    import_item, ret, types, variable,
};
use ellie_core::{defs, error, utils};

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
    Getter,
    Setter,
    NativeClass,
    NativeFunction,
    None,
}

impl Default for Collecting {
    fn default() -> Self {
        Collecting::None
    }
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
    pub fn to_parser(self, resolver: fn(String) -> ResolvedImport) -> Parser {
        Parser {
            scope: self.scope,
            resolver,
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

    pub fn to_no_resolver_parser(self) -> Parser {
        Parser {
            scope: self.scope,
            resolver: |_| ResolvedImport::default(),
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
    pub resolve_error: String,
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
                scope: "definer_processor".to_string(),
                debug_message: "f49561600f1dbc3e9f415e7566a68eaf".to_string(),
                title: error::errorList::error_s26.title.clone(),
                code: error::errorList::error_s26.code,
                message: error::errorList::error_s26.message.clone(),
                builded_message: error::BuildedError::build_from_string(
                    error::errorList::error_s26.message.clone(),
                ),
                pos: self.keyword_pos,
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

    pub fn is_iterable(&self, target: types::Types) -> bool {
        match target {
            types::Types::Integer(_) => true,
            types::Types::Float(_) => false,
            types::Types::Bool(_) => true,
            types::Types::String(_) => true,
            _ => todo!(),
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
                found_item = self.look_up_for_item(*c.item, target);
            }
            _ => found_item = None,
        }

        found_item
    }

    pub fn resolve_deep_call(&self, target: types::Types) -> types::Types {
        match target {
            types::Types::Integer(_) => target,
            types::Types::Float(_) => target,
            types::Types::Bool(_) => target,
            types::Types::String(_) => target,
            types::Types::Char(_) => target,
            types::Types::Null => target,
            types::Types::Void => target,
            types::Types::Collective(_) => todo!(),
            types::Types::Array(_) => todo!(),
            types::Types::Cloak(_) => todo!(),
            types::Types::Reference(_) => todo!(),
            types::Types::BraceReference(_) => todo!(),
            types::Types::Operator(_) => todo!(),
            types::Types::ArrowFunction(_) => todo!(),
            types::Types::NewCall(_) => todo!(),
            types::Types::FunctionCall(_) => todo!(),
            types::Types::Negative(_) => todo!(),
            types::Types::VariableType(_) => todo!(),
        }
    }

    pub fn resolve_variable(&self, target: types::Types) -> String {
        match target {
            types::Types::Integer(_) => "int".to_string(),
            types::Types::Float(_) => "float".to_string(),
            types::Types::String(_) => "string".to_string(),
            types::Types::Char(_) => "char".to_string(),
            types::Types::Bool(_) => "bool".to_string(),
            types::Types::Negative(_) => "bool".to_string(),
            types::Types::Collective(_) => "collective".to_string(),
            types::Types::Cloak(_) => "cloak".to_string(),
            types::Types::Array(_) => "array".to_string(),
            types::Types::Void => "void".to_string(),
            types::Types::Reference(_) => {
                #[cfg(feature = "std")]
                std::println!("Not implemented for: types {:#?}", target);
                "".to_string()
            }
            types::Types::BraceReference(_) => {
                #[cfg(feature = "std")]
                std::println!("Not implemented for: types {:#?}", target);
                "".to_string()
            }
            types::Types::Operator(_) => {
                #[cfg(feature = "std")]
                std::println!("Not implemented for: types {:#?}", target);
                "".to_string()
            }

            types::Types::ArrowFunction(_) => "function".to_string(),
            types::Types::NewCall(e) => {
                if let Ok(resolved) = self.resolve_new_call(e) {
                    if let Collecting::Class(e) = resolved {
                        e.data.name
                    } else {
                        "nen".to_string()
                    }
                } else {
                    "nen".to_string()
                }
            }
            types::Types::FunctionCall(e) => {
                let fn_found = self.check_keyword(e.data.name);

                if fn_found.found {
                    if let NameCheckResponseType::Function(fn_data) = fn_found.found_type {
                        fn_data.data.return_type.raw_name()
                    } else if let NameCheckResponseType::Variable(v_data) = fn_found.found_type {
                        if let definers::DefinerCollecting::Function(fn_type) = v_data.data.rtype {
                            fn_type.returning.raw_name()
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
            types::Types::VariableType(e) => {
                let fn_found = self.check_keyword(e.data.value);

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
        }
    }

    pub fn resolve_reference_function_call(
        self,
        reference_data: types::reference_type::ReferenceType,
        caller_data: types::function_call::FunctionCallCollector,
    ) -> Option<Vec<ellie_core::error::Error>> {
        let found = false;
        let mut errors = Vec::new();

        let targeted_var = self.check_keyword(self.resolve_variable(*reference_data.reference));

        if !targeted_var.found {
            errors.push(error::Error {
                scope: self.scope.scope_name.clone(),
                debug_message: "308b8c43cf79f06bacd36ca7df72f97e".to_string(),
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

        //panic!("?? : {:#?} \n\n\n, {:#?}", reference_data.clone(), caller_data);

        if !found {
            errors.push(error::Error {
                scope: self.scope.scope_name.clone(),
                debug_message: "1b202878ad52316b13fdb826df01a5cc".to_string(),
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

    pub fn resolve_new_call(
        &self,
        caller_data: types::new_call::NewCallCollector,
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
                                if self.resolve_deep_call(c.data.value).get_type() != "class" {
                                    errors.push(error::Error {
                                        scope: "function_call_processor".to_string(),
                                        debug_message: "replace_79".to_string(),
                                        title: error::errorList::error_s31.title.clone(),
                                        code: error::errorList::error_s31.code,
                                        message: error::errorList::error_s31.message.clone(),
                                        builded_message: error::Error::build(
                                            error::errorList::error_s31.message.clone(),
                                            vec![error::ErrorBuildField {
                                                key: "token".to_string(),
                                                value: caller_data.raw_value.clone(),
                                            }],
                                        ),
                                        pos: caller_data.data.value_pos,
                                    });
                                } else {
                                    found_item = looked_up;
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
                    scope: "function_call_processor".to_string(),
                    debug_message: "replace_7d9".to_string(),
                    title: error::errorList::error_s31.title.clone(),
                    code: error::errorList::error_s31.code,
                    message: error::errorList::error_s31.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s31.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: caller_data.raw_value.clone(),
                        }],
                    ),
                    pos: caller_data.data.value_pos,
                });
            }
        }

        if !found {
            errors.push(error::Error {
                scope: self.scope.scope_name.clone(),
                debug_message: "replace_parser_540".to_string(),
                title: error::errorList::error_s6.title.clone(),
                code: error::errorList::error_s6.code,
                message: error::errorList::error_s6.message.clone(),
                builded_message: error::Error::build(
                    error::errorList::error_s6.message.clone(),
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
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
                        if let definers::DefinerCollecting::Function(fn_type) = e.data.rtype {
                            if caller_data.data.params.len() != fn_type.params.len() {
                                errors.push(error::Error {
                                    scope: self.scope.scope_name.clone(),
                                    debug_message: "3bc1a7957682f626543a90bc4634482c".to_string(),
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
                                                value: fn_type.params.len().to_string(),
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
                                    match fn_type.params[index].clone() {
                                        definers::DefinerCollecting::Array(_) => {
                                            panic!("Definer Resolving on 'Array' is not supported");
                                        }
                                        definers::DefinerCollecting::GrowableArray(_) => {
                                            panic!("Definer Resolving on 'GrowableArray' is not supported");
                                        }
                                        definers::DefinerCollecting::Nullable(_) => {
                                            panic!(
                                                "Definer Resolving on 'Nullable' is not supported"
                                            );
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
                                        definers::DefinerCollecting::Collective(_) => {
                                            panic!("Definer Resolving on 'Collective' is not supported");
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
                                debug_message: "103834decfc74a584c612db8db0029db".to_string(),
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
                                debug_message: "dc7d0d3714c9594f6758f5e56e474d29".to_string(),
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
                                match e.data.parameters[index].rtype.clone() {
                                    definers::DefinerCollecting::Array(_) => {
                                        panic!("Definer Resolving on 'Array' is not supported");
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
                                        let resolved_type =
                                            self.resolve_variable(caller_param.value);

                                        if resolved_type != e.rtype {
                                            errors.push(error::Error {
                                                scope: self.scope.scope_name.clone(),
                                                debug_message: "648271468e3a888e0c070fd7a5228cfc"
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
                                    definers::DefinerCollecting::Collective(_) => {
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
                            debug_message: "c3bde7f48c8baaf0ac3d8182895ecace".to_string(),
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
                debug_message: "8eab94a5d7dd30bd4b86df2c30b16b7d".to_string(),
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

    pub fn check_keyword(&self, name: String) -> NameCheckResponse {
        let mut found = false;
        let mut found_item: Collecting = Collecting::None;

        for item in self.collected.clone() {
            match item.clone() {
                Collecting::Variable(e) => {
                    if e.data.name == name {
                        found = true;
                        found_item = item;
                        break;
                    }
                }
                Collecting::Function(e) => {
                    if e.data.name == name {
                        found = true;
                        found_item = item;
                        break;
                    }
                }
                Collecting::Class(e) => {
                    if e.data.name == name {
                        found = true;
                        found_item = item;
                        break;
                    }
                }
                Collecting::FileKey(e) => {
                    if e.data.key_name == name {
                        found = true;
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
                found: found,
                found_type: NameCheckResponseType::None,
            },
        }
    }
}
