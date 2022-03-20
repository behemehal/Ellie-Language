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
        message: "Targeted variable '$token' not found in scope".to_owned(),
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
        message: "Expected operator found value instead, '$token'".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S13: error::Error = error::Error {
        code: 0x12,
        title: "SyntaxError".to_owned(),
        message: "Expected operator found '$token'".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S14: error::Error = error::Error {
        code: 0x13,
        title: "TypeError".to_owned(),
        message: "Cannot leave char empty".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S15: error::Error = error::Error {
        code: 0x14,
        title: "TypeError".to_owned(),
        message: "Char type can take one character only".to_owned(),
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
        message: "Types cannot have number properties".to_owned(),
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
        message: "Cannot apply data to generic type".to_owned(),
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
        title: "ImportError".to_owned(),
        message: "Cannot compile '$token'".to_owned(),
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
        title: "ParserIntegrityError".to_owned(),
        message: "$token".to_owned(),
        ..Default::default()
    };
    pub static ref ERROR_S42: error::Error = error::Error {
        code: 0x41,
        title: "SyntaxError".to_owned(),
        message: "'$token' is not found in '$token1' attributes".to_owned(),
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
}
