pub mod arithmetic_type;
pub mod array_type;
pub mod arrow_function;
pub mod assignment_type;
pub mod bool_type;
pub mod bracket_reference_type;
pub mod char_type;
pub mod cloak_type;
pub mod collective_type;
pub mod comparison_type;
pub mod constructed_class;
pub mod double_type;
pub mod float_type;
pub mod function_call;
pub mod integer_type;
pub mod logical_type;
pub mod negative_type;
pub mod null_resolver;
pub mod operator_type;
pub mod reference_type;
pub mod string_type;
pub mod variable_type;

use alloc::{borrow::ToOwned, boxed::Box, string::String, vec::Vec};
use ellie_core::definite;
use enum_as_inner::EnumAsInner;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, EnumAsInner, Deserialize)]
pub enum Types {
    Integer(integer_type::IntegerTypeCollector),
    Float(float_type::FloatTypeCollector),
    Bool(bool_type::BoolType),
    String(string_type::StringTypeCollector),
    Char(char_type::CharType),
    Collective(collective_type::CollectiveCollector),
    Reference(reference_type::ReferenceTypeCollector),
    BracketReference(bracket_reference_type::BracketReferenceCollector),
    Operator(operator_type::OperatorTypeCollector),
    Cloak(cloak_type::CloakTypeCollector),
    Array(array_type::ArrayTypeCollector),
    ArrowFunction(arrow_function::ArrowFunctionCollector),
    ConstructedClass(constructed_class::ConstructedClassCollector),
    FunctionCall(function_call::FunctionCallCollector),
    Void,
    NullResolver(null_resolver::NullResolver),
    Negative(negative_type::Negative),
    VariableType(variable_type::VariableTypeCollector),
    Null,
}

impl Types {
    pub fn to_definite(self) -> definite::types::Types {
        match self {
            Types::Integer(e) => definite::types::Types::Integer(e.to_definite()),
            Types::Float(e) => definite::types::Types::Float(e.to_definite()),
            Types::Bool(e) => definite::types::Types::Bool(e.to_definite()),
            Types::String(e) => definite::types::Types::String(e.to_definite()),
            Types::Char(e) => definite::types::Types::Char(e.to_definite()),
            Types::Collective(e) => definite::types::Types::Collective(e.to_definite()),
            Types::Reference(e) => definite::types::Types::Reference(e.to_definite()),
            Types::BracketReference(e) => definite::types::Types::BracketReference(e.to_definite()),
            Types::Operator(e) => definite::types::Types::Operator(e.to_definite()),
            Types::Cloak(e) => definite::types::Types::Cloak(e.to_definite()),
            Types::Array(e) => definite::types::Types::Array(e.to_definite()),
            Types::ArrowFunction(e) => definite::types::Types::ArrowFunction(e.to_definite()),
            Types::ConstructedClass(e) => definite::types::Types::ConstructedClass(e.to_definite()),
            Types::FunctionCall(e) => definite::types::Types::FunctionCall(e.to_definite()),
            Types::NullResolver(e) => definite::types::Types::NullResolver(e.to_definite()),
            Types::Negative(e) => definite::types::Types::Negative(e.to_definite()),
            Types::VariableType(e) => definite::types::Types::VariableType(e.to_definite()),
            Types::Void => definite::types::Types::Void,
            Types::Null => definite::types::Types::Null,
        }
    }

