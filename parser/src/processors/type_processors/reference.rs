use crate::parser;
use crate::processors::value_processor;
use crate::syntax::{definers, types, variable};
use alloc::{borrow::ToOwned, string::String, vec, vec::Vec};
use ellie_core::{defs, error};

fn resolve_chain<F>(
    parser: parser::Parser<F>,
    chain: types::reference_type::Chain,
    last_entry: definers::DefinerCollecting,
) -> Result<definers::DefinerCollecting, Vec<error::Error>>
where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
{
    let mut errors = Vec::new();
    let mut found = definers::DefinerCollecting::Dynamic;
    match last_entry.clone() {
        definers::DefinerCollecting::Array(_) => todo!(),
        definers::DefinerCollecting::Cloak(_) => todo!(),
        definers::DefinerCollecting::Future(_) => todo!(),
        definers::DefinerCollecting::Nullable(_) => todo!(),
        definers::DefinerCollecting::GrowableArray(_) => todo!(),
        definers::DefinerCollecting::Generic(e) => {
            fn resolve_reference_chain(
                target: parser::Collecting,
                chain: types::Types,
            ) -> Result<definers::DefinerCollecting, (String, i8)> {
                /*
                    Error codes:
                    0 => cannot used as refferencer,
                    1 => not exists as parameter
                */

                match target {
                    parser::Collecting::Variable(_) => todo!(),
                    parser::Collecting::Function(_) => todo!(),
                    parser::Collecting::Class(targeted_item) => {
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

                        match chain.clone() {
                            types::Types::VariableType(chain_variable) => {
                                let found_property = all_properties
                                    .into_iter()
                                    .find(|q| q.0 == chain_variable.data.value);
                                match found_property {
                                    Some(q) => Ok(q.1),
                                    None => Err((chain_variable.data.value, 1)),
                                }
                            }
                            e => Err((e.get_type(), 0)),
                        }
                    }
                    parser::Collecting::Caller(_) => todo!(),
                    parser::Collecting::Getter(_) => todo!(),
                    parser::Collecting::Setter(_) => todo!(),
                    parser::Collecting::NativeClass => todo!(),
                    parser::Collecting::Enum(_) => todo!(),
                    parser::Collecting::NativeFunction(_) => todo!(),
                    parser::Collecting::None => todo!(),
                    _ => panic!("Unexpected parser behaviour: {:?}", target),
                }
            }

            let found_generic = parser.check_keyword(e.rtype.clone(), true);
            if found_generic.found {
                match found_generic.found_type {
                    parser::NameCheckResponseType::Class(target) => {
                        let resolved_reference_chain = resolve_reference_chain(
                            parser::Collecting::Class(target.clone()), //Remove clone
                            chain.value.clone(),
                        );
                        match resolved_reference_chain {
                            Ok(e) => {
                                found = e;
                            }
                            Err(e) => {
                                found =
                                    definers::DefinerCollecting::Generic(definers::GenericType {
                                        rtype: "null".to_owned(),
                                    });
                                if e.1 == 0 {
                                    errors.push(error::Error {
                                        path: parser.options.path.clone(),
                                        scope: parser.scope.scope_name.clone(),
                                        debug_message: "ce95796e35010ebe0e46e4a8f05bde13"
                                            .to_owned(),
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
                                        path: parser.options.path.clone(),
                                        scope: parser.scope.scope_name.clone(),
                                        debug_message: "d694fee1d772992e7d3e20ce6333ab31"
                                            .to_owned(),
                                        title: error::errorList::error_s34.title.clone(),
                                        code: error::errorList::error_s34.code,
                                        message: error::errorList::error_s34.message.clone(),
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
                    parser::NameCheckResponseType::Variable(target) => {
                        found = target.data.value.to_definer();
                        /*
                        match target.data.value {
                            types::Types::Integer(_) => todo!(),
                            types::Types::Float(_) => todo!(),
                            types::Types::Bool(_) => todo!(),
                            types::Types::String(_) => todo!(),
                            types::Types::Char(_) => todo!(),
                            types::Types::Collective(_) => todo!(),
                            types::Types::Reference(_) => todo!(),
                            types::Types::Operator(_) => todo!(),
                            types::Types::Cloak(_) => todo!(),
                            types::Types::Array(_) => todo!(),
                            types::Types::ArrowFunction(_) => todo!(),
                            types::Types::ConstructedClass(_) => todo!(),
                            types::Types::FunctionCall(_) => todo!(),
                            types::Types::Void => todo!(),
                            types::Types::NullResolver(_) => todo!(),
                            types::Types::Negative(_) => todo!(),
                            types::Types::VariableType(_) => todo!(),
                            types::Types::Null => todo!(),
                        }
                        */
                    }
                    _ => panic!("Unexpected parser behaviour"),
                }
            } else {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "22f6c5cabea80aaffa81819df4325677".to_owned(),
                    title: error::errorList::error_s38.title.clone(),
                    code: error::errorList::error_s38.code,
                    message: error::errorList::error_s38.message.clone(),
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
    };

    if errors.len() > 0 {
        Err(errors)
    } else {
        Ok(found)
    }
}

pub fn collect_reference<F>(
    parser: parser::Parser<F>,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
{
    if let types::Types::Reference(ref mut reference_data) = itered_data.data.value {
        let last_entry = reference_data.data.chain.len();

        if letter_char == "."
            && !reference_data.on_dot
            && (last_entry == 0
                || reference_data.data.chain[last_entry - 1]
                    .value
                    .is_type_complete())
        {
            reference_data.on_dot = true;
            reference_data
                .data
                .chain
                .push(types::reference_type::Chain::default())
        } else {
            reference_data.on_dot = false;
            let mut will_be_itered = if last_entry == 0 {
                variable::VariableCollector {
                    ignore_existence: true,
                    ..Default::default()
                }
            } else {
                variable::VariableCollector {
                    data: variable::Variable {
                        value: reference_data.data.chain[last_entry - 1].value.clone(),
                        ..Default::default()
                    },
                    ignore_existence: true,
                    ..Default::default()
                }
            };

            value_processor::collect_value(
                parser.clone(),
                &mut will_be_itered,
                errors,
                letter_char,
                next_char,
                last_char,
            );

            if last_entry == 0 {
                reference_data
                    .data
                    .chain
                    .push(types::reference_type::Chain {
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos,
                        },
                        value: will_be_itered.data.value.clone(),
                    });
            } else {
                reference_data.data.chain[last_entry - 1].value = will_be_itered.data.value.clone();
                if reference_data.data.chain[last_entry - 1].pos.is_zero() {
                    reference_data.data.chain[last_entry - 1].pos.range_start = parser.pos;
                }
                reference_data.data.chain[last_entry - 1].pos.range_end =
                    parser.pos.clone().skip_char(1);
            }
            reference_data.complete = will_be_itered.data.value.is_type_complete();

            if reference_data.complete
                && (next_char == ";"
                    || next_char == ","
                    || next_char == "."
                    || next_char == " "
                    || next_char == ")"
                    || next_char == "]"
                    || next_char == "{"
                    || next_char == "}")
            {
                if last_entry == 1 {
                    //Resolve the referenced

                    match parser.resolve_deep_call(*reference_data.data.reference.clone()) {
                        parser::DeepCallResponse::TypeResponse(e) => {
                            reference_data.last_entry = e.to_definer();
                        }
                        parser::DeepCallResponse::ElementResponse(e) => match e {
                            parser::Collecting::ImportItem(_) => todo!(),
                            parser::Collecting::Variable(_) => todo!(),
                            parser::Collecting::Function(_) => todo!(),
                            parser::Collecting::ForLoop(_) => todo!(),
                            parser::Collecting::Condition(_) => todo!(),
                            parser::Collecting::Class(e) => {
                                reference_data.last_entry =
                                    definers::DefinerCollecting::Generic(definers::GenericType {
                                        rtype: e.data.name,
                                    })
                            }
                            parser::Collecting::Ret(_) => todo!(),
                            parser::Collecting::Constructor(_) => todo!(),
                            parser::Collecting::Caller(_) => todo!(),
                            parser::Collecting::Import(_) => todo!(),
                            parser::Collecting::FileKey(_) => todo!(),
                            parser::Collecting::Getter(_) => todo!(),
                            parser::Collecting::Setter(_) => todo!(),
                            parser::Collecting::NativeClass => todo!(),
                            parser::Collecting::ValueCall(_) => todo!(),
                            parser::Collecting::Enum(_) => todo!(),
                            parser::Collecting::NativeFunction(_) => todo!(),
                            parser::Collecting::None => todo!(),
                        },
                        parser::DeepCallResponse::NoElement => todo!(),
                    }
                }

                match resolve_chain(
                    parser.clone(),
                    reference_data.data.chain[last_entry - 1].clone(),
                    reference_data.last_entry.clone(),
                ) {
                    Ok(e) => {
                        reference_data.last_entry = e;
                    }
                    Err(e) => errors.extend(e),
                }
            }
        }
    }
}
