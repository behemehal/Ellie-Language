<<<<<<< HEAD
<<<<<<< HEAD
=======
<<<<<<< HEAD
=======

>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
=======
>>>>>>> FFI
#[derive(PartialEq, Debug, Clone)]
pub enum CallerType {
    FunctionCaller,
    VariableCaller,
    Unknown,
}

<<<<<<< HEAD
<<<<<<< HEAD
=======
<<<<<<< HEAD
=======

>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
=======
>>>>>>> FFI
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
