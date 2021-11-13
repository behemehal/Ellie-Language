use crate::processors::Processor;
use ellie_core::{defs, error, utils};

#[derive(Default, Clone, Debug)]
pub struct CloakType {
    pub entries: Vec<DefinerTypes>,
    pub at_comma: bool,
    pub child_cache: Box<DefinerCollector>,
    pub not_empty: bool,
}

#[derive(Default, Clone, Debug)]
pub struct ArrayType {
    pub rtype: Box<DefinerTypes>,
    pub size: usize,
    pub raw_size: String,
    pub at_comma: bool,
    pub type_collected: bool,
    pub child_cache: Box<DefinerCollector>,
}

#[derive(Default, Clone, Debug)]
pub struct CollectiveType {
    pub key: Box<DefinerTypes>,
    pub value: Box<DefinerTypes>,
    pub key_collected: bool,
    pub at_comma: bool,
    pub child_cache: Box<DefinerCollector>,
}

#[derive(Default, Clone, Debug)]
pub struct VectorType {
    pub rtype: Box<DefinerTypes>,
}

#[derive(Default, Clone, Debug)]
pub struct FutureType {
    pub rtype: Box<DefinerTypes>,
    pub child_cache: Box<DefinerCollector>,
}

#[derive(Default, Clone, Debug)]
pub struct NullableType {
    pub rtype: Box<DefinerTypes>,
    pub child_cache: Box<DefinerCollector>,
}

#[derive(Default, Clone, Debug)]
pub struct GenericType {
    pub rtype: String,
}

#[derive(Default, Clone, Debug)]
pub struct FunctionType {
    pub returning: Box<DefinerTypes>,
    pub params: Vec<DefinerTypes>,
    pub child_cache: Box<DefinerCollector>,
    pub brace_stared: bool,
    pub parameters_collected: bool,
    pub return_char_typed: bool,
    pub at_comma: bool,
    pub not_empty: bool,
}

#[derive(Clone, Debug)]
pub enum DefinerTypes {
    Cloak(CloakType),
    Array(ArrayType),
    Collective(CollectiveType),
    Vector(VectorType),
    Future(FutureType),
    Nullable(NullableType),
    Generic(GenericType),
    Function(FunctionType),
}

impl Default for DefinerTypes {
    fn default() -> Self {
        DefinerTypes::Generic(GenericType {
            rtype: String::new(),
        })
    }
}

#[derive(Default, Clone, Debug)]
pub struct DefinerCollector {
    pub definer_type: DefinerTypes,
    pub complete: bool,
}
