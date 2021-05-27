<<<<<<< HEAD
=======

>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
#[derive(PartialEq, Debug, Clone)]
pub enum CallerType {
    FunctionCaller,
    VariableCaller,
    Unknown,
}

<<<<<<< HEAD
=======

>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
#[derive(PartialEq, Debug, Clone)]
pub struct Caller {
    pub initialized: bool,
    pub name: String,
    pub named: bool,
    pub value_complete: bool,
    pub rtype: CallerType,
    pub raw_value: String,
    pub value: crate::syntax::types::Types,
    pub pos: crate::parser::defs::Cursor,
}