    pub fn from_definite(self, from: definite::types::Types) -> Self {
        match from {
            definite::types::Types::Integer(e) => {
                Types::Integer(integer_type::IntegerTypeCollector::default().from_definite(e))
            }
            definite::types::Types::Float(e) => {
                Types::Float(float_type::FloatTypeCollector::default().from_definite(e))
            }
            definite::types::Types::Bool(e) => {
                Types::Bool(bool_type::BoolType::default().from_definite(e))
            }
            definite::types::Types::String(e) => {
                Types::String(string_type::StringTypeCollector::default().from_definite(e))
            }
            definite::types::Types::Char(e) => {
                Types::Char(char_type::CharType::default().from_definite(e))
            }
            definite::types::Types::Collective(e) => {
                Types::Collective(collective_type::CollectiveCollector::default().from_definite(e))
            }
            definite::types::Types::Reference(e) => {
                Types::Reference(reference_type::ReferenceTypeCollector::default().from_definite(e))
            }
            definite::types::Types::BracketReference(e) => Types::BracketReference(
                bracket_reference_type::BracketReferenceCollector::default().from_definite(e),
            ),
            definite::types::Types::Operator(e) => {
                Types::Operator(operator_type::OperatorTypeCollector::default().from_definite(e))
            }
            definite::types::Types::Cloak(e) => {
                Types::Cloak(cloak_type::CloakTypeCollector::default().from_definite(e))
            }
            definite::types::Types::Array(e) => {
                Types::Array(array_type::ArrayTypeCollector::default().from_definite(e))
            }
            definite::types::Types::ArrowFunction(e) => Types::ArrowFunction(
                arrow_function::ArrowFunctionCollector::default().from_definite(e),
            ),
            definite::types::Types::ConstructedClass(e) => Types::ConstructedClass(
                constructed_class::ConstructedClassCollector::default().from_definite(e),
            ),
            definite::types::Types::FunctionCall(e) => Types::FunctionCall(
                function_call::FunctionCallCollector::default().from_definite(e),
            ),
            definite::types::Types::NullResolver(e) => {
                Types::NullResolver(null_resolver::NullResolver::default().from_definite(e))
            }
            definite::types::Types::Void => Types::Void,
            definite::types::Types::Negative(e) => {
                Types::Negative(negative_type::Negative::default().from_definite(e))
            }
            definite::types::Types::VariableType(e) => Types::VariableType(
                variable_type::VariableTypeCollector::default().from_definite(e),
            ),
            definite::types::Types::Null => Types::Null,
        }
    }

