use crate::alloc::borrow::ToOwned;
use crate::alloc::string::{String, ToString};
use crate::alloc::vec;
use crate::alloc::vec::Vec;
use crate::parser;
use alloc::boxed::Box;
use ellie_core::{defs, error};

pub fn collect_import<F, E>(
    parser: &mut parser::Parser<F, E>,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    _next_char: &str,
    _last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
    E: FnMut(ellie_core::defs::ParserOptions, String, bool) -> parser::ResolvedImport
        + Clone
        + Sized,
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
                                debug_message: "8b4e4919952d16259090caf60605e668".to_owned(),
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
                                debug_message: "88e0c60a306000681938c6f6e39cee14".to_owned(),
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
                                        debug_message: "6967eb85f692aa87d35e26353937d89a"
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
                                        match item.clone() {
                                            crate::parser::Collecting::ImportItem(_) => {
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

                    import_data.resolution_id = response.resolution_id.clone();
                    import_data.id = response.id.clone();

                    if !response.found {
                        if response.resolve_error == "" {
                            errors.push(error::Error {
                                path: parser.options.path.clone(),
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "755d8570111b781d36503edae1cdbfd0".to_owned(),
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
                                debug_message: "b0c03d4e18d4f93b25be41defb3dcdae".to_owned(),
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
                                        debug_message: "acaf98c42468bfed782009e15041922f"
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
                                        match item.clone() {
                                            crate::parser::Collecting::ImportItem(_) => {
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
                debug_message: "325ddb713c2970216180ab2fec2381a0".to_owned(),
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
    } else {
        panic!("Unexpected parser behaviour")
    }
}
