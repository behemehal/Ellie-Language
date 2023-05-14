use crate::alloc::borrow::ToOwned;
use crate::error;

lazy_static! {
    pub static ref ERROR_S1: error::Error = error::Error {
        code: 0x00,
        title: "SyntaxError".to_owned(),
        message: "Unexpected Token '$token'".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S2: error::Error = error::Error {
        code: 0x01,
        title: "SyntaxError".to_owned(),
        message: "Expected return type".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S3: error::Error = error::Error {
        code: 0x02,
        title: "ReferenceError".to_owned(),
        message: "Expected '$token1' found '$token2'".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S4: error::Error = error::Error {
        code: 0x03,
        title: "ReferenceError".to_owned(),
        message: "Setter '$token' is not found in the scope".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S5: error::Error = error::Error {
        code: 0x04,
        title: "SyntaxError".to_owned(),
        message: "Unknown escape character '$token'".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S6: error::Error = error::Error {
        code: 0x05,
        title: "ReferenceError".to_owned(),
        message: "'$token' is not defined".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S7: error::Error = error::Error {
        code: 0x06,
        title: "ReferenceError".to_owned(),
        message: "$name requires '$token' parameters, found '$token2' length of parameters"
            .to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S8: error::Error = error::Error {
        code: 0x07,
        title: "ReferenceError".to_owned(),
        message: "Expected type annotations".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S9: error::Error = error::Error {
        code: 0x08,
        title: "ReferenceError".to_owned(),
        message: "Static type expected".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S10: error::Error = error::Error {
        code: 0x09,
        title: "TypeError".to_owned(),
        message: "Duplicate parameter".to_owned(),
        semi_assist: true,
        ..Default::default()
    };
    pub static ref ERROR_S11: error::Error = error::Error {
        code: 0x10,
        title: "TypeError".to_owned(),
        message: "'$token' is not suitable for to be constructed".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S12: error::Error = error::Error {
        code: 0x11,
        title: "SyntaxError".to_owned(),
        message: "Setters must have 1 parameter".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S13: error::Error = error::Error {
        code: 0x12,
        title: "SyntaxError".to_owned(),
        message: "'$token' and '$token1' has incompatible type returns".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S14: error::Error = error::Error {
        code: 0x13,
        title: "TypeError".to_owned(),
        message: "Setters cannot return data".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S15: error::Error = error::Error {
        code: 0x14,
        title: "TypeError".to_owned(),
        message: "Cannot invoke class without new".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S16: error::Error = error::Error {
        code: 0x15,
        title: "OverflowError".to_owned(),
        message: "The value '$val' cannot fit to integer".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S17: error::Error = error::Error {
        code: 0x16,
        title: "OverflowError".to_owned(),
        message: "The value '$val' has infinite size".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S18: error::Error = error::Error {
        code: 0x17,
        title: "TypeError".to_owned(),
        message: "Cannot assign a value to constant variable".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S19: error::Error = error::Error {
        code: 0x18,
        title: "OverflowError".to_owned(),
        message: "Fixed size exceeded: expected '$token' elements, got '$token2' elements"
            .to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S20: error::Error = error::Error {
        code: 0x19,
        title: "TypeError".to_owned(),
        message: "Negative number is not allowed on '$token'".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S21: error::Error = error::Error {
        code: 0x20,
        title: "SyntaxError".to_owned(),
        message: "Reserved keyword '$token'".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S22: error::Error = error::Error {
        code: 0x21,
        title: "SyntaxError".to_owned(),
        message: "Unexpected item".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S23: error::Error = error::Error {
        code: 0x22,
        title: "ReferenceError".to_owned(),
        message: "Getter '$token' is not found in the scope".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S24: error::Error = error::Error {
        code: 0x23,
        title: "ReferenceError".to_owned(),
        message: "'$token' is already defined".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S25: error::Error = error::Error {
        code: 0x24,
        title: "TypeError".to_owned(),
        message: "'$token' is not a function".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S26: error::Error = error::Error {
        code: 0x25,
        title: "SyntaxError".to_owned(),
        message: "Unexpected ending".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S27: error::Error = error::Error {
        code: 0x26,
        title: "TypeError".to_owned(),
        message: "Un-Assignable type".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S28: error::Error = error::Error {
        code: 0x27,
        title: "ImportError".to_owned(),
        message: "Cannot resolve '$token' module".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S29: error::Error = error::Error {
        code: 0x28,
        title: "TypeError".to_owned(),
        message: "Supplied data '$token' is not iterable".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S30: error::Error = error::Error {
        code: 0x29,
        title: "TypeError".to_owned(),
        message: "Class can only have one constructor".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S31: error::Error = error::Error {
        code: 0x30,
        title: "TypeError".to_owned(),
        message: "'$token' is not a constructable".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S32: error::Error = error::Error {
        code: 0x31,
        title: "ImportError".to_owned(),
        message: "$token".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S33: error::Error = error::Error {
        code: 0x32,
        title: "RuntimeError".to_owned(),
        message: "Main function required".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S34: error::Error = error::Error {
        code: 0x33,
        title: "TypeError".to_owned(),
        message: "'$token' is not found in properties".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S35: error::Error = error::Error {
        code: 0x34,
        title: "SyntaxError".to_owned(),
        message: "Cannot add a new parameter after multiple parameter".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S36: error::Error = error::Error {
        code: 0x35,
        title: "SyntaxError".to_owned(),
        message: "'$token' cannot be used as collective parameter".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S37: error::Error = error::Error {
        code: 0x36,
        title: "SyntaxError".to_owned(),
        message: "'$token' cannot be used as reference pointer".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S38: error::Error = error::Error {
        code: 0x37,
        title: "ReferenceError".to_owned(),
        message: "'$token' required in scope".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S39: error::Error = error::Error {
        code: 0x38,
        title: "ReferenceError".to_owned(),
        message: "Cannot define built-in types".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S40: error::Error = error::Error {
        code: 0x39,
        title: "ParserError".to_owned(),
        message: "Parser messages value can only be string, but found '$token'".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S41: error::Error = error::Error {
        code: 0x40,
        title: "UnimplementedFeature".to_owned(),
        message: "$token".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S42: error::Error = error::Error {
        code: 0x41,
        title: "SyntaxError".to_owned(),
        message: "Getter '$token' is not found in '$token1' properties".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S43: error::Error = error::Error {
        code: 0x42,
        title: "TypeError".to_owned(),
        message: "Invalid left-hand side in assignment".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S44: error::Error = error::Error {
        code: 0x43,
        title: "ReferenceError".to_owned(),
        message: "Targeted class requires '$token' generic parameters, found '$token2' generic parameters".to_owned(),
        reference_message: "Class definied here".to_owned(),
        ..Default::default()
    };

    pub static ref ERROR_S45: error::Error = error::Error {
        code: 0x44,
        title: "ReferenceError".to_owned(),
        message: "'$token' is not a type".to_owned(),
        ..Default::default()
    };

    pub static ref ERROR_S46: error::Error = error::Error {
        code: 0x45,
        title: "ReferenceError".to_owned(),
        message: "Referenced type does not implement 'as' convertor".to_owned(),
        ..Default::default()
    };

    pub static ref ERROR_S47: error::Error = error::Error {
        code: 0x46,
        title: "RuntimeRestriction".to_owned(),
        message: "$token".to_owned(),
        ..Default::default()
    };

    pub static ref ERROR_S48: error::Error = error::Error {
        code: 0x47,
        title: "ReferenceError".to_owned(),
        message: "'$token' does not implement index queries".to_owned(),
        ..Default::default()
    };

    pub static ref ERROR_S49: error::Error = error::Error {
        code: 0x48,
        title: "ReferenceError".to_owned(),
        message: "Cannot query index to '$target' with '$token'".to_owned(),
        ..Default::default()
    };

    pub static ref ERROR_S50: error::Error = error::Error {
        code: 0x49,
        title: "TypeError".to_owned(),
        message: "Unimplemented type conversion '$target' to '$type'".to_owned(),
        ..Default::default()
    };

    pub static ref ERROR_S51: error::Error = error::Error {
        code: 0x50,
        title: "TypeError".to_owned(),
        message: "Cannot resolve non nullAble foreign type".to_owned(),
        full_assist: true,
        ..Default::default()
    };

    pub static ref ERROR_S52: error::Error = error::Error {
        code: 0x51,
        title: "TypeError".to_owned(),
        message: "'$opType' not implemented in '$target' for '$value'".to_owned(),
        ..Default::default()
    };

    pub static ref ERROR_S53: error::Error = error::Error {
        code: 0x52,
        title: "SyntaxError".to_owned(),
        message: "$opType is not chainable".to_owned(),
        ..Default::default()
    };

    pub static ref ERROR_S54: error::Error = error::Error {
        code: 0x53,
        title: "SyntaxError".to_owned(),
        message: "Char can only take one character".to_owned(),
        semi_assist: true,
        ..Default::default()
    };

    pub static ref ERROR_S55: error::Error = error::Error {
        code: 0x54,
        title: "TypeError".to_owned(),
        message: "Consider giving this a type".to_owned(),
        ..Default::default()
    };

    pub static ref ERROR_S56: error::Error = error::Error {
        code: 0x55,
        title: "TypeError".to_owned(),
        message: "Cannot assign file key to this element".to_owned(),
        semi_assist: true,
        ..Default::default()
    };

    pub static ref ERROR_S57: error::Error = error::Error {
        code: 0x56,
        title: "TypeError".to_owned(),
        message: "Unassigned file keys".to_owned(),
        semi_assist: true,
        ..Default::default()
    };

    pub static ref ERROR_S58: error::Error = error::Error {
        code: 0x57,
        title: "ExperimentalFeature".to_owned(),
        message: "Usage of '$token' is experimental, use '--experiental-feature' flag to use it".to_owned(),
        semi_assist: true,
        ..Default::default()
    };

    pub static ref ERROR_S59: error::Error = error::Error {
        code: 0x58,
        title: "UnfinishedFeature".to_owned(),
        message: "Usage of '$token' is forbidden for now".to_owned(),
        semi_assist: true,
        ..Default::default()
    };

    pub static ref ERROR_S60: error::Error = error::Error {
        code: 0x59,
        title: "ReferenceError".to_owned(),
        message: "Cannot use non-static variables here, use const instead".to_owned(),
        semi_assist: true,
        ..Default::default()
    };

    pub static ref ERROR_S61: error::Error = error::Error {
        code: 0x60,
        title: "ReferenceError".to_owned(),
        message: "Cannot access variable from this scope".to_owned(),
        semi_assist: true,
        ..Default::default()
    };

    pub static ref ERROR_S62: error::Error = error::Error {
        code: 0x61,
        title: "ReferenceError".to_owned(),
        message: "Cannot use variables with value here, use const or constructor to build them".to_owned(),
        semi_assist: true,
        ..Default::default()
    };

    pub static ref ERROR_S63: error::Error = error::Error {
        code: 0x62,
        title: "ReferenceError".to_owned(),
        message: "'$token' is not suitable for to be constructor parameter".to_owned(),
        semi_assist: true,
        ..Default::default()
    };
}