    pub fn to_definer(self) -> crate::syntax::definers::DefinerCollecting {
        match self {
            Types::Integer(_) => crate::syntax::definers::DefinerCollecting::Generic(
                crate::syntax::definers::GenericType {
                    rtype: "int".to_owned(),
                },
            ),
            Types::Float(_) => crate::syntax::definers::DefinerCollecting::Generic(
                crate::syntax::definers::GenericType {
                    rtype: "float".to_owned(),
                },
            ),
            Types::Bool(_) => crate::syntax::definers::DefinerCollecting::Generic(
                crate::syntax::definers::GenericType {
                    rtype: "bool".to_owned(),
                },
            ),
            Types::String(_) => crate::syntax::definers::DefinerCollecting::Generic(
                crate::syntax::definers::GenericType {
                    rtype: "string".to_owned(),
                },
            ),
            Types::Char(_) => crate::syntax::definers::DefinerCollecting::Generic(
                crate::syntax::definers::GenericType {
                    rtype: "char".to_owned(),
                },
            ),
            Types::Collective(e) => {
                let mut keys = e
                    .data
                    .entries
                    .clone()
                    .into_iter()
                    .map(|x| x.data.key.to_definer())
                    .collect::<Vec<_>>();
                keys.dedup();
                let mut values = e
                    .data
                    .entries
                    .clone()
                    .into_iter()
                    .map(|x| x.data.value.to_definer())
                    .collect::<Vec<_>>();
                values.dedup();

                let key_type = if keys.len() > 1 || keys.len() == 0 {
                    crate::syntax::definers::DefinerCollecting::Dynamic
                } else {
                    keys[0].clone()
                };

                let value_type = if values.len() > 1 || values.len() == 0 {
                    crate::syntax::definers::DefinerCollecting::Dynamic
                } else {
                    values[0].clone()
                };

                crate::syntax::definers::DefinerCollecting::Collective(
                    crate::syntax::definers::CollectiveType {
                        key: Box::new(key_type),
                        value: Box::new(value_type),
                        ..Default::default()
                    },
                )
            }
            Types::Reference(e) => e.last_entry,
            Types::BracketReference(_) => todo!(),
            Types::Operator(e) => match e.data.operator {
                operator_type::Operators::ComparisonType(_) => {
                    crate::syntax::definers::DefinerCollecting::Generic(
                        crate::syntax::definers::GenericType {
                            rtype: "bool".to_owned(),
                        },
                    )
                }
                operator_type::Operators::LogicalType(op) => match op {
                    logical_type::LogicalOperators::And => e.data.second.to_definer(),
                    logical_type::LogicalOperators::Or => e.data.first.to_definer(),
                    logical_type::LogicalOperators::Null => panic!("Unexpected parser behaviour"),
                },
                operator_type::Operators::ArithmeticType(_) => e.data.first.to_definer(),
                operator_type::Operators::AssignmentType(_) => e.data.first.to_definer(),
                operator_type::Operators::Null => panic!("Unexpected parser behaviour"),
            },
            Types::Cloak(e) => {
                let rtype = e
                    .data
                    .collective
                    .into_iter()
                    .map(|x| x.value.to_definer())
                    .collect::<Vec<_>>();
                crate::syntax::definers::DefinerCollecting::Cloak(
                    crate::syntax::definers::CloakType {
                        rtype,
                        ..Default::default()
                    },
                )
            }

            Types::Array(e) => {
                let mut array_values = e
                    .data
                    .collective
                    .clone()
                    .into_iter()
                    .map(|x| x.value.to_definer())
                    .collect::<Vec<_>>();
                array_values.dedup();

                let array_type = if array_values.len() > 1 || array_values.len() == 0 {
                    crate::syntax::definers::DefinerCollecting::Dynamic
                } else {
                    array_values[0].clone()
                };

                crate::syntax::definers::DefinerCollecting::Array(
                    crate::syntax::definers::ArrayType {
                        rtype: Box::new(array_type),
                        len: crate::syntax::types::integer_type::IntegerTypeCollector::build(
                            e.data.collective.len(),
                        ),
                        ..Default::default()
                    },
                )
            }
            Types::ArrowFunction(e) => crate::syntax::definers::DefinerCollecting::Function(
                crate::syntax::definers::FunctionType {
                    params: e
                        .data
                        .parameters
                        .into_iter()
                        .map(|x| x.rtype)
                        .collect::<Vec<_>>(),
                    returning: Box::new(e.data.return_type),
                    ..Default::default()
                },
            ),
            Types::ConstructedClass(e) => crate::syntax::definers::DefinerCollecting::Generic(
                crate::syntax::definers::GenericType {
                    rtype: e.data.class_name(),
                },
            ),
            Types::FunctionCall(e) => e.return_type,
            Types::Void => crate::syntax::definers::DefinerCollecting::Generic(
                crate::syntax::definers::GenericType {
                    rtype: "void".to_owned(),
                },
            ),
            Types::NullResolver(e) => e.value.to_definer(),
            Types::Negative(_) => crate::syntax::definers::DefinerCollecting::Generic(
                crate::syntax::definers::GenericType {
                    rtype: "bool".to_owned(),
                },
            ),
            Types::VariableType(e) => crate::syntax::definers::DefinerCollecting::Generic(
                crate::syntax::definers::GenericType {
                    rtype: e.data.value,
                },
            ),
            Types::Null => crate::syntax::definers::DefinerCollecting::Generic(
                crate::syntax::definers::GenericType {
                    rtype: "null".to_owned(),
                },
            ),
        }
    }

