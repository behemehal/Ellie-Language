use crate::syntax::types;
use crate::mapper::defs;

#[derive(PartialEq, Eq, Default, Debug, Clone, Copy)]
pub struct NumberType {
    pub value: usize,
    pub complete: bool,
}

#[derive(PartialEq, Eq, Default, Debug, Clone)]
pub struct StringType {
    pub value: String,
    pub quote_type: String,
    pub complete: bool,
}

#[derive(PartialEq, Default, Debug, Clone, Copy)]
pub struct DoubleType {
    pub value: f32,
    pub complete: bool,
}

#[derive(PartialEq, Eq, Default, Debug, Clone, Copy)]
pub struct BoolType {
    pub value: bool,
}

#[derive(PartialEq, Default, Debug, Clone)]
pub struct CollectiveEntry {
    pub key: String,
    pub dynamic: bool,
    pub key_named: bool,
    pub r#type: String,
    pub typed: bool,
    pub value_complete: bool,
    pub raw_value: String,
    pub value: Box<types::Types>,
}

#[derive(PartialEq, Default, Debug, Clone)]
pub struct CollectiveType {
    pub layer_size: usize,
    pub collective: Vec<CollectiveEntry>,
}

#[derive(PartialEq, Default, Debug, Clone)]
pub struct ArrayEntry {
    pub value_complete: bool,
    pub value: Box<types::Types>,
}

#[derive(PartialEq, Default, Debug, Clone)]
pub struct ArrayType {
    pub layer_size: usize,
    pub complete: bool,
    pub comma: bool,
    pub child_start: bool,
    pub collective: Vec<ArrayEntry>,
}


#[derive(PartialEq, Default, Debug, Clone)]
pub struct FunctionCallParameter {
    pub value: types::Types,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Default, Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub name_pos: defs::Cursor,
    pub comma: bool,
    pub complete: bool,
    pub params: Vec<FunctionCallParameter>,
}

#[derive(PartialEq, Default, Debug, Clone)]
pub struct RefferenceType {
    pub refference: Box<types::Types>,
    pub on_dot: bool,
    pub chain: Vec<String>,
}

pub enum ArithmeticOperatorItem {
    
}

#[derive(PartialEq, Default, Debug, Clone)]
pub struct ArithmeticOperator {

}


#[derive(PartialEq, Debug, Clone)]
pub enum Types {
    Number(NumberType),
    Double(DoubleType),
    Bool(BoolType),
    String(StringType),
    Collective(CollectiveType),
    Refference(RefferenceType),
    Array(ArrayType),
    Function,
    FunctionCall(FunctionCall),
    Void,
    Null,
}

impl Types {
    
    pub fn is_string_open(&self) -> bool {
        match &*self {
            Types::Number(_) => true,
            Types::Double(_) => true,
            Types::Bool(_) => true,
            Types::String(data) => !data.complete,
            Types::Collective(_) => false,
            Types::Refference(data) => false,
            Types::Array(data) => {
                if !data.complete {
                    if data.collective.len() == 0 {
                        false
                    } else {
                        !data.collective[data.collective.len() - 1].value_complete
                    }
                } else {
                    false
                }
            },
            Types::Function => false,
            Types::FunctionCall(data) => false,
            Types::Void => true,
            Types::Null => true,
        }
    }

    pub fn is_complete(&self) -> bool {
        match &*self {
            Types::Number(_) => true, //Always complete
            Types::Double(_) => true, //Always complete
            Types::Bool(_) => true, //Always complete
            Types::String(data) => data.complete,
            Types::Collective(_) => false,
            Types::Refference(data) => !data.on_dot,
            Types::Array(data) => data.complete,
            Types::Function => false,
            Types::FunctionCall(data) => data.complete,
            Types::Void => true,
            Types::Null => true,
        }
    }

    pub fn is_array(&self) -> bool {
        match *self {
            Types::Number(_) => false,
            Types::Double(_) => false,
            Types::Bool(_) => false,
            Types::String(_) => false,
            Types::Collective(_) => false,
            Types::Refference(_) => false,
            Types::Array(_) => true,
            Types::Function => false,
            Types::FunctionCall(_) => false,
            Types::Void => false,
            Types::Null => false,
        }
    
    }
    
    pub fn is_string(&self) -> bool {
        match *self {
            Types::Number(_) => false,
            Types::Double(_) => false,
            Types::Bool(_) => false,
            Types::String(_) => true,
            Types::Collective(_) => false,
            Types::Refference(_) => false,
            Types::Array(_) => false,
            Types::Function => false,
            Types::FunctionCall(_) => false,
            Types::Void => false,
            Types::Null => false,
        }
    }

    pub fn is_string_non_complete(&self) -> bool {
        match &*self {
            Types::Number(_) => false,
            Types::Double(_) => false,
            Types::Bool(_) => false,
            Types::String(data) => !data.complete,
            Types::Collective(_) => false,
            Types::Refference(_) => false,
            Types::Array(_) => false,
            Types::Function => false,
            Types::FunctionCall(_) => false,
            Types::Void => false,
            Types::Null => false,
        }
    }

    pub fn is_array_non_complete(&self) -> Option<bool> {
        match &*self {
            Types::Number(_) => None,
            Types::Double(_) => None,
            Types::Bool(_) => None,
            Types::String(_) => None,
            Types::Collective(_) => None,
            Types::Refference(_) => None,
            Types::Array(a) => Some(!a.complete),
            Types::Function => None,
            Types::FunctionCall(_) => None,
            Types::Void => None,
            Types::Null => None,
        }
    }

    pub fn is_array_complete(&self) -> bool {
        match &*self {
            Types::Number(_) => false,
            Types::Double(_) => false,
            Types::Bool(_) => false,
            Types::String(_) => false,
            Types::Collective(_) => false,
            Types::Refference(_) => false,
            Types::Array(a) => a.complete,
            Types::Function => false,
            Types::FunctionCall(_) => false,
            Types::Void => false,
            Types::Null => false,
        }
    }

    pub fn make_complete(&mut self) {
        match self {
            Types::Number(e) => e.complete = true,
            Types::Double(e) => e.complete = true,
            Types::Bool(_) => (),
            Types::String(e) => e.complete = true,
            Types::Collective(_) => (),
            Types::Refference(_) => (),
            Types::Array(e) => e.complete = true,
            Types::Function => (),
            Types::FunctionCall(_) => (),
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