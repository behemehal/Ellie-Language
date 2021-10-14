extern crate alloc;
pub mod com;
pub mod definite;
pub mod defs;
pub mod error;
use core::mem::size_of_val;
use ellie_parser::parser;
use libc::c_char;
use libc::strlen;

#[repr(C)]
pub struct ParserResponse {
    pub parsed: crate::definite::DefiniteParsed,
    pub syntax_errors: *mut crate::error::Error,
}

#[repr(C)]
pub enum ResolvedFileContent {
    PreBuilt(*mut ellie_core::definite::items::Collecting),
    Raw(*mut c_char),
}

#[repr(C)]
pub struct ResolvedImport {
    pub found: bool,
    pub resolve_error: *mut c_char,
    pub resolved_path: *mut c_char,
    pub resolution_id: u64,
    pub id: u64,
    pub file_content: ResolvedFileContent,
}

#[no_mangle]
pub extern "C" fn map(
    code: *mut c_char,
    resolve_import: extern "C" fn(crate::defs::ParserOptions, *mut c_char, bool) -> ResolvedImport,
    com: extern "C" fn(com::Message),
    options: defs::ParserOptions,
) -> ParserResponse {
    /*
        parser::Parser<
            impl FnMut(ellie_core::com::Message) + Clone + Sized,
            impl FnMut(ellie_core::defs::ParserOptions, String, bool) -> parser::ResolvedImport + Clone + Sized,
        >
    */
    unsafe {
        //-> Parser<impl FnMut(com::Message) + Clone + Sized>
        let raw_code = code as *mut u8;
        let raw_code_len = strlen(code);

        let path_len = strlen(options.path);
        let path = String::from_raw_parts(options.path as *mut u8, path_len, path_len);

        let line_ending_len = strlen(options.line_ending);
        let line_ending = String::from_raw_parts(
            options.line_ending as *mut u8,
            line_ending_len,
            line_ending_len,
        );

        let parser = parser::Parser::new(
            String::from_raw_parts(raw_code, raw_code_len, raw_code_len),
            move |a, b, c| {
                let c_options = defs::ParserOptions {
                    path: a.path.as_ptr() as *mut i8,
                    functions: a.functions,
                    break_on_error: a.break_on_error,
                    loops: a.loops,
                    enums: a.enums,
                    classes: a.classes,
                    getters: a.getters,
                    setters: a.setters,
                    conditions: a.conditions,
                    global_variables: a.global_variables,
                    line_ending: a.line_ending.as_ptr() as *mut i8,
                    dynamics: a.dynamics,
                    collectives: a.collectives,
                    variables: a.variables,
                    import_std: a.import_std,
                    constants: a.constants,
                    parser_type: match a.parser_type {
                        ellie_core::defs::ParserType::RawParser => defs::ParserType::RawParser,
                        ellie_core::defs::ParserType::ClassParser => defs::ParserType::ClassParser,
                        ellie_core::defs::ParserType::HeaderParser => {
                            defs::ParserType::HeaderParser
                        }
                    },
                    allow_import: a.allow_import,
                };

                let req = resolve_import(c_options, b.as_ptr() as *mut i8, c);

                let resolve_error_len = strlen(req.resolve_error);
                let resolve_error = String::from_raw_parts(
                    req.resolve_error as *mut u8,
                    resolve_error_len,
                    resolve_error_len,
                );
                let resolved_path_len = strlen(req.resolved_path);
                let resolved_path = String::from_raw_parts(
                    req.resolved_path as *mut u8,
                    resolved_path_len,
                    resolved_path_len,
                );

                ellie_parser::parser::ResolvedImport {
                    found: req.found,
                    resolve_error,
                    resolved_path,
                    resolution_id: req.resolution_id,
                    id: req.id,
                    file_content: match req.file_content {
                        ResolvedFileContent::PreBuilt(e) => {
                            let collected_len = size_of_val(&e);
                            ellie_parser::parser::ResolvedFileContent::PreBuilt(
                                Vec::from_raw_parts(e, collected_len, collected_len),
                            )
                        }
                        ResolvedFileContent::Raw(e) => {
                            let code_len = strlen(e);
                            ellie_parser::parser::ResolvedFileContent::Raw(String::from_raw_parts(
                                code_len as *mut u8,
                                code_len,
                                code_len,
                            ))
                        }
                    },
                }
            },
            |e| {
                com(com::Message {
                    id: e.id.as_ptr() as *mut i8,
                    message_type: match e.message_type {
                        ellie_core::com::MessageType::ParserLineExec => {
                            com::MessageType::ParserLineExec
                        }
                        ellie_core::com::MessageType::ParserImportItem => {
                            com::MessageType::ParserImportItem
                        }
                        ellie_core::com::MessageType::ParserImportNativeItem => {
                            com::MessageType::ParserImportNativeItem
                        }
                        ellie_core::com::MessageType::ParserVariableItem => {
                            com::MessageType::ParserVariableItem
                        }
                        ellie_core::com::MessageType::ParserFunctionItem => {
                            com::MessageType::ParserFunctionItem
                        }
                        ellie_core::com::MessageType::ParserClassItem => {
                            com::MessageType::ParserClassItem
                        }
                        ellie_core::com::MessageType::ParseComplete => {
                            com::MessageType::ParseComplete
                        }
                    },
                    from: e.from.as_ptr() as *mut i8,
                    from_chain: match e.from_chain {
                        Some(e) => com::ErrorChainOption::Some(e.as_ptr() as *mut i8),
                        None => com::ErrorChainOption::None,
                    },
                    message_data: e.message_data.as_ptr() as *mut i8,
                });
            },
            ellie_core::defs::ParserOptions {
                path,
                functions: options.functions,
                break_on_error: options.break_on_error,
                loops: options.loops,
                enums: options.enums,
                classes: options.classes,
                getters: options.getters,
                setters: options.setters,
                conditions: options.conditions,
                global_variables: options.global_variables,
                line_ending,
                dynamics: options.dynamics,
                collectives: options.collectives,
                variables: options.variables,
                import_std: options.import_std,
                constants: options.constants,
                parser_type: match options.parser_type {
                    defs::ParserType::RawParser => ellie_core::defs::ParserType::RawParser,
                    defs::ParserType::ClassParser => ellie_core::defs::ParserType::ClassParser,
                    defs::ParserType::HeaderParser => ellie_core::defs::ParserType::HeaderParser,
                },
                allow_import: options.allow_import,
            },
        );
        let response = parser.map();
        ParserResponse {
            parsed: definite::build_definite_parsed_from(response.parsed.to_definite()),
            syntax_errors: response
                .syntax_errors
                .into_iter()
                .map(|error| error::build_error_from(error))
                .collect::<Vec<_>>()
                .as_mut_ptr(),
        }
    }
}