    pub fn same_as_type(self, target: crate::syntax::definers::DefinerCollecting) -> bool {
        match self {
            Types::Integer(_) => {
                target.raw_name() == "generic" && target.as_generic().unwrap().rtype == "int"
            }
            Types::Float(_) => {
                target.raw_name() == "generic" && target.as_generic().unwrap().rtype == "float"
            }
            Types::Bool(_) => {
                target.raw_name() == "generic" && target.as_generic().unwrap().rtype == "bool"
            }
            Types::String(_) => {
                target.raw_name() == "generic" && target.as_generic().unwrap().rtype == "string"
            }
            Types::Char(_) => {
                target.raw_name() == "generic" && target.as_generic().unwrap().rtype == "char"
            }
            Types::Collective(_) => todo!(),
            Types::Reference(_) => todo!(),
            Types::BracketReference(_) => todo!(),
            Types::Operator(_) => todo!(),
            Types::Array(_) => todo!(),
            Types::Cloak(_) => todo!(),
            Types::ArrowFunction(_) => todo!(),
            Types::FunctionCall(_) => todo!(),
            Types::ConstructedClass(_) => todo!(), //e.data.value.same_as_type(target),
            Types::VariableType(_) => todo!(),
            Types::NullResolver(_) => todo!(),
            Types::Negative(_) => {
                target.raw_name() == "generic" && target.as_generic().unwrap().rtype == "bool"
            }
            Types::Void => {
                target.raw_name() == "generic" && target.as_generic().unwrap().rtype == "void"
            }
            Types::Null => {
                target.raw_name() == "generic" && target.as_generic().unwrap().rtype == "null"
            }
        }
    }

    pub fn get_type(&self) -> String {
        match &*self {
            Types::Integer(_) => "int".to_owned(),
            Types::Float(_) => "float".to_owned(),
            Types::Bool(_) => "bool".to_owned(),
            Types::String(_) => "string".to_owned(),
            Types::Char(_) => "char".to_owned(),
            Types::Collective(_) => "collective".to_owned(),
            Types::Reference(e) => e.last_entry.raw_name(),
            Types::BracketReference(e) => e.resolved.raw_name(),
            Types::Operator(_) => "operator".to_owned(),
            Types::Array(_) => "array".to_owned(),
            Types::Cloak(_) => "cloak".to_owned(),
            Types::ArrowFunction(_) => "arrowFunction".to_owned(),
            Types::FunctionCall(_) => "functionCall".to_owned(),
            Types::ConstructedClass(_) => "classCall".to_owned(),
            Types::VariableType(_) => "variable".to_owned(),
            Types::Negative(_) => "bool".to_owned(),
            Types::NullResolver(e) => e.value.clone().get_type(),
            Types::Void => "void".to_owned(),
            Types::Null => "null".to_owned(),
        }
    }

