use crate::alloc::string::{String, ToString};
use crate::alloc::vec;
use crate::alloc::vec::Vec;
use crate::parser;
use alloc::boxed::Box;
use ellie_core::{defs, error};

pub fn collect_import(
    parser: &mut parser::Parser,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    _next_char: String,
    _last_char: String,
    _options: defs::ParserOptions,
) {
    if let parser::Collecting::Import(ref mut importdata) = parser.current {
        if letter_char != " " && letter_char != "\n" || importdata.path.is_empty() {
            importdata.pos.range_end = parser.pos;
            if letter_char == ";" {
                let response = (parser.resolver)(importdata.path.clone());
                if !response.found {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "8f7a6c658bc0f0f15d771f1b85d949d5".to_string(),
                        title: error::errorList::error_s28.title.clone(),
                        code: error::errorList::error_s28.code,
                        message: error::errorList::error_s28.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s28.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: importdata.path.clone(),
                            }],
                        ),
                        pos: importdata.path_pos,
                    });
                } else {
                    for item in response.file_content.items {
                        match item.clone() {
                            crate::parser::Collecting::ImportItem(e) => {
                                if e.public {
                                    parser.collected.push(item);
                                }
                            }
                            crate::parser::Collecting::Variable(e) => {
                                if e.data.public {
                                    parser.collected.push(crate::parser::Collecting::ImportItem(
                                        crate::syntax::import_item::ImportItem {
                                            item: Box::new(item),
                                            public: importdata.public,
                                        },
                                    ));
                                }
                            }
                            crate::parser::Collecting::Function(e) => {
                                if e.data.public {
                                    parser.collected.push(crate::parser::Collecting::ImportItem(
                                        crate::syntax::import_item::ImportItem {
                                            item: Box::new(item),
                                            public: importdata.public,
                                        },
                                    ));
                                }
                            }
                            crate::parser::Collecting::Class(e) => {
                                if e.data.public {
                                    parser.collected.push(crate::parser::Collecting::ImportItem(
                                        crate::syntax::import_item::ImportItem {
                                            item: Box::new(item),
                                            public: importdata.public,
                                        },
                                    ));
                                }
                            }
                            _ => {
                                parser.collected.push(crate::parser::Collecting::ImportItem(
                                    crate::syntax::import_item::ImportItem {
                                        item: Box::new(item),
                                        public: importdata.public,
                                    },
                                ));
                            }
                        }
                    }
                }
                parser.collected.push(parser.current.clone());
                parser.current = parser::Collecting::None;
            } else if letter_char != " " {
                if importdata.path.is_empty() {
                    importdata.path_pos.range_start = parser.pos;
                }
                importdata.path += letter_char;
                importdata.path_pos.range_end = parser.pos.skip_char(1);
            }
        } else if letter_char != " " {
            errors.push(error::Error {
                scope: parser.scope.scope_name.clone(),
                debug_message: "b1628292046d6dbd525b8aa097c0bdab".to_string(),
                title: error::errorList::error_s1.title.clone(),
                code: error::errorList::error_s1.code,
                message: error::errorList::error_s1.message.clone(),
                builded_message: error::Error::build(
                    error::errorList::error_s1.message.clone(),
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
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
