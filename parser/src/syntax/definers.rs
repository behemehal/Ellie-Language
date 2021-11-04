use alloc::borrow::ToOwned;
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

    pub fn from_definite(self, from: definite::definers::FunctionType) -> Self {
        FunctionType {
            complete: true,
            params: from
                .params
                .into_iter()
                .map(|x| DefinerCollecting::default().from_definite(x))
                .collect(),
            returning: Box::new(DefinerCollecting::default().from_definite(*from.returning)),
            ..Default::default()
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

    pub fn from_definite(self, from: definite::definers::CloakType) -> Self {
        CloakType {
            complete: true,
            rtype: from
                .rtype
                .into_iter()
                .map(|x| DefinerCollecting::default().from_definite(x))
                .collect(),
            bracket_inserted: true,
            at_comma: true,
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

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct FutureType {
    pub complete: bool,
    pub brace_started: bool,
    pub value: Box<DefinerCollecting>,
}

impl FutureType {
    pub fn to_definite(self) -> definite::definers::FutureType {
        definite::definers::FutureType {
            value: Box::new(self.value.to_definite()),
        }
    }

    pub fn from_definite(self, from: definite::definers::FutureType) -> Self {
        FutureType {
            complete: true,
            brace_started: true,
            value: Box::new(DefinerCollecting::default().from_definite(*from.value)),
        }
    }
}

impl ArrayType {
    pub fn to_definite(self) -> definite::definers::ArrayType {
        definite::definers::ArrayType {
            rtype: Box::new(self.rtype.to_definite()),
            len: self.len.to_definite(),
        }
    }

    pub fn from_definite(self, from: definite::definers::ArrayType) -> Self {
        ArrayType {
            complete: true,
            rtype: Box::new(DefinerCollecting::default().from_definite(*from.rtype)),
            len: crate::syntax::types::integer_type::IntegerTypeCollector::default()
                .from_definite(from.len),
            ..Default::default()
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

    pub fn from_definite(self, from: definite::definers::GrowableArrayType) -> Self {
        GrowableArrayType {
            complete: true,
            rtype: Box::new(DefinerCollecting::default().from_definite(*from.rtype)),
            bracket_inserted: true,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct GenericType {
    pub rtype: String,
    pub hash: String,
}

impl GenericType {
    pub fn to_definite(self) -> definite::definers::GenericType {
        definite::definers::GenericType {
            rtype: self.rtype,
            hash: self.hash,
        }
    }

    pub fn from_definite(self, from: definite::definers::GenericType) -> Self {
        GenericType {
            rtype: from.rtype,
            hash: from.hash,
        }
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

    pub fn from_definite(self, from: definite::definers::CollectiveType) -> Self {
        CollectiveType {
            complete: true,
            key: Box::new(DefinerCollecting::default().from_definite(*from.key)),
            value: Box::new(DefinerCollecting::default().from_definite(*from.value)),
            at_comma: false,
            has_key: false,
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

    pub fn from_definite(self, from: definite::definers::NullableType) -> Self {
        NullableType {
            value: Box::new(DefinerCollecting::default().from_definite(*from.value)),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, EnumAsInner, Deserialize)]
pub enum DefinerCollecting {
    Array(ArrayType),
    Future(FutureType),
    GrowableArray(GrowableArrayType),
    Generic(GenericType),
    Function(FunctionType),
    Cloak(CloakType),
    Collective(CollectiveType),
    Nullable(NullableType),
    Dynamic,
    Error(i8),
}

impl Default for DefinerCollecting {
    fn default() -> Self {
        DefinerCollecting::Generic(GenericType::default())
    }
}

impl DefinerCollecting {
    pub fn get_hash(self) -> String {
        match self {
            DefinerCollecting::Array(_) => "ellie_array_hash".to_owned(),
            DefinerCollecting::Future(_) => "ellie_future_hash".to_owned(),
            DefinerCollecting::GrowableArray(_) => todo!(),
            DefinerCollecting::Generic(e) => e.hash,
            DefinerCollecting::Function(_) => "ellie_function_hash".to_owned(),
            DefinerCollecting::Cloak(e) => {
                if e.rtype.len() == 1 {
                    e.rtype[0].clone().get_hash()
                } else {
                    "ellie_cloak_hash".to_owned()
                }
            }
            DefinerCollecting::Collective(_) => "ellie_collective_hash".to_owned(),
            DefinerCollecting::Nullable(_) => "ellie_nullAble_hash".to_owned(),
            DefinerCollecting::Dynamic => "ellie_dyn_hash".to_owned(),
            DefinerCollecting::Error(_) => "ellie_error_hash".to_owned(),
        }
    }

    pub fn to_definite(self) -> definite::definers::DefinerCollecting {
        match self {
            DefinerCollecting::Array(e) => {
                definite::definers::DefinerCollecting::Array(e.to_definite())
            }
            DefinerCollecting::Future(e) => {
                definite::definers::DefinerCollecting::Future(e.to_definite())
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
            DefinerCollecting::Error(i) => {
                definite::definers::DefinerCollecting::Generic(definite::definers::GenericType {
                    rtype: format!("ERR({})", i),
                    hash: "ellie_error_hash".to_owned(),
                })
            }
        }
    }

    pub fn from_definite(self, from: definite::definers::DefinerCollecting) -> Self {
        match from {
            definite::definers::DefinerCollecting::Array(e) => {
                DefinerCollecting::Array(ArrayType::default().from_definite(e))
            }
            definite::definers::DefinerCollecting::Future(e) => {
                DefinerCollecting::Future(FutureType::default().from_definite(e))
            }
            definite::definers::DefinerCollecting::GrowableArray(e) => {
                DefinerCollecting::GrowableArray(GrowableArrayType::default().from_definite(e))
            }
            definite::definers::DefinerCollecting::Generic(e) => {
                DefinerCollecting::Generic(GenericType::default().from_definite(e))
            }
            definite::definers::DefinerCollecting::Function(e) => {
                DefinerCollecting::Function(FunctionType::default().from_definite(e))
            }
            definite::definers::DefinerCollecting::Cloak(e) => {
                DefinerCollecting::Cloak(CloakType::default().from_definite(e))
            }
            definite::definers::DefinerCollecting::Collective(e) => {
                DefinerCollecting::Collective(CollectiveType::default().from_definite(e))
            }
            definite::definers::DefinerCollecting::Nullable(e) => {
                DefinerCollecting::Nullable(NullableType::default().from_definite(e))
            }
            definite::definers::DefinerCollecting::Dynamic => DefinerCollecting::Dynamic,
        }
    }

    pub fn same_as(self, other: DefinerCollecting) -> bool {
        match self {
            DefinerCollecting::Array(data) => {
                if let DefinerCollecting::Array(other_data) = other {
                    other_data.len.raw == data.len.raw && other_data.rtype.same_as(*data.rtype)
                } else {
                    false
                }
            }
            DefinerCollecting::Future(data) => {
                if let DefinerCollecting::Future(other_data) = other {
                    other_data.value.same_as(*data.value)
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
                    let other = other_data.rtype;
                    let curr = data.rtype;
                    let same = other == curr;
                    same
                } else {
                    false
                }
            }
            DefinerCollecting::Function(data) => {
                if let DefinerCollecting::Function(other_data) = other {
                    if other_data.returning.same_as(*data.returning) {
                        if other_data.params.len() == data.params.len() {
                            let mut have_changes = false;
                            for i in 0..other_data.params.len() {
                                if !other_data.params[i].clone().same_as(data.params[i].clone()) {
                                    have_changes = true;
                                    break;
                                }
                            }

                            !have_changes
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            DefinerCollecting::Cloak(data) => {
                if let DefinerCollecting::Cloak(other_data) = other {
                    if data.rtype.len() == other_data.rtype.len() {
                        let mut have_changes = false;
                        for i in 0..other_data.rtype.len() {
                            if !other_data.rtype[i].clone().same_as(data.rtype[i].clone()) {
                                have_changes = true;
                                break;
                            }
                        }

                        !have_changes
                    } else {
                        false
                    }
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
            DefinerCollecting::Error(_) => true,
        }
    }

    pub fn is_type_empty(&self) -> bool {
        match self {
            DefinerCollecting::Array(data) => !data.complete,
            DefinerCollecting::Future(data) => data.value.is_type_empty(),
            DefinerCollecting::GrowableArray(data) => !data.complete,
            DefinerCollecting::Nullable(data) => data.value.is_type_empty(),
            DefinerCollecting::Generic(data) => data.rtype.is_empty(),
            DefinerCollecting::Function(data) => !data.complete,
            DefinerCollecting::Cloak(data) => !data.complete,
            DefinerCollecting::Collective(data) => !data.complete,
            DefinerCollecting::Dynamic => false,
            _ => true,
        }
    }

    pub fn is_definer_complete(&self) -> bool {
        match self {
            DefinerCollecting::Array(data) => data.complete,
            DefinerCollecting::Future(data) => data.value.is_definer_complete(),
            DefinerCollecting::GrowableArray(data) => data.complete,
            DefinerCollecting::Nullable(data) => data.value.is_definer_complete(),
            DefinerCollecting::Generic(data) => !data.rtype.is_empty(),
            DefinerCollecting::Function(data) => data.complete,
            DefinerCollecting::Cloak(data) => data.complete,
            DefinerCollecting::Collective(data) => data.complete,
            DefinerCollecting::Dynamic => true,
            _ => true,
        }
    }

    pub fn is_generic(&self) -> bool {
        match self {
            DefinerCollecting::Generic(_) => true,
            _ => false,
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
            DefinerCollecting::Array(_) => "array".to_owned(),
            DefinerCollecting::Future(_) => "future".to_owned(),
            DefinerCollecting::GrowableArray(_) => "growableArray".to_owned(),
            DefinerCollecting::Nullable(_) => "nullAble".to_owned(),
            DefinerCollecting::Generic(data) => data.rtype.clone(),
            DefinerCollecting::Function(_) => "function".to_owned(),
            DefinerCollecting::Cloak(_) => "cloak".to_owned(),
            DefinerCollecting::Collective(_) => "collective".to_owned(),
            DefinerCollecting::Dynamic => "dyn".to_owned(),
            _ => "unknown".to_owned(),
        }
    }

    pub fn raw_name_with_extensions_with_hashes(&self) -> String {
        match self {
            DefinerCollecting::Array(e) => {
                "array(".to_owned()
                    + &e.len.raw.to_string()
                    + &",".to_owned()
                    + &*e.rtype.raw_name_with_extensions()
                    + &")".to_owned()
            }
            DefinerCollecting::Future(e) => {
                "future(".to_owned() + &*e.value.raw_name_with_extensions() + &")".to_owned()
            }
            DefinerCollecting::GrowableArray(e) => {
                "growableArray(".to_owned()
                    + &*e.rtype.raw_name_with_extensions().to_string()
                    + &")".to_owned()
            }
            DefinerCollecting::Nullable(e) => "_".to_owned() + &*e.value.raw_name_with_extensions(),
            DefinerCollecting::Generic(data) => format!("{}({})", data.rtype.clone(), data.hash),
            DefinerCollecting::Function(e) => {
                let mut params = String::new();
                for i in &e.params {
                    params += &format!("{},", i.raw_name_with_extensions().to_string()).to_string();
                }

                "fn(".to_owned()
                    + &params
                    + &")::".to_owned()
                    + &*e.returning.raw_name_with_extensions().to_string()
            }
            DefinerCollecting::Cloak(e) => {
                let mut params = String::new();
                for i in &e.rtype {
                    params += &format!("{},", i.raw_name_with_extensions()).to_string();
                }
                "cloak(".to_owned() + &params + &")".to_owned()
            }
            DefinerCollecting::Collective(e) => format!(
                "collective({}, {})",
                e.key.raw_name_with_extensions(),
                e.value.raw_name_with_extensions()
            ),
            DefinerCollecting::Dynamic => "dyn".to_owned(),
            DefinerCollecting::Error(i) => format!("unexpectedBehavior({})", i),
        }
    }

    pub fn raw_name_with_extensions(&self) -> String {
        match self {
            DefinerCollecting::Array(e) => {
                "array(".to_owned()
                    + &e.len.raw.to_string()
                    + &",".to_owned()
                    + &*e.rtype.raw_name_with_extensions()
                    + &")".to_owned()
            }
            DefinerCollecting::Future(e) => {
                "future(".to_owned() + &*e.value.raw_name_with_extensions() + &")".to_owned()
            }
            DefinerCollecting::GrowableArray(e) => {
                "growableArray(".to_owned()
                    + &*e.rtype.raw_name_with_extensions().to_string()
                    + &")".to_owned()
            }
            DefinerCollecting::Nullable(e) => "_".to_owned() + &*e.value.raw_name_with_extensions(),
            DefinerCollecting::Generic(data) => data.rtype.clone(),
            DefinerCollecting::Function(e) => {
                let mut params = String::new();
                for i in &e.params {
                    params += &format!("{},", i.raw_name_with_extensions().to_string()).to_string();
                }

                "fn(".to_owned()
                    + &params
                    + &")::".to_owned()
                    + &*e.returning.raw_name_with_extensions().to_string()
            }
            DefinerCollecting::Cloak(e) => {
                let mut params = String::new();
                for i in &e.rtype {
                    params += &format!("{},", i.raw_name_with_extensions()).to_string();
                }
                "cloak(".to_owned() + &params + &")".to_owned()
            }
            DefinerCollecting::Collective(e) => format!(
                "collective({}, {})",
                e.key.raw_name_with_extensions(),
                e.value.raw_name_with_extensions()
            ),
            DefinerCollecting::Dynamic => "dyn".to_owned(),
            DefinerCollecting::Error(i) => format!("unexpectedBehavior({})", i),
        }
    }
}
