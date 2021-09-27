use crate::alloc::borrow::ToOwned;
use crate::alloc::string::ToString;
use crate::alloc::vec;
use crate::alloc::vec::Vec;
use crate::parser;
use alloc::boxed::Box;
use ellie_core::{defs, error};

pub fn collect_import<F>(
    parser: &mut parser::Parser<F>,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    _next_char: &str,
    _last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
{
    let parser_clone = parser.clone();
    if let parser::Collecting::Import(ref mut import_data) = parser.current {
        if letter_char != " " && letter_char != "\n" || import_data.path.is_empty() {
            import_data.pos.range_end = parser.pos;
            if letter_char == ";" {
                if import_data.native {
                    let import_resolve_chain_id = ellie_core::utils::generate_hash();
                    (parser.emit_message)(ellie_core::com::Message {
                        id: import_resolve_chain_id.clone(),
                        message_type: ellie_core::com::MessageType::ParserImportItem,
                        from: parser.options.path.clone(),
                        from_chain: None,
                        message_data: alloc::format!("{:?}", parser.pos.clone()),
                    });
                    let response = (parser.resolver)(
                        parser_clone.options.clone(),
                        import_data.path.clone(),
                        import_data.native,
                    );
                    import_data.resolution_id = response.resolution_id.clone();
                    import_data.id = response.id.clone();

                    if !response.found {
                        if response.resolve_error == "" {
                            errors.push(error::Error {
                                path: parser.options.path.clone(),
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "e666644e8cbc8dd6d53e68bab187f73e".to_owned(),
                                title: error::errorList::error_s28.title.clone(),
                                code: error::errorList::error_s28.code,
                                message: error::errorList::error_s28.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s28.message.clone(),
                                    vec![error::ErrorBuildField {
                                        key: "token".to_owned(),
                                        value: import_data.path.clone(),
                                    }],
                                ),
                                pos: import_data.path_pos,
                            });
                        } else {
                            errors.push(error::Error {
                                path: parser.options.path.clone(),
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "f22d55a3f3e435efcc20bd68f232bbe7".to_owned(),
                                title: error::errorList::error_s32.title.clone(),
                                code: error::errorList::error_s32.code,
                                message: error::errorList::error_s32.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s32.message.clone(),
                                    vec![error::ErrorBuildField {
                                        key: "token".to_owned(),
                                        value: response.resolve_error,
                                    }],
                                ),
                                pos: import_data.path_pos,
                            });
                        }
                    } else {
                        (parser.emit_message)(ellie_core::com::Message {
                            id: ellie_core::utils::generate_hash(),
                            message_type: ellie_core::com::MessageType::ParserImportItem,
                            from: parser.options.path.clone(),
                            from_chain: Some(import_resolve_chain_id),
                            message_data: alloc::format!("{:?}", parser.pos.clone()),
                        });

                        match response.file_content {
                            parser::ResolvedFileContent::PreBuilt(_) => todo!(),
                            parser::ResolvedFileContent::Raw(content) => {
                                let inner_parser = parser_clone
                                    .clone()
                                    .read_native_header(content, response.resolved_path);

                                if !inner_parser.syntax_errors.is_empty() {
                                    errors.extend(inner_parser.syntax_errors);
                                    errors.push(error::Error {
                                        path: parser.options.path.clone(),
                                        scope: parser.scope.scope_name.clone(),
                                        debug_message: "6d95aa7ab5c8be550f962815786edab3"
                                            .to_string(),
                                        title: error::errorList::error_s33.title.clone(),
                                        code: error::errorList::error_s33.code,
                                        message: error::errorList::error_s33.message.clone(),
                                        builded_message: error::Error::build(
                                            error::errorList::error_s33.message.clone(),
                                            vec![error::ErrorBuildField {
                                                key: "token".to_owned(),
                                                value: import_data.path.clone(),
                                            }],
                                        ),
                                        pos: import_data.path_pos,
                                    });
                                } else {
                                    for item in inner_parser.parsed.items {
                                        let parser_iter_clone = parser_clone.clone();
                                        match item.clone() {
                                            crate::parser::Collecting::ImportItem(e) => {
                                                parser.collected.push(item);
                                            }
                                            crate::parser::Collecting::Variable(e) => {
                                                parser.collected.push(
                                                    crate::parser::Collecting::ImportItem(
                                                        crate::syntax::import_item::ImportItem {
                                                            resolution_id: response
                                                                .resolution_id
                                                                .clone(),
                                                            from_import: response.id,
                                                            from_path: import_data.path.clone(),
                                                            item: Box::new(item),
                                                            public: e.data.public,
                                                        },
                                                    ),
                                                );
                                            }
                                            crate::parser::Collecting::Function(e) => {
                                                parser.collected.push(
                                                    crate::parser::Collecting::ImportItem(
                                                        crate::syntax::import_item::ImportItem {
                                                            resolution_id: response
                                                                .resolution_id
                                                                .clone(),
                                                            from_import: response.id,
                                                            from_path: import_data.path.clone(),
                                                            item: Box::new(item),
                                                            public: e.data.public,
                                                        },
                                                    ),
                                                );
                                            }
                                            crate::parser::Collecting::Class(e) => {
                                                parser.collected.push(
                                                    crate::parser::Collecting::ImportItem(
                                                        crate::syntax::import_item::ImportItem {
                                                            resolution_id: import_data
                                                                .resolution_id
                                                                .clone(),
                                                            from_import: response.id,
                                                            from_path: import_data.path.clone(),
                                                            item: Box::new(item),
                                                            public: e.data.public,
                                                        },
                                                    ),
                                                );
                                            }
                                            crate::parser::Collecting::NativeFunction(e) => {
                                                parser.collected.push(
                                                    crate::parser::Collecting::ImportItem(
                                                        crate::syntax::import_item::ImportItem {
                                                            resolution_id: import_data
                                                                .resolution_id
                                                                .clone(),
                                                            from_import: response.id,
                                                            from_path: import_data.path.clone(),
                                                            item: Box::new(item),
                                                            public: e.public,
                                                        },
                                                    ),
                                                );
                                            }
                                            _ => {
                                                parser.collected.push(
                                                    crate::parser::Collecting::ImportItem(
                                                        crate::syntax::import_item::ImportItem {
                                                            resolution_id: import_data
                                                                .resolution_id
                                                                .clone(),
                                                            from_import: response.id,
                                                            from_path: import_data.path.clone(),
                                                            item: Box::new(item),
                                                            public: import_data.public,
                                                        },
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    import_data.pos.range_end = parser.pos.clone().skip_char(1);
                    parser.collected.push(parser.current.clone());
                    parser.current = parser::Collecting::None;
                } else {
                    let import_resolve_chain_id = ellie_core::utils::generate_hash();
                    (parser.emit_message)(ellie_core::com::Message {
                        id: import_resolve_chain_id.clone(),
                        message_type: ellie_core::com::MessageType::ParserImportItem,
                        from: parser.options.path.clone(),
                        from_chain: None,
                        message_data: alloc::format!("{:?}", parser.pos.clone()),
                    });
                    let response = (parser.resolver)(
                        parser_clone.options.clone(),
                        import_data.path.clone(),
                        import_data.native,
                    );

                    if !response.found {
                        if response.resolve_error == "" {
                            errors.push(error::Error {
                                path: parser.options.path.clone(),
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "4fa7b5ca3e9ee2b6a3c6b2226aa73a68".to_owned(),
                                title: error::errorList::error_s28.title.clone(),
                                code: error::errorList::error_s28.code,
                                message: error::errorList::error_s28.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s28.message.clone(),
                                    vec![error::ErrorBuildField {
                                        key: "token".to_owned(),
                                        value: import_data.path.clone(),
                                    }],
                                ),
                                pos: import_data.path_pos,
                            });
                        } else {
                            errors.push(error::Error {
                                path: parser.options.path.clone(),
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "e410784f7de21d10cfe0f5335a96eb8f".to_owned(),
                                title: error::errorList::error_s32.title.clone(),
                                code: error::errorList::error_s32.code,
                                message: error::errorList::error_s32.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s32.message.clone(),
                                    vec![error::ErrorBuildField {
                                        key: "token".to_owned(),
                                        value: response.resolve_error,
                                    }],
                                ),
                                pos: import_data.path_pos,
                            });
                        }
                    } else {
                        (parser.emit_message)(ellie_core::com::Message {
                            id: ellie_core::utils::generate_hash(),
                            message_type: ellie_core::com::MessageType::ParserImportItem,
                            from: parser.options.path.clone(),
                            from_chain: Some(import_resolve_chain_id),
                            message_data: alloc::format!("{:?}", parser.pos.clone()),
                        });
                        match response.file_content {
                            parser::ResolvedFileContent::PreBuilt(_) => todo!(),
                            parser::ResolvedFileContent::Raw(content) => {
                                let inner_parser = parser_clone
                                    .clone()
                                    .read_module(content, response.resolved_path);

                                if !inner_parser.syntax_errors.is_empty() {
                                    errors.extend(inner_parser.syntax_errors);
                                    errors.push(error::Error {
                                        path: parser.options.path.clone(),
                                        scope: parser.scope.scope_name.clone(),
                                        debug_message: "0ca9053776bc5666870a42e2b41c3213"
                                            .to_string(),
                                        title: error::errorList::error_s33.title.clone(),
                                        code: error::errorList::error_s33.code,
                                        message: error::errorList::error_s33.message.clone(),
                                        builded_message: error::Error::build(
                                            error::errorList::error_s33.message.clone(),
                                            vec![error::ErrorBuildField {
                                                key: "token".to_owned(),
                                                value: import_data.path.clone(),
                                            }],
                                        ),
                                        pos: import_data.path_pos,
                                    });
                                } else {
                                    for item in inner_parser.parsed.items {
                                        let parser_iter_clone = parser_clone.clone();
                                        match item.clone() {
                                            crate::parser::Collecting::ImportItem(e) => {
                                                parser.collected.push(item);
                                            }
                                            crate::parser::Collecting::Variable(e) => {
                                                parser.collected.push(
                                                    crate::parser::Collecting::ImportItem(
                                                        crate::syntax::import_item::ImportItem {
                                                            resolution_id: response
                                                                .resolution_id
                                                                .clone(),
                                                            from_import: response.id,
                                                            from_path: import_data.path.clone(),
                                                            item: Box::new(item),
                                                            public: e.data.public,
                                                        },
                                                    ),
                                                );
                                            }
                                            crate::parser::Collecting::Function(e) => {
                                                parser.collected.push(
                                                    crate::parser::Collecting::ImportItem(
                                                        crate::syntax::import_item::ImportItem {
                                                            resolution_id: response
                                                                .resolution_id
                                                                .clone(),
                                                            from_import: response.id,
                                                            from_path: import_data.path.clone(),
                                                            item: Box::new(item),
                                                            public: e.data.public,
                                                        },
                                                    ),
                                                );
                                            }
                                            crate::parser::Collecting::Class(e) => {
                                                parser.collected.push(
                                                    crate::parser::Collecting::ImportItem(
                                                        crate::syntax::import_item::ImportItem {
                                                            resolution_id: response
                                                                .resolution_id
                                                                .clone(),
                                                            from_import: response.id,
                                                            from_path: import_data.path.clone(),
                                                            item: Box::new(item),
                                                            public: e.data.public,
                                                        },
                                                    ),
                                                );
                                            }
                                            _ => {
                                                parser.collected.push(
                                                    crate::parser::Collecting::ImportItem(
                                                        crate::syntax::import_item::ImportItem {
                                                            resolution_id: response
                                                                .resolution_id
                                                                .clone(),
                                                            from_import: response.id,
                                                            from_path: import_data.path.clone(),
                                                            item: Box::new(item),
                                                            public: import_data.public,
                                                        },
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    import_data.pos.range_end = parser.pos.clone().skip_char(1);
                    parser.collected.push(parser.current.clone());
                    parser.current = parser::Collecting::None;
                }
            } else if letter_char != " " || (letter_char == " " && !import_data.path.is_empty()) {
                if import_data.path.is_empty() {
                    import_data.path_pos.range_start = parser.pos;
                }
                import_data.path += letter_char;
                import_data.path_pos.range_end = parser.pos.skip_char(1);
            }
        } else if letter_char != " " {
            errors.push(error::Error {
                path: parser.options.path.clone(),
                scope: parser.scope.scope_name.clone(),
                debug_message: "e068fe053195a64f273e5a9e8181c8b3".to_owned(),
                title: error::errorList::error_s1.title.clone(),
                code: error::errorList::error_s1.code,
                message: error::errorList::error_s1.message.clone(),
                builded_message: error::Error::build(
                    error::errorList::error_s1.message.clone(),
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: letter_char.to_string(),
                    }],
                ),
                pos: defs::Cursor {
                    range_start: parser.pos,
                    range_end: parser.pos.clone().skip_char(1),
                },
            });
        }
    }
}
