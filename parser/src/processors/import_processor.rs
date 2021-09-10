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

                    if !response.found {
                        if response.resolve_error == "" {
                            errors.push(error::Error {
                                path: parser.options.path.clone(),
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "259ae5e2c92723d0d3858811a949309d".to_owned(),
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
                                debug_message: "e53ee0d8151cc6f548e834bc82341a8e".to_owned(),
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
                                        debug_message: "ad07f5d56c1cc9a7b67bf89c20a0f202"
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
                                                if e.public {
                                                    if !parser_iter_clone
                                                        .clone()
                                                        .import_exists(&e.from_path)
                                                    {
                                                        parser.collected.push(item);
                                                    } else {
                                                        #[cfg(feature = "std")]
                                                        std::println!("\u{001b}[33m[ParserInfo]\u{001b}[0m: Ignore {:#?} from {}", e.from_path, parser.options.path);
                                                    }
                                                }
                                            }
                                            crate::parser::Collecting::Variable(e) => {
                                                if e.data.public {
                                                    parser
                                                        .collected
                                                        .push(crate::parser::Collecting::ImportItem(
                                                        crate::syntax::import_item::ImportItem {
                                                            from_path: import_data.path.clone(),
                                                            item: Box::new(item),
                                                            public: import_data.public,
                                                        },
                                                    ));
                                                }
                                            }
                                            crate::parser::Collecting::Function(e) => {
                                                if e.data.public {
                                                    parser
                                                        .collected
                                                        .push(crate::parser::Collecting::ImportItem(
                                                        crate::syntax::import_item::ImportItem {
                                                            from_path: import_data.path.clone(),
                                                            item: Box::new(item),
                                                            public: import_data.public,
                                                        },
                                                    ));
                                                }
                                            }
                                            crate::parser::Collecting::Class(e) => {
                                                if e.data.public {
                                                    parser
                                                        .collected
                                                        .push(crate::parser::Collecting::ImportItem(
                                                        crate::syntax::import_item::ImportItem {
                                                            from_path: import_data.path.clone(),
                                                            item: Box::new(item),
                                                            public: import_data.public,
                                                        },
                                                    ));
                                                }
                                            }
                                            _ => {
                                                parser.collected.push(
                                                    crate::parser::Collecting::ImportItem(
                                                        crate::syntax::import_item::ImportItem {
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
                                debug_message: "57f1dfe58286356c46ca5f503d101e56".to_owned(),
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
                                debug_message: "8748679d86b3441e9dae7051e0423adf".to_owned(),
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
                                        debug_message: "a46998b54db8a3287fe39075c7f2b6fb"
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
                                                if e.public {
                                                    if !parser_iter_clone
                                                        .clone()
                                                        .import_exists(&e.from_path)
                                                    {
                                                        parser.collected.push(item);
                                                    } else {
                                                        #[cfg(feature = "std")]
                                                        std::println!("\u{001b}[33m[ParserInfo]\u{001b}[0m: Ignore {:#?} from {}", e.from_path, parser.options.path);
                                                    }
                                                }
                                            }
                                            crate::parser::Collecting::Variable(e) => {
                                                if e.data.public {
                                                    parser
                                                        .collected
                                                        .push(crate::parser::Collecting::ImportItem(
                                                        crate::syntax::import_item::ImportItem {
                                                            from_path: import_data.path.clone(),
                                                            item: Box::new(item),
                                                            public: import_data.public,
                                                        },
                                                    ));
                                                }
                                            }
                                            crate::parser::Collecting::Function(e) => {
                                                if e.data.public {
                                                    parser
                                                        .collected
                                                        .push(crate::parser::Collecting::ImportItem(
                                                        crate::syntax::import_item::ImportItem {
                                                            from_path: import_data.path.clone(),
                                                            item: Box::new(item),
                                                            public: import_data.public,
                                                        },
                                                    ));
                                                }
                                            }
                                            crate::parser::Collecting::Class(e) => {
                                                if e.data.public {
                                                    parser
                                                        .collected
                                                        .push(crate::parser::Collecting::ImportItem(
                                                        crate::syntax::import_item::ImportItem {
                                                            from_path: import_data.path.clone(),
                                                            item: Box::new(item),
                                                            public: import_data.public,
                                                        },
                                                    ));
                                                }
                                            }
                                            _ => {
                                                parser.collected.push(
                                                    crate::parser::Collecting::ImportItem(
                                                        crate::syntax::import_item::ImportItem {
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
                debug_message: "a3f3525efba20f0a53164a3b3a7dbf7d".to_owned(),
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
