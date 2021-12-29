use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum CliOuputType {
    Warning,
    Error,
    Info
}

#[derive(Serialize, Deserialize, Clone)]

pub struct CliOuputExtraData<K, V> {
    pub key: K,
    pub value: V,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CliOuput<K, V> {
    pub code: u8,
    pub rtype: CliOuputType,
    pub message: String,
    pub extra: Vec<CliOuputExtraData<K, V>>
}

lazy_static! {
    pub static ref COMPILE_SUCCESS_WITH_NO_WARNINGS : CliOuput<String, String> = CliOuput {
        code: 0,
        rtype: CliOuputType::Info,
        message: "Compiling succeeded with no warnings".to_string(),
        extra: Vec::new(),
    };

    pub static ref COMPILE_SUCCESS_WITH_WARNINGS : CliOuput<String, String> = CliOuput {
        code: 1,
        rtype: CliOuputType::Info,
        message: "Compiling succeeded with warnings".to_string(),
        extra: Vec::new(),
    };

    pub static ref COMPILE_FAILED_WITH_ERRORS_WITH_NO_WARNINGS : CliOuput<String, String> = CliOuput {
        code: 2,
        rtype: CliOuputType::Info,
        message: "Compiling failed with errors".to_string(),
        extra: Vec::new(),
    };

    pub static ref COMPILE_FAILED_WITH_ERRORS_WITH_WARNINGS : CliOuput<String, String> = CliOuput {
        code: 3,
        rtype: CliOuputType::Info,
        message: "Compiling failed with errors and warnings".to_string(),
        extra: Vec::new(),
    };

    pub static ref COMPILER_ERROR : CliOuput<String, ellie_core::error::Error> = CliOuput {
        code: 4,
        rtype: CliOuputType::Error,
        message: String::new(),
        extra: Vec::new(),
    };

    pub static ref COMPILER_WARNING : CliOuput<String, ellie_core::warning::Warning> = CliOuput {
        code: 5,
        rtype: CliOuputType::Warning,
        message: String::new(),
        extra: Vec::new(),
    };

    pub static ref WRITE_FILE_ERROR : CliOuput<String, String> = CliOuput {
        code: 3,
        rtype: CliOuputType::Info,
        message: "Failed to write output file".to_string(),
        extra: Vec::new(),
    };

    pub static ref PATH_ERROR : CliOuput<String, String> = CliOuput {
        code: 3,
        rtype: CliOuputType::Info,
        message: "Failed to resolve path".to_string(),
        extra: Vec::new(),
    };

}