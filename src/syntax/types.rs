#[derive(PartialEq, Eq, Default, Debug, Clone, Copy)]
pub struct NumberType {
    pub value: usize,
    pub complete: bool
}

#[derive(PartialEq, Eq, Default, Debug, Clone)]
pub struct StringType {
    pub value: String,
    pub quote_type: String,
    pub complete: bool
}

#[derive(PartialEq, Default, Debug, Clone, Copy)]
pub struct DoubleType {
    pub value: f32
}

#[derive(PartialEq, Eq, Default, Debug, Clone, Copy)]
pub struct BoolType {
    pub value: bool
}

#[derive(PartialEq, Default, Debug, Clone)]
pub struct CollectiveEntry {
    pub key: String,
    pub value: Box<crate::syntax::types::Types>,
}

#[derive(PartialEq, Default, Debug, Clone)]
pub struct CollectiveType {
    pub layer_size : usize,
    pub collective: Vec<CollectiveEntry> 
}

#[derive(PartialEq, Default, Debug, Clone)]
pub struct FunctionRefference {
    pub name: String,
    pub params: Vec<Types>
}


#[derive(PartialEq,  Default, Debug, Clone)]
pub struct RefferenceType {
    pub refference: Box<crate::syntax::types::Types>,
    pub on_dot: bool,
    pub chain: Vec<String>
}


//impl Copy for RefferenceType {
//    fn Copy() -> Self { 
//        RefferenceType {
//            refference: Box::new(),
//            chain: Vec::new()
//        }
//    }
//}

#[derive(PartialEq, Debug, Clone)]
pub enum Types {
    Number(NumberType),
    Double(DoubleType),
    Bool(BoolType),
    String(StringType),
    Collective(CollectiveType),
    Refference(RefferenceType),
    Dynamic,
    Array,
    Function,
    Null
}

impl Default for Types {
    fn default() -> Self { Types::Null }
}