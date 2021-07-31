use enum_as_inner::EnumAsInner;
use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserialize as Derd;
use serde::Serialize as Serd;

pub mod iterator;
pub mod scope;

use crate::alloc::boxed::Box;
use crate::alloc::string::{String, ToString};
use crate::alloc::vec;
use crate::alloc::vec::Vec;
use alloc::fmt;

use crate::syntax::{
    caller, class, condition, constructor, definers, file_key, forloop, function, import,
    import_item, ret, types, variable,
};
use ellie_core::{defs, error, utils};

#[derive(Debug, Clone, PartialEq, Default, Serd, Derd)]
pub struct Parsed {
    pub name: String,
    pub items: Vec<Collecting>,
}

#[derive(Debug, Clone, PartialEq, Serd, Derd)]
pub struct ParserResponse {
    pub parsed: Parsed,
    pub syntax_errors: Vec<error::Error>,
}

#[derive(PartialEq, Debug, Clone, Serd, Derd)]
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
    FileKey(file_key::FileKeyCollector),
    Getter,
    Setter,
    NativeClass,
    NativeFunction,
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

impl<'de> Deserialize<'de> for Parser {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Scope,
            Resolver,
            Code,
            Options,
            Collected,
            GenericVariables,
            Pos,
            OnComment,
            OnLineComment,
            IgnoreLine,
            Current,
            KeywordPos,
            KeywordCatch,
            KeywordCache,
        }

        // This part could also be generated independently by:
        //
        //    #[derive(Deserialize)]
        //    #[serde(field_identifier, rename_all = "lowercase")]
        //    enum Field { Secs, Nanos }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`secs` or `nanos`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "scope" => Ok(Field::Scope),
                            "resolver" => Ok(Field::Resolver),
                            "code" => Ok(Field::Code),
                            "options" => Ok(Field::Options),
                            "collected" => Ok(Field::Collected),
                            "generic_variables" => Ok(Field::GenericVariables),
                            "pos" => Ok(Field::Pos),
                            "on_comment" => Ok(Field::OnComment),
                            "on_line_comment" => Ok(Field::OnLineComment),
                            "ignore_line" => Ok(Field::IgnoreLine),
                            "current" => Ok(Field::Current),
                            "keyword_pos" => Ok(Field::KeywordPos),
                            "keyword_catch" => Ok(Field::KeywordCatch),
                            "keyword_cache" => Ok(Field::KeywordCache),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ParserVisitor;

        impl<'de> Visitor<'de> for ParserVisitor {
            type Value = Parser;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Parser")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Parser, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let scope = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let resolver = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let code = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let options = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                let collected = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(4, &self))?;
                let generic_variables = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(5, &self))?;
                let pos = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(6, &self))?;
                let on_comment = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(7, &self))?;
                let on_line_comment = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(8, &self))?;
                let ignore_line = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(9, &self))?;
                let current = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(10, &self))?;
                let keyword_pos = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(11, &self))?;
                let keyword_catch = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(12, &self))?;
                let keyword_cache = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(13, &self))?;

                Ok(Parser {
                    scope,
                    resolver,
                    code,
                    options,
                    collected,
                    generic_variables,
                    pos,
                    on_comment,
                    on_line_comment,
                    ignore_line,
                    current,
                    keyword_pos,
                    keyword_catch,
                    keyword_cache,
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Parser, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut scope = None;
                let mut resolver = None;
                let mut code = None;
                let mut options = None;
                let mut collected = None;
                let mut generic_variables = None;
                let mut pos = None;
                let mut on_comment = None;
                let mut on_line_comment = None;
                let mut ignore_line = None;
                let mut current = None;
                let mut keyword_pos = None;
                let mut keyword_catch = None;
                let mut keyword_cache = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Scope => {
                            if scope.is_some() {
                                return Err(de::Error::duplicate_field("scope"));
                            }
                            scope = Some(map.next_value()?);
                        }
                        Resolver => {
                            if resolver.is_some() {
                                return Err(de::Error::duplicate_field("resolver"));
                            }
                            resolver = Some(map.next_value()?);
                        }
                        Code => {
                            if code.is_some() {
                                return Err(de::Error::duplicate_field("code"));
                            }
                            code = Some(map.next_value()?);
                        }
                        Options => {
                            if options.is_some() {
                                return Err(de::Error::duplicate_field("options"));
                            }
                            options = Some(map.next_value()?);
                        }
                        Collected => {
                            if collected.is_some() {
                                return Err(de::Error::duplicate_field("collected"));
                            }
                            collected = Some(map.next_value()?);
                        }
                        GenericVariable => {
                            if generic_variables.is_some() {
                                return Err(de::Error::duplicate_field("generic_variables"));
                            }
                            generic_variables = Some(map.next_value()?);
                        }
                        Pos => {
                            if pos.is_some() {
                                return Err(de::Error::duplicate_field("pos"));
                            }
                            pos = Some(map.next_value()?);
                        }
                        OnComment => {
                            if on_comment.is_some() {
                                return Err(de::Error::duplicate_field("on_comment"));
                            }
                            on_comment = Some(map.next_value()?);
                        }
                        OnLineComment => {
                            if on_line_comment.is_some() {
                                return Err(de::Error::duplicate_field("on_line_comment"));
                            }
                            on_line_comment = Some(map.next_value()?);
                        }
                        IgnoreLine => {
                            if ignore_line.is_some() {
                                return Err(de::Error::duplicate_field("ignore_line"));
                            }
                            ignore_line = Some(map.next_value()?);
                        }
                        Current => {
                            if current.is_some() {
                                return Err(de::Error::duplicate_field("current"));
                            }
                            current = Some(map.next_value()?);
                        }
                        KeywordPos => {
                            if keyword_pos.is_some() {
                                return Err(de::Error::duplicate_field("keyword_pos"));
                            }
                            keyword_pos = Some(map.next_value()?);
                        }
                        KeywordCatch => {
                            if keyword_catch.is_some() {
                                return Err(de::Error::duplicate_field("keyword_catch"));
                            }
                            keyword_catch = Some(map.next_value()?);
                        }
                        KeywordCache => {
                            if keyword_cache.is_some() {
                                return Err(de::Error::duplicate_field("keyword_cache"));
                            }
                            keyword_cache = Some(map.next_value()?);
                        }
                    }
                }

                let scope = scope.ok_or_else(|| de::Error::missing_field("scope"))?;
                let resolver = resolver.ok_or_else(|| de::Error::missing_field("resolver"))?;
                let code = code.ok_or_else(|| de::Error::missing_field("code"))?;
                let options = options.ok_or_else(|| de::Error::missing_field("options"))?;
                let collected = collected.ok_or_else(|| de::Error::missing_field("collected"))?;
                let generic_variables = generic_variables
                    .ok_or_else(|| de::Error::missing_field("generic_variables"))?;
                let pos = pos.ok_or_else(|| de::Error::missing_field("pos"))?;
                let on_comment =
                    on_comment.ok_or_else(|| de::Error::missing_field("on_comment"))?;
                let on_line_comment =
                    on_line_comment.ok_or_else(|| de::Error::missing_field("on_line_comment"))?;
                let ignore_line =
                    ignore_line.ok_or_else(|| de::Error::missing_field("ignore_line"))?;
                let current = current.ok_or_else(|| de::Error::missing_field("current"))?;
                let keyword_pos =
                    keyword_pos.ok_or_else(|| de::Error::missing_field("keyword_pos"))?;
                let keyword_catch =
                    keyword_catch.ok_or_else(|| de::Error::missing_field("keyword_catch"))?;
                let keyword_cache =
                    keyword_cache.ok_or_else(|| de::Error::missing_field("keyword_cach"))?;
                Ok(Parser {
                    scope,
                    resolver,
                    code,
                    options,
                    collected,
                    generic_variables,
                    pos,
                    on_comment,
                    on_line_comment,
                    ignore_line,
                    current,
                    keyword_pos,
                    keyword_catch,
                    keyword_cache,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "scope",
            "resolver",
            "code",
            "options",
            "collected",
            "generic_variables",
            "pos",
            "on_comment",
            "on_line_comment",
            "ignore_line",
            "current",
            "keyword_pos",
            "keyword_catch",
            "keyword_cache",
        ];
        deserializer.deserialize_struct("Parser", FIELDS, ParserVisitor)
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
                debug_message: "05eb25bb1aa583dc68f52a1ebae1a3f3".to_string(),
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

    pub fn is_iterable(&self, target: types::Types) -> bool {
        match target {
            types::Types::Integer(_) => true,
            types::Types::Float(_) => false,
            types::Types::Bool(_) => true,
            types::Types::String(_) => true,
            _ => todo!(),
        }
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
            types::Types::ClassCall(_) => todo!(),
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
        }
    }

    pub fn resolve_reference_function_call(
        &self,
        reference_data: types::reference_type::ReferenceType,
        caller_data: types::function_call::FunctionCallCollector,
    ) -> Option<Vec<ellie_core::error::Error>> {
        let found = false;
        let mut errors = Vec::new();

        let targeted_var = self.check_keyword(self.resolve_variable(*reference_data.reference));

        if !targeted_var.found {
            errors.push(error::Error {
                scope: self.scope.scope_name.clone(),
                debug_message: "1a66ca3e07341170860da807397ca1fa".to_string(),
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
                debug_message: "6e168904ad0910b81a3c77b24b499083".to_string(),
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
                                    debug_message: "b3f3b6d516d47c72c2b0a91cb27989ac".to_string(),
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
                                debug_message: "bb5a7280e35506e7f98c4650a2a24493".to_string(),
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
                                debug_message: "38be2a2bb4c2a5c25e275cee84626b67".to_string(),
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
                                    definers::DefinerCollecting::Nullable(_) => {
                                        panic!("Definer Resolving on 'Nullable' is not supported");
                                    }
                                    definers::DefinerCollecting::Generic(e) => {
                                        let resolved_type =
                                            self.resolve_variable(caller_param.value);

                                        if resolved_type != e.rtype {
                                            errors.push(error::Error {
                                                scope: self.scope.scope_name.clone(),
                                                debug_message: "e35d13ab9be3da1ca1fb4f48e37e938b"
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
                            debug_message: "918731c4d5900ff364cdc7c09c1eea8b".to_string(),
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
                debug_message: "bc3b49a1e33d5216a4d5bd493ce8fe81".to_string(),
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
                                    debug_message: "473dc066a98feec68d03f27e79eca27e".to_string(),
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
                                debug_message: "cb70930db847c82274bbe6b1f30b6c5e".to_string(),
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
                                debug_message: "3a373e872fa16f53349c72f1df038a40".to_string(),
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
                                    definers::DefinerCollecting::Nullable(_) => {
                                        panic!("Definer Resolving on 'Nullable' is not supported");
                                    }
                                    definers::DefinerCollecting::Generic(e) => {
                                        let resolved_type =
                                            self.resolve_variable(caller_param.value);

                                        if resolved_type != e.rtype {
                                            errors.push(error::Error {
                                                scope: self.scope.scope_name.clone(),
                                                debug_message: "a7b1b8681c961237cc37c498cee33a69"
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
                    }
                }
                _ => (),
            }
        }

        if !found {
            errors.push(error::Error {
                scope: self.scope.scope_name.clone(),
                debug_message: "85ac9a6548f95f5caaf5fbddfe328581".to_string(),
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
