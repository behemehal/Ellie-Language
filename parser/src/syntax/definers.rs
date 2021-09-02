use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use ellie_core::definite;
use enum_as_inner::EnumAsInner;
use serde::{Deserialize, Serialize};

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

impl FunctionType {
    pub fn to_definite(self) -> definite::definers::FunctionType {
        definite::definers::FunctionType {
            params: self.params.into_iter().map(|x| x.to_definite()).collect(),
            returning: Box::new(self.returning.to_definite()),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct CloakType {
    pub complete: bool,
    pub rtype: Vec<DefinerCollecting>,
    pub bracket_inserted: bool,
    pub at_comma: bool,
}

impl CloakType {
    pub fn to_definite(self) -> definite::definers::CloakType {
        definite::definers::CloakType {
            rtype: self.rtype.into_iter().map(|x| x.to_definite()).collect(),
        }
    }
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

impl ArrayType {
    pub fn to_definite(self) -> definite::definers::ArrayType {
        definite::definers::ArrayType {
            rtype: Box::new(self.rtype.to_definite()),
            len: self.len.to_definite(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct GrowableArrayType {
    pub complete: bool,
    pub rtype: Box<DefinerCollecting>,
    pub bracket_inserted: bool,
}

impl GrowableArrayType {
    pub fn to_definite(self) -> definite::definers::GrowableArrayType {
        definite::definers::GrowableArrayType {
            rtype: Box::new(self.rtype.to_definite()),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct GenericType {
    pub rtype: String,
}

impl GenericType {
    pub fn to_definite(self) -> definite::definers::GenericType {
        definite::definers::GenericType { rtype: self.rtype }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct CollectiveType {
    pub complete: bool,
    pub key: Box<DefinerCollecting>,
    pub value: Box<DefinerCollecting>,
    pub at_comma: bool,
    pub has_key: bool,
}

impl CollectiveType {
    pub fn to_definite(self) -> definite::definers::CollectiveType {
        definite::definers::CollectiveType {
            key: Box::new(self.key.to_definite()),
            value: Box::new(self.value.to_definite()),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct NullableType {
    pub value: Box<DefinerCollecting>,
}

impl NullableType {
    pub fn to_definite(self) -> definite::definers::NullableType {
        definite::definers::NullableType {
            value: Box::new(self.value.to_definite()),
        }
    }
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
    pub fn to_definite(self) -> definite::definers::DefinerCollecting {
        match self {
            DefinerCollecting::Array(e) => {
                definite::definers::DefinerCollecting::Array(e.to_definite())
            }
            DefinerCollecting::GrowableArray(e) => {
                definite::definers::DefinerCollecting::GrowableArray(e.to_definite())
            }
            DefinerCollecting::Generic(e) => {
                definite::definers::DefinerCollecting::Generic(e.to_definite())
            }
            DefinerCollecting::Function(e) => {
                definite::definers::DefinerCollecting::Function(e.to_definite())
            }
            DefinerCollecting::Cloak(e) => {
                definite::definers::DefinerCollecting::Cloak(e.to_definite())
            }
            DefinerCollecting::Collective(e) => {
                definite::definers::DefinerCollecting::Collective(e.to_definite())
            }
            DefinerCollecting::Nullable(e) => {
                definite::definers::DefinerCollecting::Nullable(e.to_definite())
            }
            DefinerCollecting::Dynamic => definite::definers::DefinerCollecting::Dynamic,
        }
    }

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

    pub fn is_dynamic(&self) -> bool {
        match self {
            DefinerCollecting::Dynamic => true,
            _ => false,
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

    pub fn raw_name_with_extensions(&self) -> String {
        match self {
            DefinerCollecting::Array(e) => {
                "array(".to_string()
                    + &e.len.raw.to_string()
                    + &",".to_string()
                    + &*e.rtype.raw_name_with_extensions()
                    + &")".to_string()
            }
            DefinerCollecting::GrowableArray(e) => {
                "dynamicArray(".to_string()
                    + &*e.rtype.raw_name_with_extensions().to_string()
                    + &")".to_string()
            }
            DefinerCollecting::Nullable(e) => {
                "_".to_string() + &*e.value.raw_name_with_extensions()
            }
            DefinerCollecting::Generic(data) => data.rtype.clone(),
            DefinerCollecting::Function(e) => {
                let mut params = String::new();
                for i in &e.params {
                    params += &format!("{},", i.raw_name_with_extensions().to_string()).to_string();
                }

                "fn(".to_string()
                    + &params
                    + &")::".to_string()
                    + &*e.returning.raw_name_with_extensions().to_string()
            }
            DefinerCollecting::Cloak(e) => {
                let mut params = String::new();
                for i in &e.rtype {
                    params += &format!("{},", i.raw_name_with_extensions()).to_string();
                }
                "fn(".to_string() + &params + &")".to_string()
            }
            DefinerCollecting::Collective(e) => format!(
                "collective({}, {})",
                e.key.raw_name_with_extensions(),
                e.value.raw_name_with_extensions()
            ),
            DefinerCollecting::Dynamic => "dyn".to_string(),
        }
    }
}