    pub fn is_type_complete(&self) -> bool {
        match self {
            Types::Integer(e) => e.complete,
            Types::Float(e) => e.complete,
            Types::Bool(_) => true,
            Types::String(data) => data.complete,
            Types::Char(data) => data.complete,
            Types::Collective(e) => e.complete,
            Types::Reference(data) => !data.on_dot && data.complete,
            Types::BracketReference(data) => data.complete,
            Types::Operator(e) => {
                e.first_filled
                    && e.data.operator != operator_type::Operators::Null
                    && (e.second_is_not_null && e.data.second.is_type_complete())
            }
            Types::Array(data) => data.complete,
            Types::Cloak(data) => data.complete,
            Types::ArrowFunction(data) => data.complete,
            Types::FunctionCall(data) => data.complete,
            Types::ConstructedClass(e) => e.complete,
            Types::NullResolver(_) => true,
            Types::VariableType(_) => true,
            Types::Negative(e) => e.value.is_type_complete(),
            Types::Void => false,
            Types::Null => true,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            Types::Integer(_) => false, //Always complete
            Types::Float(_) => false,   //Always complete
            Types::Bool(_) => false,
            Types::String(_) => false,
            Types::Char(_) => false,
            Types::Collective(_) => false,
            Types::Reference(_) => false,
            Types::BracketReference(_) => false,
            Types::Operator(_) => false,
            Types::Array(_) => true,
            Types::Cloak(_) => false,
            Types::ArrowFunction(_) => false,
            Types::FunctionCall(_) => false,
            Types::ConstructedClass(_) => false,
            Types::VariableType(_) => false,
            Types::NullResolver(e) => {
                e.value.get_type() == "array" || e.value.get_type() == "growableArray"
            }
            Types::Negative(_) => false,
            Types::Void => false,
            Types::Null => false,
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            Types::Integer(_) => true, //Always complete
            Types::Float(_) => false,  //Always complete
            Types::Bool(_) => false,
            Types::String(_) => false,
            Types::Char(_) => false,
            Types::Collective(_) => false,
            Types::Reference(e) => e.last_entry.raw_name() == "int",
            Types::BracketReference(e) => e.resolved.raw_name() == "int",
            Types::Operator(_) => false,
            Types::Array(_) => false,
            Types::Cloak(_) => false,
            Types::ArrowFunction(_) => false,
            Types::FunctionCall(_) => false,
            Types::ConstructedClass(_) => false,
            Types::VariableType(_) => false,
            Types::NullResolver(e) => e.value.get_type() == "int",
            Types::Negative(_) => false,
            Types::Void => false,
            Types::Null => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            Types::Integer(_) => false, //Always complete
            Types::Float(_) => true,    //Always complete
            Types::Bool(_) => false,
            Types::String(_) => false,
            Types::Char(_) => false,
            Types::Collective(_) => false,
            Types::Reference(e) => e.last_entry.raw_name() == "float",
            Types::BracketReference(e) => e.resolved.raw_name() == "float",
            Types::Operator(_) => false,
            Types::Array(_) => false,
            Types::Cloak(_) => false,
            Types::ArrowFunction(_) => false,
            Types::FunctionCall(_) => false,
            Types::ConstructedClass(_) => false,
            Types::VariableType(_) => false,
            Types::NullResolver(e) => e.value.get_type() == "float",
            Types::Negative(_) => false,
            Types::Void => false,
            Types::Null => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Types::Integer(_) => false, //Always complete
            Types::Float(_) => false,   //Always complete
            Types::Bool(_) => true,
            Types::String(_) => false,
            Types::Char(_) => false,
            Types::Collective(_) => false,
            Types::Reference(e) => e.last_entry.raw_name() == "bool",
            Types::BracketReference(e) => e.resolved.raw_name() == "bool",
            Types::Operator(_) => false,
            Types::Array(_) => false,
            Types::Cloak(_) => false,
            Types::ArrowFunction(_) => false,
            Types::FunctionCall(_) => false,
            Types::ConstructedClass(_) => false,
            Types::VariableType(_) => false,
            Types::NullResolver(e) => e.value.get_type() == "bool",
            Types::Negative(_) => false,
            Types::Void => false,
            Types::Null => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Types::Integer(_) => false, //Always complete
            Types::Float(_) => false,   //Always complete
            Types::Bool(_) => false,
            Types::String(_) => true,
            Types::Char(_) => false,
            Types::Collective(_) => false,
            Types::Reference(e) => e.last_entry.raw_name() == "string",
            Types::BracketReference(e) => e.resolved.raw_name() == "string",
            Types::Operator(_) => false,
            Types::Array(_) => false,
            Types::Cloak(_) => false,
            Types::ArrowFunction(_) => false,
            Types::FunctionCall(_) => false,
            Types::ConstructedClass(_) => false,
            Types::VariableType(_) => true,
            Types::NullResolver(e) => e.value.get_type() == "string",
            Types::Negative(_) => false,
            Types::Void => false,
            Types::Null => false,
        }
    }

    pub fn make_complete(&mut self) {
        match self {
            Types::Integer(e) => e.complete = true,
            Types::Float(e) => e.complete = true,
            Types::Bool(_) => (),
            Types::String(e) => e.complete = true,
            Types::Char(e) => e.complete = true,
            Types::Collective(_) => (),
            Types::Reference(_) => (),
            Types::BracketReference(_) => (),
            Types::Operator(_) => (),
            Types::Array(e) => e.complete = true,
            Types::Cloak(e) => e.complete = true,
            Types::ArrowFunction(e) => e.complete = true,
            Types::FunctionCall(_) => (),
            Types::ConstructedClass(_) => (),
            Types::VariableType(_) => (),
            Types::NullResolver(_) => (),
            Types::Negative(_) => (),
            Types::Void => (),
            Types::Null => (),
        };
    }
}

impl Default for Types {
    fn default() -> Self {
        Types::Null
    }
}
