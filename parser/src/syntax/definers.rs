use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use enum_as_inner::EnumAsInner;
use serde::Deserialize;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct FunctionType {
    pub complete: bool,
    pub params: Vec<DefinerCollecting>,
    pub returning: Box<DefinerCollecting>,
    pub return_typed: bool,
    pub return_keyword: i8,
    pub parameter_collected: bool,
    pub bracket_inserted: bool,
    pub at_comma: bool,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct CloakType {
    pub complete: bool,
    pub rtype: Vec<DefinerCollecting>,
    pub bracket_inserted: bool,
    pub at_comma: bool,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArrayType {
    pub complete: bool,
    pub rtype: Box<DefinerCollecting>,
    pub bracket_inserted: bool,
    pub len: crate::syntax::types::integer_type::IntegerTypeCollector,
    pub at_comma: bool,
    pub typed: bool,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct GrowableArrayType {
    pub complete: bool,
    pub rtype: Box<DefinerCollecting>,
    pub bracket_inserted: bool,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct GenericType {
    pub rtype: String,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct CollectiveType {
    pub complete: bool,
    pub key: Box<DefinerCollecting>,
    pub value: Box<DefinerCollecting>,
    pub at_comma: bool,
    pub has_key: bool,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct NullableType {
    pub value: Box<DefinerCollecting>,
}

#[derive(PartialEq, Debug, Clone, Serialize, EnumAsInner, Deserialize)]
pub enum DefinerCollecting {
    Array(ArrayType),
    GrowableArray(GrowableArrayType),
    Generic(GenericType),
    Function(FunctionType),
    Cloak(CloakType),
    Collective(CollectiveType),
    Nullable(NullableType),
    Dynamic,
}

impl Default for DefinerCollecting {
    fn default() -> Self {
        DefinerCollecting::Generic(GenericType::default())
    }
}

impl DefinerCollecting {
    pub fn same_as(self, other: DefinerCollecting) -> bool {
        if self == other {
            match self {
                DefinerCollecting::Array(data) => {
                    if let DefinerCollecting::Array(other_data) = other {
                        other_data.len.raw == data.len.raw && other_data.rtype.same_as(*data.rtype)
                    } else {
                        false
                    }
                }
                DefinerCollecting::GrowableArray(data) => {
                    if let DefinerCollecting::GrowableArray(other_data) = other {
                        other_data.rtype.same_as(*data.rtype)
                    } else {
                        false
                    }
                }
                DefinerCollecting::Nullable(data) => {
                    if let DefinerCollecting::Nullable(other_data) = other {
                        other_data.value.same_as(*data.value)
                    } else {
                        false
                    }
                }
                DefinerCollecting::Generic(data) => {
                    if let DefinerCollecting::Generic(other_data) = other {
                        other_data.rtype == data.rtype
                    } else {
                        false
                    }
                }
                DefinerCollecting::Function(data) => {
                    if let DefinerCollecting::Function(other_data) = other {
                        if other_data.returning.same_as(*data.returning) {
                            let mut have_changes = false;

                            for i in 0..other_data.params.len() {
                                if !other_data.params[i].clone().same_as(data.params[i].clone()) {
                                    have_changes = true;
                                    break;
                                }
                            }

                            have_changes
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                DefinerCollecting::Cloak(data) => {
                    if let DefinerCollecting::Cloak(other_data) = other {
                        let mut have_changes = false;

                        for i in 0..other_data.rtype.len() {
                            if !other_data.rtype[i].clone().same_as(data.rtype[i].clone()) {
                                have_changes = true;
                                break;
                            }
                        }

                        have_changes
                    } else {
                        false
                    }
                }
                DefinerCollecting::Collective(data) => {
                    if let DefinerCollecting::Collective(other_data) = other {
                        other_data.key.same_as(*data.key) && other_data.value.same_as(*data.value)
                    } else {
                        false
                    }
                }
                DefinerCollecting::Dynamic => true,
            }
        } else {
            false
        }
    }

    pub fn is_type_empty(&self) -> bool {
        match self {
            DefinerCollecting::Array(data) => !data.complete,
            DefinerCollecting::GrowableArray(data) => !data.complete,
            DefinerCollecting::Nullable(data) => data.value.is_type_empty(),
            DefinerCollecting::Generic(data) => data.rtype.is_empty(),
            DefinerCollecting::Function(data) => !data.complete,
            DefinerCollecting::Cloak(data) => !data.complete,
            DefinerCollecting::Collective(data) => !data.complete,
            DefinerCollecting::Dynamic => false,
        }
    }

    pub fn is_definer_complete(&self) -> bool {
        match self {
            DefinerCollecting::Array(data) => data.complete,
            DefinerCollecting::GrowableArray(data) => data.complete,
            DefinerCollecting::Nullable(data) => data.value.is_definer_complete(),
            DefinerCollecting::Generic(data) => !data.rtype.is_empty(),
            DefinerCollecting::Function(data) => data.complete,
            DefinerCollecting::Cloak(data) => data.complete,
            DefinerCollecting::Collective(data) => data.complete,
            DefinerCollecting::Dynamic => true,
        }
    }

    pub fn is_generic(&self) -> bool {
        match self {
            DefinerCollecting::Array(_) => false,
            DefinerCollecting::GrowableArray(_) => false,
            DefinerCollecting::Nullable(_) => false,
            DefinerCollecting::Generic(_) => true,
            DefinerCollecting::Function(_) => false,
            DefinerCollecting::Cloak(_) => false,
            DefinerCollecting::Collective(_) => false,
            DefinerCollecting::Dynamic => true,
        }
    }

    pub fn raw_name(&self) -> String {
        match self {
            DefinerCollecting::Array(_) => "array".to_string(),
            DefinerCollecting::GrowableArray(_) => "dynamicArray".to_string(),
            DefinerCollecting::Nullable(_) => "nullAble".to_string(),
            DefinerCollecting::Generic(data) => data.rtype.clone(),
            DefinerCollecting::Function(_) => "function".to_string(),
            DefinerCollecting::Cloak(_) => "cloak".to_string(),
            DefinerCollecting::Collective(_) => "collective".to_string(),
            DefinerCollecting::Dynamic => "dyn".to_string(),
        }
    }
}
